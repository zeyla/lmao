//! Types for working with and running connections to the gateway.
//!
//! At the heart of the `shard` module is the [`Shard`] itself: it's the
//! interface used to start a shard, send messages to the gateway, and receive
//! [`Event`]s from it, such as [new messages] or [channel deletions].
//!
//! Once running, the shard maintains [information about itself] that you can
//! obtain through it. This is information such as the latency or the current
//! [`Stage`] of the connection, like whether it's [`Disconnected`] or
//! [`Resuming`] the connection.
//!
//! Shards are configurable through the [`ShardBuilder`], which provides a clean
//! interface for correctly configuring a shard.
//!
//! [`Event`]: ::twilight_model::gateway::event::Event
//! [`Disconnected`]: Stage::Disconnected
//! [`Resuming`]: Stage::Resuming
//! [channel deletions]: ::twilight_model::gateway::event::Event::ChannelDelete
//! [information about itself]: Shard::info
//! [new messages]: ::twilight_model::gateway::event::Event::MessageCreate

pub mod stage;

mod builder;
mod config;
mod event;
mod r#impl;
mod json;
mod processor;
mod sink;

pub use self::{
    builder::{LargeThresholdError, ShardBuilder, ShardIdError},
    config::Config,
    event::Events,
    processor::heartbeat::Latency,
    r#impl::{
        CommandError, Information, ResumeSession, SessionInactiveError, Shard, ShardStartError,
    },
    sink::ShardSink,
    stage::Stage,
};

use async_tungstenite::{tokio::ConnectStream, WebSocketStream};

type ShardStream = WebSocketStream<ConnectStream>;
