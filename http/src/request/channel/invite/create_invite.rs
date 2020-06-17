use crate::json_to_vec;
use crate::request::prelude::*;
use twilight_model::{
    id::{ChannelId, UserId},
    invite::{Invite, TargetUserType},
};

#[derive(Default, Serialize)]
struct CreateInviteFields {
    max_age: Option<u64>,
    max_uses: Option<u64>,
    temporary: Option<bool>,
    unique: Option<bool>,
    target_user: Option<String>,
    target_user_type: Option<TargetUserType>,
}

/// Create an invite, with options.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::ChannelId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let channel_id = ChannelId(123);
/// let invite = client
///     .create_invite(channel_id)
///     .max_uses(3)
///     .await?;
/// # Ok(()) }
/// ```
pub struct CreateInvite<'a> {
    channel_id: ChannelId,
    fields: CreateInviteFields,
    fut: Option<Pending<'a, Invite>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateInvite<'a> {
    pub fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: CreateInviteFields::default(),
            fut: None,
            http,
            reason: None,
        }
    }

    /// Set the maximum age for an invite.
    ///
    /// If no age is specified, Discord sets the age to 86400 seconds, or 24 hours.
    /// Set to 0 to never expire.
    pub fn max_age(mut self, max_age: u64) -> Self {
        self.fields.max_age.replace(max_age);

        self
    }

    /// Set the maximum uses for an invite.
    ///
    /// Discord defaults this to infinite.
    pub fn max_uses(mut self, max_uses: u64) -> Self {
        self.fields.max_uses.replace(max_uses);

        self
    }

    /// Set whether the invite will grant temporary membership.
    pub fn temporary(mut self, temporary: bool) -> Self {
        self.fields.temporary.replace(temporary);

        self
    }

    /// If true, don't try to reuse a similar invite (useful for creating many unique one time use
    /// invites) ([Discord Docs])
    ///
    /// [Discord Docs]: https://discord.com/developers/docs/resources/channel#create-channel-invite
    pub fn unique(mut self, unique: bool) -> Self {
        self.fields.unique.replace(unique);

        self
    }

    /// Set the target user for this invite.
    pub fn target_user(mut self, target_user: UserId) -> Self {
        self.fields.target_user.replace(target_user.0.to_string());

        self
    }

    /// Set the target user type for this invite.
    pub fn target_user_type(mut self, target_user_type: TargetUserType) -> Self {
        self.fields.target_user_type.replace(target_user_type);

        self
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                json_to_vec(&self.fields)?,
                headers,
                Route::CreateInvite {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                json_to_vec(&self.fields)?,
                Route::CreateInvite {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateInvite<'_>, Invite);
