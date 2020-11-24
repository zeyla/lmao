use super::{
    config::Config as ClusterConfig,
    r#impl::{Cluster, ClusterStartError},
};
use crate::shard::{LargeThresholdError, ResumeSession, ShardBuilder};
use std::{
    collections::HashMap,
    convert::TryFrom,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    ops::{Bound, RangeBounds},
    sync::Arc,
};
use twilight_gateway_queue::{LocalQueue, Queue};
use twilight_http::Client;
use twilight_model::gateway::{payload::update_status::UpdateStatusInfo, Intents};

/// Starting a cluster failed.
#[derive(Debug)]
pub enum ShardSchemeRangeError {
    /// Start of the shard range was greater than the end or total.
    IdTooLarge {
        /// Last shard in the range to manage.
        end: u64,
        /// First shard in the range to manage.
        start: u64,
        /// Total number of shards used by the bot.
        total: u64,
    },
}

impl Display for ShardSchemeRangeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::IdTooLarge { end, start, total } => f.write_fmt(format_args!(
                "The shard ID range {}-{}/{} is larger than the total",
                start, end, total
            )),
        }
    }
}

impl Error for ShardSchemeRangeError {}

/// The method of sharding to use.
///
/// By default this is [`Auto`].
///
/// [`Auto`]: #variant.Auto
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ShardScheme {
    /// Specifies to retrieve the amount of shards recommended by Discord and
    /// then start all of them.
    ///
    /// For example, if Discord recommends 10 shards, then all 10 shards will be
    /// started.
    Auto,
    /// Star a single bucket for very large bot sharding
    Bucket {
        /// The shard id of the first shard to start, this should be less than the concurrency.
        /// If you for example have a concurrency of 16 and the bucket id is 0, it will start:
        /// shard 0, 16, 32, 48 and so on.
        bucket_id: u64,
        /// The amount of concurrency allowed by discord, this is given by the [`max_concurrency`]
        /// field on [`SessionStartLimit`].
        ///
        /// [`max_concurrency`]: ::twilight_model::gateway::SessionStartLimit.max_concurrency
        /// [`SessionStartLimit`]: ::twilight_model::gateway::SessionStartLimit
        concurrency: u64,
        /// The total amount of shards to start, not only in this bucket but the complete total.
        total: u64,
    },
    /// Specifies to start a range of shards.
    ///
    /// # Examples
    ///
    /// For example, if your bot uses 50 shards, then you might specify to start
    /// shards 0 through 24:
    ///
    /// ```
    /// use twilight_gateway::cluster::ShardScheme;
    /// use std::convert::TryFrom;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let range = ShardScheme::try_from((0..24, 50))?;
    /// # Ok(()) }
    /// ```
    Range {
        /// First shard ID to spawn.
        from: u64,
        /// Last shard ID to spawn.
        ///
        /// This doesn't necessarily have to be up to the `total`.
        to: u64,
        /// Total number of shards used by the bot.
        total: u64,
    },
}

impl Default for ShardScheme {
    fn default() -> Self {
        Self::Auto
    }
}

impl<T: RangeBounds<u64>> TryFrom<(T, u64)> for ShardScheme {
    type Error = ShardSchemeRangeError;

    fn try_from((range, total): (T, u64)) -> Result<Self, Self::Error> {
        let start = match range.start_bound() {
            Bound::Excluded(num) => *num - 1,
            Bound::Included(num) => *num,
            Bound::Unbounded => 0,
        };
        let end = match range.end_bound() {
            Bound::Excluded(num) => *num - 1,
            Bound::Included(num) => *num,
            Bound::Unbounded => total - 1,
        };

        if start > end {
            return Err(ShardSchemeRangeError::IdTooLarge { end, start, total });
        }

        Ok(Self::Range {
            from: start,
            to: end,
            total,
        })
    }
}

