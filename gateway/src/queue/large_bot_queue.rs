use super::{DayLimiter, Queue};
use futures_channel::{
    mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
    oneshot::{self, Sender},
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use std::{fmt::Debug, future::Future, pin::Pin, time::Duration};
use tokio::time::delay_for;

/// Queue built for single-process clusters that require identifying via
/// [Sharding for Very Large Bots].
///
/// Usage with other processes will cause inconsistencies between each process
/// cluster's ratelimit buckets. If you use multiple processes for clusters,
/// then refer to the [module-level] documentation.
///
/// [Sharding for Very Large Bots]: https://discord.com/developers/docs/topics/gateway#sharding-for-very-large-bots
/// [module-level]: ./index.html
#[derive(Debug)]
pub struct LargeBotQueue {
    buckets: Vec<UnboundedSender<Sender<()>>>,
    limiter: DayLimiter,
}

impl LargeBotQueue {
    /// Create a new large bot queue.
    ///
    /// You must provide the number of buckets Discord requires your bot to
    /// connect with.
    pub async fn new(buckets: usize, http: &twilight_http::Client) -> Self {
        let mut queues = Vec::with_capacity(buckets);
        for _ in 0..buckets {
            let (tx, rx) = unbounded();

            tokio::spawn(waiter(rx));

            queues.push(tx)
        }

        let limiter = DayLimiter::new(http).await.expect(
            "Getting the first session limits failed, \
             Is network connection available?",
        );

        // The level_enabled macro does not turn off with the dynamic
        // tracing levels. It is made for the static_max_level_xxx features
        // And will return false if you do not use those features of if
        // You use the feature but then dynamically set a lower feature.
        if tracing::level_enabled!(tracing::Level::INFO) {
            let lock = limiter.0.lock().await;
            tracing::info!(
                "{}/{} identifies used before next reset in {:.2?}",
                lock.current,
                lock.total,
                lock.next_reset
            );
        }

        Self {
            buckets: queues,
            limiter,
        }
    }
}

async fn waiter(mut rx: UnboundedReceiver<Sender<()>>) {
    const DUR: Duration = Duration::from_secs(6);
    while let Some(req) = rx.next().await {
        if let Err(err) = req.send(()) {
            tracing::warn!("skipping, send failed with: {:?}", err);
        }
        delay_for(DUR).await;
    }
}

impl Queue for LargeBotQueue {
    /// Request to be able to identify with the gateway. This will place this
    /// request behind all other requests, and the returned future will resolve
    /// once the request has been completed.
    fn request(&'_ self, shard_id: [u64; 2]) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        #[allow(clippy::cast_possible_truncation)]
        let bucket = (shard_id[0] % (self.buckets.len() as u64)) as usize;
        let (tx, rx) = oneshot::channel();

        Box::pin(async move {
            self.limiter.get().await;
            if let Err(err) = self.buckets[bucket].clone().send(tx).await {
                tracing::warn!("skipping, send failed with: {:?}", err);
                return;
            }

            tracing::info!("waiting for allowance on shard {}", shard_id[0]);

            let _ = rx.await;
        })
    }
}
