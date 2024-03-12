//! Events that Lavalink sends to clients.

/// The type of event that something is.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum Opcode {
    /// Lavalink is connected and ready.
    Ready,
    /// An update about a player's current track.
    PlayerUpdate,
    /// Updated statistics about a node.
    Stats,
    /// Meta information about a track starting or ending.
    Event,
}


use crate::http::{Track, Exception};
use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::GuildMarker, Id};

/// An incoming event from a Lavalink node.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum IncomingEvent {
    /// Dispatched when you successfully connect to the Lavalink node.
    Ready(Ready),
    /// An update about the information of a player.
    PlayerUpdate(PlayerUpdate),
    /// New statistics about a node and its host.
    Stats(Stats),
    /// Dispatched when player or voice events occur.
    Event(Event),
}

impl From<Ready> for IncomingEvent {
    fn from(event: Ready) -> IncomingEvent {
        Self::Ready(event)
    }
}


impl From<Event> for IncomingEvent {
    fn from(event: Event) -> IncomingEvent {
        Self::Event(event)
    }
}

impl From<PlayerUpdate> for IncomingEvent {
    fn from(event: PlayerUpdate) -> IncomingEvent {
        Self::PlayerUpdate(event)
    }
}

impl From<Stats> for IncomingEvent {
    fn from(event: Stats) -> IncomingEvent {
        Self::Stats(event)
    }
}

/// The discord voice information that lavalink uses for connection and sending information.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct VoiceState {
    /// The Discord voice token to authenticate with.
    pub token: String,
    /// The Discord voice endpoint to connect to.
    pub endpoint: String,
    /// The Discord voice session id to authenticate with. Note this is seperate from the lavalink session id.
    pub session_id: String,
}

/// An update about the information of a player. Filters are currently unsupported
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlayerUpdate {
    /// Op code for this websocket event.
    pub op: Opcode,
    /// The guild ID of the player.
    pub guild_id: Id<GuildMarker>,
    /// The new state of the player.
    pub state: PlayerUpdateState,

}

/// New statistics about a node and its host.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct PlayerUpdateState {
    /// Unix timestamp of the player in milliseconds.
    pub time: i64,
    /// Track position in milliseconds. None if not playing anything.
    pub position: i64,
    /// True when the player is connected to the voice gateway.
    pub connected: bool,
    /// The ping of the node to the Discord voice server in milliseconds (-1 if not connected).
    pub ping: i64,
}

/// Dispatched by Lavalink upon successful connection and authorization. Contains fields determining if resuming was successful, as well as the session id.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Ready {
    /// Op code for this websocket event.
    pub op: Opcode,
    /// Whether this session was resumed.
    pub resumed: bool,
    /// The Lavalink session id of this connection. Not to be confused with a Discord voice session id.
    pub session_id: String,
}

/// Statistics about a node and its host.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// Op code for this websocket event.
    pub op: Opcode,
    /// CPU information about the node's host.
    pub cpu: StatsCpu,
    /// Statistics about audio frames.
    #[serde(rename = "frameStats", skip_serializing_if = "Option::is_none")]
    pub frame_stats: Option<StatsFrame>,
    /// Memory information about the node's host.
    pub memory: StatsMemory,
    /// The current number of total players (active and not active) within
    /// the node.
    pub players: u64,
    /// The current number of active players within the node.
    pub playing_players: u64,
    /// The uptime of the Lavalink server in seconds.
    pub uptime: u64,
}

/// CPU information about a node and its host.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatsCpu {
    /// The number of CPU cores.
    pub cores: usize,
    /// The load of the Lavalink server.
    pub lavalink_load: f64,
    /// The load of the system as a whole.
    pub system_load: f64,
}

/// CPU information about a node and its host.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatsFrame {
    /// The number of CPU cores.
    pub sent: i64,
    /// The load of the Lavalink server.
    pub nulled: i64,
    /// The load of the system as a whole.
    pub deficit: i64,
}

/// Memory information about a node and its host.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct StatsMemory {
    /// The number of bytes allocated.
    pub allocated: u64,
    /// The number of bytes free.
    pub free: u64,
    /// The number of bytes reservable.
    pub reservable: u64,
    /// The number of bytes used.
    pub used: u64,
}

/// Server dispatched an event. See the Event Types section for more information.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct Event {
    /// Op code for this websocket event.
    pub op: Opcode,
    /// The guild id that this was recieved from.
    pub guild_id: String,
    /// The type of event.
    pub r#type: EventType,
    /// The data of the event type.
    #[serde(flatten)]
    pub data: EventData,
}

/// Server dispatched an event.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
pub enum EventType {
    /// Dispatched when a track starts playing.
    TrackStartEvent,
    /// Dispatched when a track ends.
    TrackEndEvent,
    /// Dispatched when a track throws an exception.
    TrackExceptionEvent,
    /// Dispatched when a track gets stuck while playing.
    TrackStuckEvent,
    /// Dispatched when the websocket connection to Discord voice servers is closed.
    WebsocketClosedEvent,
}

/// Server dispatched an event.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum EventData {
    /// Dispatched when a track starts playing.
    TrackStartEvent(TrackStart),
    /// Dispatched when a track ends.
    TrackEndEvent(TrackEnd),
    /// Dispatched when a track throws an exception.
    TrackExceptionEvent(TrackException),
    /// Dispatched when a track gets stuck while playing.
    TrackStuckEvent(TrackStuck),
    /// Dispatched when the websocket connection to Discord voice servers is closed.
    WebsocketClosedEvent(WebsocketClosed),
}


/// The reason for the track ending.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub enum TrackEndReason {
    /// The track finished playing.
    Finished,
    /// The track failed to load.
    LoadFailed,
    /// The track was stopped.
    Stopped,
    /// The track was replaced
    Replaced,
    /// The track was cleaned up.
    Cleanup,
}


/// A track ended event from lavalink.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackEnd {
    /// The track that ended playing.
    pub track: Track,
    /// The reason that the track ended.
    pub reason: TrackEndReason,
}

/// A track started.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackStart {
    /// The track that started playing.
    pub track: Track,
}

/// Dispatched when a track throws an exception.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackException {
    /// The track that threw the exception.
    pub track: Track,
    /// The occurred exception.
    pub exception: Exception,
}

/// Dispatched when a track gets stuck while playing.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct TrackStuck {
    /// The track that got stuck.
    pub track: Track,
    /// The threshold in milliseconds that was exceeded.
    pub threshold_ms: u64,
}


/// The voice websocket connection to Discord has been closed.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(rename_all = "camelCase")]
pub struct WebsocketClosed {
    /// [Discord websocket opcode](https://discord.com/developers/docs/topics/opcodes-and-status-codes#voice-voice-close-event-codes) that closed the connection.
    pub code: u64,
    /// Reason the connection was closed.
    pub reason: String,
    /// True if Discord closed the connection, false if Lavalink closed it.
    pub by_remote: bool,
}