/// Builder to configure and construct a [`Cluster`].
///
/// # Examples
///
/// Create a cluster with only the `GUILD_MESSAGES` intents with a
/// [`large_threshold`] of 100.
///
/// ```rust,no_run
/// use std::env;
/// use twilight_gateway::{Cluster, Intents};
///
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let token = env::var("DISCORD_TOKEN")?;
///
/// let cluster = Cluster::builder(token, Intents::GUILD_MESSAGES)
///     .large_threshold(100)?
///     .build()
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`Cluster`]: ./struct.Cluster.html
/// [`large_threshold`]: #method.large_threshold
#[derive(Debug)]
pub struct ClusterBuilder(ClusterConfig, ShardBuilder);

impl ClusterBuilder {
    /// Create a new builder to construct and configure a cluster.
    pub fn new(token: impl Into<String>, intents: Intents) -> Self {
        Self::_new(token.into(), intents)
    }

    fn _new(mut token: String, intents: Intents) -> Self {
        if !token.starts_with("Bot ") {
            token.insert_str(0, "Bot ");
        }

        let http_client = Client::new(token.clone());

        let shard_config =
            ShardBuilder::new(token.clone(), intents).http_client(http_client.clone());

        Self(
            ClusterConfig {
                http_client,
                shard_config: shard_config.0,
                shard_scheme: ShardScheme::Auto,
                queue: Arc::new(Box::new(LocalQueue::new())),
                resume_sessions: HashMap::new(),
            },
            ShardBuilder::new(token, intents),
        )
    }

    /// Consume the builder and create the cluster.
    ///
    /// # Errors
    ///
    /// Returns [`ClusterStartError::RetrievingGatewayInfo`] if there was an
    /// HTTP error Retrieving the gateway information.
    ///
    /// [`ClusterStartError::RetrievingGatewayInfo`]: enum.ClusterStartError.html#variant.RetrievingGatewayInfo
    pub async fn build(mut self) -> Result<Cluster, ClusterStartError> {
        if self.0.shard_config.gateway_url.is_none() {
            let gateway_url = (self.1)
                .0
                .http_client
                .gateway()
                .authed()
                .await
                .ok()
                .map(|s| s.url);

            self = self.gateway_url(gateway_url);
        }

        self.0.shard_config = (self.1).0;

        Cluster::new_with_config(self.0).await
    }

    /// Set the URL that will be used to connect to the gateway.
    pub fn gateway_url(mut self, gateway_url: Option<String>) -> Self {
        self.1 = self.1.gateway_url(gateway_url);

        self
    }

    /// Set the `twilight_http` Client used by the cluster and the shards it
    /// manages.
    ///
    /// This is needed so that the cluster and shards can retrieve gateway
    /// information.
    ///
    /// Defaults to a new, default HTTP client is used.
    pub fn http_client(mut self, http_client: Client) -> Self {
        self.1 = self.1.http_client(http_client);

        self
    }

    /// Set the "large threshold" of shards.
    ///
    /// Refer to the shard's [`ShardBuilder::large_threshold`] for more
    /// information.
    ///
    /// # Errors
    ///
    /// Returns [`LargeThresholdError::TooFew`] if the provided value is below
    /// 50.
    ///
    /// Returns [`LargeThresholdError::TooMany`] if the provided value is above
    /// 250.
    ///
    /// [`LargeThresholdError::TooFew`]: ../shard/enum.LargeThresholdError.html#variant.TooFew
    /// [`LargeThresholdError::TooMany`]: ../shard/enum.LargeThresholdError.html#variant.TooMany
    /// [`ShardBuilder::large_treshold`]: ../shard/ShardBuilder.html#method.large_threshold
    pub fn large_threshold(mut self, large_threshold: u64) -> Result<Self, LargeThresholdError> {
        self.1 = self.1.large_threshold(large_threshold)?;

        Ok(self)
    }

    /// Set the presence to use when identifying with the gateway.
    ///
    /// Refer to the shard's [`ShardBuilder::presence`] for more information.
    ///
    /// [`ShardBuilder::presence`]: ../shard/struct.ShardBuilder.html#method.presence
    pub fn presence(mut self, presence: UpdateStatusInfo) -> Self {
        self.1 = self.1.presence(presence);

        self
    }

    /// Set the scheme to use for shard managing.
    ///
    /// For example, [`ShardScheme::Auto`] means that the cluster will
    /// automatically manage all of the shards that Discord recommends you use.
    /// [`ShardScheme::Range`] means that it will manage a range of shards, but
    /// not necessarily all of the shards that your bot uses.
    ///
    /// The default value is [`ShardScheme::Auto`]. For most setups this is an
    /// acceptable default.
    ///
    /// # Examples
    ///
    /// Configure a cluster to manage shards 0-9 out of 20 shards total:
    ///
    /// ```no_run
    /// use twilight_gateway::{cluster::{Cluster, ShardScheme}, Intents};
    /// use std::{
    ///     convert::TryFrom,
    ///     env,
    /// };
    ///
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let token = env::var("DISCORD_TOKEN")?;
    /// let scheme = ShardScheme::try_from((0..=9, 20))?;
    ///
    /// let cluster = Cluster::builder(token, Intents::GUILD_MESSAGES)
    ///     .shard_scheme(scheme)
    ///     .build()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`ShardScheme::Auto`]: enum.ShardScheme.html#variant.Auto
    /// [`ShardScheme::Range`]: enum.ShardScheme.html#variant.Range
    pub fn shard_scheme(mut self, scheme: ShardScheme) -> Self {
        self.0.shard_scheme = scheme;

        self
    }

    /// Set the queue to use for queueing shard connections.
    ///
    /// This is useful when you have a very large bot or when you have a more
    /// advanced setup with multiple processes connecting at the same time.
    ///
    /// Refer to the [`queue`] module for more information.
    ///
    /// [`queue`]: ../queue/index.html
    pub fn queue(mut self, queue: Arc<Box<dyn Queue>>) -> Self {
        self.0.queue = Arc::clone(&queue);
        self.1 = self.1.queue(queue);

        self
    }

    /// Set the session information to resume shards with.
    ///
    /// This requires having recovered the resume data when shutting down the
    /// cluster via [`Cluster::down_resumable`].
    ///
    /// Note that this does not guarantee all or any of the shards will be able
    /// to resume. If their sessions are invalid they will have to re-identify
    /// to initialize a new session.
    ///
    /// [`Cluster::down_resumable`]: struct.Cluster.html#method.down_resumable
    pub fn resume_sessions(mut self, resume_sessions: HashMap<u64, ResumeSession>) -> Self {
        self.0.resume_sessions = resume_sessions;
        self
    }
}

impl<T: Into<String>> From<(T, Intents)> for ClusterBuilder {
    fn from((token, intents): (T, Intents)) -> Self {
        Self::new(token, intents)
    }
}

#[cfg(test)]
mod tests {
    use super::{ClusterBuilder, ShardScheme, ShardSchemeRangeError};
    use crate::Intents;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{
        convert::TryFrom,
        error::Error,
        fmt::{Debug, Display},
        hash::Hash,
    };

    assert_fields!(ShardSchemeRangeError::IdTooLarge: end, start, total);
    assert_fields!(ShardScheme::Range: from, to, total);
    assert_impl_all!(ClusterBuilder: Debug, From<(String, Intents)>, Send, Sync);
    assert_impl_all!(ShardSchemeRangeError: Debug, Display, Error, Send, Sync);
    assert_impl_all!(
        ShardScheme: Clone,
        Debug,
        Default,
        Eq,
        Hash,
        PartialEq,
        Send,
        Sync
    );

    #[test]
    fn test_shard_scheme() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            ShardScheme::Range {
                from: 0,
                to: 9,
                total: 10,
            },
            ShardScheme::try_from((0..=9, 10))?
        );

        Ok(())
    }
}
