//! Models to (de)serialize incoming/outgoing websocket events and HTTP
//! responses.

pub mod outgoing {
    //! Events that clients send to Lavalink.
    use serde::{Deserialize, Serialize};
    use twilight_model::{
        gateway::payload::incoming::VoiceServerUpdate,
        id::{marker::GuildMarker, Id},
    };

    use crate::http::UpdatePlayerTrack;

    /// An outgoing event to send to Lavalink.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(untagged)]
    pub enum OutgoingEvent {
        /// Destroy a player for a guild.
        Destroy(Destroy),
        /// Equalize a player.
        Equalizer(Equalizer),
        /// Pause or unpause a player.
        Pause(Pause),
        /// Play a track.
        Play(Play),
        /// Seek a player's active track to a new position.
        Seek(Seek),
        /// Stop a player.
        Stop(Stop),
        /// A combined voice server and voice state update.
        VoiceUpdate(VoiceUpdate),
        /// Set the volume of a player.
        Volume(Volume),
    }

    impl From<Destroy> for OutgoingEvent {
        fn from(event: Destroy) -> OutgoingEvent {
            Self::Destroy(event)
        }
    }

    impl From<Equalizer> for OutgoingEvent {
        fn from(event: Equalizer) -> OutgoingEvent {
            Self::Equalizer(event)
        }
    }

    impl From<Pause> for OutgoingEvent {
        fn from(event: Pause) -> OutgoingEvent {
            Self::Pause(event)
        }
    }

    impl From<Play> for OutgoingEvent {
        fn from(event: Play) -> OutgoingEvent {
            Self::Play(event)
        }
    }

    impl From<Seek> for OutgoingEvent {
        fn from(event: Seek) -> OutgoingEvent {
            Self::Seek(event)
        }
    }

    impl From<Stop> for OutgoingEvent {
        fn from(event: Stop) -> OutgoingEvent {
            Self::Stop(event)
        }
    }

    impl From<VoiceUpdate> for OutgoingEvent {
        fn from(event: VoiceUpdate) -> OutgoingEvent {
            Self::VoiceUpdate(event)
        }
    }

    impl From<Volume> for OutgoingEvent {
        fn from(event: Volume) -> OutgoingEvent {
            Self::Volume(event)
        }
    }

    /// Destroy a player from a node.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Destroy {
        /// The guild ID of the player.
        pub guild_id: Id<GuildMarker>,
    }

    impl Destroy {
        /// Create a new destroy event.
        pub const fn new(guild_id: Id<GuildMarker>) -> Self {
            Self {
                guild_id,
            }
        }
    }

    impl From<Id<GuildMarker>> for Destroy {
        fn from(guild_id: Id<GuildMarker>) -> Self {
            Self {
                guild_id,
            }
        }
    }



    /// Equalize a player.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Equalizer {
        /// The bands to use as part of the equalizer.
        pub bands: Vec<EqualizerBand>,
        /// The guild ID of the player.
        #[serde(skip_serializing)]
        pub guild_id: Id<GuildMarker>,
    }

    impl Equalizer {
        /// Create a new equalizer event.
        pub fn new(guild_id: Id<GuildMarker>, bands: Vec<EqualizerBand>) -> Self {
            Self::from((guild_id, bands))
        }
    }

    impl From<(Id<GuildMarker>, Vec<EqualizerBand>)> for Equalizer {
        fn from((guild_id, bands): (Id<GuildMarker>, Vec<EqualizerBand>)) -> Self {
            Self {
                bands,
                guild_id,
            }
        }
    }

    /// A band of the equalizer event.
    #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct EqualizerBand {
        /// The band.
        pub band: i64,
        /// The gain.
        pub gain: f64,
    }

    impl EqualizerBand {
        /// Create a new equalizer band.
        pub fn new(band: i64, gain: f64) -> Self {
            Self::from((band, gain))
        }
    }

    impl From<(i64, f64)> for EqualizerBand {
        fn from((band, gain): (i64, f64)) -> Self {
            Self { band, gain }
        }
    }

    /// Pause or unpause a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Pause {
        /// The guild ID of the player.
        pub guild_id: Id<GuildMarker>,
        /// Whether to pause the player.
        ///
        /// Set to `true` to pause or `false` to resume.
        pub paused: bool,
    }

    impl Pause {
        /// Create a new pause event.
        ///
        /// Set to `true` to pause the player or `false` to resume it.
        pub fn new(guild_id: Id<GuildMarker>, pause: bool) -> Self {
            Self::from((guild_id, pause))
        }
    }

    impl From<(Id<GuildMarker>, bool)> for Pause {
        fn from((guild_id, pause): (Id<GuildMarker>, bool)) -> Self {
            Self {
                guild_id,
                paused: pause,
            }
        }
    }


    // TODO: Might need to fix this struct to abstract the guild_id to another struct pending on what the server sends back with it included.
    /// Play a track, optionally specifying to not skip the current track. Filters are not supported at the moment.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Play {
        /// Information about the track to play.
        pub track: UpdatePlayerTrack,
        /// The position in milliseconds to start the track from.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub position: Option<u64>,
        /// The position in milliseconds to end the track.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub end_time: Option<Option<u64>>,
        /// The player volume, in percentage, from 0 to 1000
        #[serde(skip_serializing_if = "Option::is_none")]
        pub volume: Option<u64>,
        ///     Whether the player is paused
        #[serde(skip_serializing_if = "Option::is_none")]
        pub paused: Option<bool>,
        /// The guild ID of the player.
        #[serde(skip_serializing)]
        pub guild_id: Id<GuildMarker>,
        /// Whether or not to replace the currently playing track with this new
        /// track.
        ///
        /// Set to `true` to keep playing the current playing track, or `false`
        /// to replace the current playing track with a new one.
        #[serde(skip_serializing)]
        pub no_replace: bool,
    }

    impl Play {
        /// Create a new play event.
        pub fn new(
            guild_id: Id<GuildMarker>,
            track: impl Into<String>,
            start_time: impl Into<Option<u64>>,
            end_time: impl Into<Option<u64>>,
            no_replace: bool,
        ) -> Self {
            Self::from((guild_id, track, start_time, end_time, no_replace))
        }
    }

    impl<T: Into<String>> From<(Id<GuildMarker>, T)> for Play {
        fn from((guild_id, track): (Id<GuildMarker>, T)) -> Self {
            Self::from((guild_id, track, None, None, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>> From<(Id<GuildMarker>, T, S)> for Play {
        fn from((guild_id, track, start_time): (Id<GuildMarker>, T, S)) -> Self {
            Self::from((guild_id, track, start_time, None, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>, E: Into<Option<u64>>>
        From<(Id<GuildMarker>, T, S, E)> for Play
    {
        fn from((guild_id, track, start_time, end_time): (Id<GuildMarker>, T, S, E)) -> Self {
            Self::from((guild_id, track, start_time, end_time, true))
        }
    }

    impl<T: Into<String>, S: Into<Option<u64>>, E: Into<Option<u64>>>
        From<(Id<GuildMarker>, T, S, E, bool)> for Play
    {
        fn from(
            (guild_id, track, start_time, end_time, no_replace): (Id<GuildMarker>, T, S, E, bool),
        ) -> Self {
            Self {
                guild_id,
                no_replace,
                position: start_time.into(),
                end_time: Some(end_time.into()),
                volume: None,
                paused: None,
                track: UpdatePlayerTrack{
                    encoded: Some(track.into()),
                },
            }
        }
    }

    /// Seek a player's active track to a new position.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Seek {
        /// The guild ID of the player.
        #[serde(skip_serializing)]
        pub guild_id: Id<GuildMarker>,
        /// The position in milliseconds to seek to.
        pub position: i64,
    }

    impl Seek {
        /// Create a new seek event.
        pub fn new(guild_id: Id<GuildMarker>, position: i64) -> Self {
            Self::from((guild_id, position))
        }
    }

    impl From<(Id<GuildMarker>, i64)> for Seek {
        fn from((guild_id, position): (Id<GuildMarker>, i64)) -> Self {
            Self {
                guild_id,
                position,
            }
        }
    }

    /// Stop a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Stop {
        /// The guild ID of the player.
        #[serde(skip_serializing)]
        pub guild_id: Id<GuildMarker>,
        /// The track object to pass set to null
        pub track: UpdatePlayerTrack,
    }

    impl Stop {
        /// Create a new stop event.
        pub fn new(guild_id: Id<GuildMarker>) -> Self {
            Self::from(guild_id)
        }
    }

    impl From<Id<GuildMarker>> for Stop {
        fn from(guild_id: Id<GuildMarker>) -> Self {
            Self {
                guild_id,
                track: UpdatePlayerTrack {
                    encoded: None,
                },
            }
        }
    }
    /// The voice payload for the combined server and state to send to lavalink.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Voice {
        /// The Discord voice token to authenticate with.
        pub token: String,
        /// The Discord voice endpoint to connect to.
        pub endpoint: String,
        /// The Discord voice session id to authenticate with. This is seperate from the session id of lavalink.
        pub session_id: String,
    }

    /// A combined voice server and voice state update.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct VoiceUpdate {
        /// The guild ID of the player.
        #[serde(skip_serializing)]
        pub guild_id: Id<GuildMarker>,
        /// The voice payload for the combined server and state to send to lavalink.
        pub voice: Voice,
    }

    impl VoiceUpdate {
        /// Create a new voice update event.
        pub fn new(
            guild_id: Id<GuildMarker>,
            session_id: impl Into<String>,
            event: VoiceServerUpdate,
        ) -> Self {
            Self::from((guild_id, session_id, event))
        }
    }

    impl<T: Into<String>> From<(Id<GuildMarker>, T, VoiceServerUpdate)> for VoiceUpdate {
        fn from((guild_id, session_id, event): (Id<GuildMarker>, T, VoiceServerUpdate)) -> Self {
            Self {
                guild_id: guild_id,
                voice: Voice{
                    token: event.token,
                    endpoint: event.endpoint.unwrap_or("NO_ENDPOINT_RETURNED".to_string()),
                    session_id: session_id.into(),
                }
            }
        }
    }

    /// Set the volume of a player.
    #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
    #[non_exhaustive]
    #[serde(rename_all = "camelCase")]
    pub struct Volume {
        /// The guild ID of the player.
        #[serde(skip_serializing)]
        pub guild_id: Id<GuildMarker>,
        /// The volume of the player from 0 to 1000. 100 is the default.
        pub volume: i64,
    }

    impl Volume {
        /// Create a new volume event.
        pub fn new(guild_id: Id<GuildMarker>, volume: i64) -> Self {
            Self::from((guild_id, volume))
        }
    }

    impl From<(Id<GuildMarker>, i64)> for Volume {
        fn from((guild_id, volume): (Id<GuildMarker>, i64)) -> Self {
            Self {
                guild_id,
                volume,
            }
        }
    }
}

pub mod incoming {
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
        // /// Dispatched when player or voice events occur.
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
        pub frame_stats: Option<StatsFrames>,
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
    pub struct StatsFrames {
        /// The number of CPU cores.
        pub sent: u64,
        /// The load of the Lavalink server.
        pub nulled: u64,
        /// The load of the system as a whole.
        pub deficit: u64,
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
}

pub use self::{
    incoming::{
        IncomingEvent, PlayerUpdate, PlayerUpdateState, Stats, StatsCpu, StatsFrames, StatsMemory,
        TrackEnd, TrackStart, TrackStuck, TrackException, WebsocketClosed,
    },
    outgoing::{
        Destroy, Equalizer, EqualizerBand, OutgoingEvent, Pause, Play, Seek, Stop, VoiceUpdate,
        Volume,
    },
};

#[cfg(test)]
mod lavalink_struct_tests {
    use super::{
        incoming::{
            IncomingEvent, PlayerUpdate, PlayerUpdateState, Stats, StatsCpu, StatsFrames,
            StatsMemory, TrackEnd, TrackStart, WebsocketClosed,
        },
        outgoing::{
            Destroy, Equalizer, EqualizerBand, OutgoingEvent, Pause, Play, Seek, Stop, VoiceUpdate,
            Volume,
        },
    };
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;
    use twilight_model::{
        gateway::payload::incoming::VoiceServerUpdate,
        id::{marker::GuildMarker, Id},
    };

    assert_fields!(Destroy: guild_id);
    assert_impl_all!(
        Destroy: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        From<Id<GuildMarker>>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(EqualizerBand: band, gain);
    assert_impl_all!(
        EqualizerBand: Clone,
        Debug,
        Deserialize<'static>,
        From<(i64, f64)>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(Equalizer: bands, guild_id);
    assert_impl_all!(
        Equalizer: Clone,
        Debug,
        Deserialize<'static>,
        From<(Id<GuildMarker>, Vec<EqualizerBand>)>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_impl_all!(
        IncomingEvent: Clone,
        Debug,
        Deserialize<'static>,
        From<PlayerUpdate>,
        From<Stats>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_impl_all!(
        OutgoingEvent: Clone,
        Debug,
        Deserialize<'static>,
        From<Destroy>,
        From<Equalizer>,
        From<Pause>,
        From<Play>,
        From<Seek>,
        From<Stop>,
        From<VoiceUpdate>,
        From<Volume>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(Pause: guild_id, paused);
    assert_impl_all!(
        Pause: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        From<(Id<GuildMarker>, bool)>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(PlayerUpdateState: position, time);
    assert_impl_all!(
        PlayerUpdateState: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(PlayerUpdate: guild_id, state);
    assert_impl_all!(
        PlayerUpdate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(Play: end_time, guild_id, no_replace, position, track);
    assert_impl_all!(
        Play: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        From<(Id<GuildMarker>, String)>,
        From<(Id<GuildMarker>, String, Option<u64>)>,
        From<(Id<GuildMarker>, String, u64)>,
        From<(Id<GuildMarker>, String, Option<u64>, Option<u64>)>,
        From<(Id<GuildMarker>, String, Option<u64>, u64)>,
        From<(Id<GuildMarker>, String, u64, Option<u64>)>,
        From<(Id<GuildMarker>, String, u64, u64)>,
        From<(Id<GuildMarker>, String, Option<u64>, Option<u64>, bool)>,
        From<(Id<GuildMarker>, String, Option<u64>, u64, bool)>,
        From<(Id<GuildMarker>, String, u64, Option<u64>, bool)>,
        From<(Id<GuildMarker>, String, u64, u64, bool)>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(Seek: guild_id, position);
    assert_impl_all!(
        Seek: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        From<(Id<GuildMarker>, i64)>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(
        Stats: cpu,
        frame_stats,
        memory,
        players,
        playing_players,
        uptime
    );
    assert_impl_all!(
        Stats: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(StatsCpu: cores, lavalink_load, system_load);
    assert_impl_all!(
        StatsCpu: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(StatsFrames: deficit, nulled, sent);
    assert_impl_all!(
        StatsFrames: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(StatsMemory: allocated, free, reservable, used);
    assert_impl_all!(
        StatsMemory: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(Stop: guild_id);
    assert_impl_all!(
        Stop: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        From<Id<GuildMarker>>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(TrackEnd: reason, track);
    assert_impl_all!(
        TrackEnd: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(TrackStart: track);
    assert_impl_all!(
        TrackStart: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(WebsocketClosed: code, reason, by_remote);
    assert_impl_all!(
        WebsocketClosed: Clone,
        Debug,
        Deserialize<'static>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(VoiceUpdate: guild_id, voice);
    assert_impl_all!(
        VoiceUpdate: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        From<(Id<GuildMarker>, String, VoiceServerUpdate)>,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );
    assert_fields!(Volume: guild_id, volume);
    assert_impl_all!(
        Volume: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync,
    );

    #[test]
    fn stats_frames_not_provided() {
        const LAVALINK_LOAD: f64 = 0.276_119_402_985_074_65;
        const MEM_ALLOCATED: u64 = 62_914_560;
        const MEM_FREE: u64 = 27_664_576;
        const MEM_RESERVABLE: u64 = 4_294_967_296;
        const MEM_USED: u64 = 35_249_984;
        const SYSTEM_LOAD: f64 = 0.195_380_536_378_835_9;

        let expected = Stats {
            op: crate::model::incoming::Opcode::Stats,
            cpu: StatsCpu {
                cores: 4,
                lavalink_load: LAVALINK_LOAD,
                system_load: SYSTEM_LOAD,
            },
            frame_stats: None,
            memory: StatsMemory {
                allocated: MEM_ALLOCATED,
                free: MEM_FREE,
                reservable: MEM_RESERVABLE,
                used: MEM_USED,
            },
            players: 0,
            playing_players: 0,
            uptime: 18589,
        };

        serde_test::assert_de_tokens(
            &expected,
            &[
                Token::Struct {
                    name: "Stats",
                    len: 6,
                },
                Token::Str("cpu"),
                Token::Struct {
                    name: "StatsCpu",
                    len: 3,
                },
                Token::Str("cores"),
                Token::U64(4),
                Token::Str("lavalinkLoad"),
                Token::F64(LAVALINK_LOAD),
                Token::Str("systemLoad"),
                Token::F64(SYSTEM_LOAD),
                Token::StructEnd,
                Token::Str("memory"),
                Token::Struct {
                    name: "StatsMemory",
                    len: 4,
                },
                Token::Str("allocated"),
                Token::U64(MEM_ALLOCATED),
                Token::Str("free"),
                Token::U64(MEM_FREE),
                Token::Str("reservable"),
                Token::U64(MEM_RESERVABLE),
                Token::Str("used"),
                Token::U64(MEM_USED),
                Token::StructEnd,
                Token::Str("op"),
                Token::UnitVariant {
                    name: "Opcode",
                    variant: "stats",
                },
                Token::Str("players"),
                Token::U64(0),
                Token::Str("playingPlayers"),
                Token::U64(0),
                Token::Str("uptime"),
                Token::U64(18589),
                Token::StructEnd,
            ],
        );
    }
}

#[cfg(test)]
mod lavalink_incoming_model_tests {
    use crate::model::TrackStart;
    use twilight_model::id::{
        Id,
        marker::GuildMarker,
    };

    use crate::http::{Track, TrackInfo};

    use super::incoming::{
            Event, EventType, EventData, Opcode, PlayerUpdate, PlayerUpdateState, Ready
        };


    // These are incoming so we only need to check that the input json can deserialize into the struct.
    fn compare_json_payload<T: serde::Serialize + std::fmt::Debug + for<'a> serde::Deserialize<'a> + std::cmp::PartialEq>
        (data_struct: T, json_payload: String) {
        // Deserialize
        let deserialized: T = serde_json::from_str(&json_payload).unwrap();
        assert_eq!(deserialized, data_struct);
    }

    #[test]
    fn should_serialize_a_ready_response() {
        let ready = Ready {
            op: Opcode::Ready,
            resumed: false,
            session_id: "la3kfsdf5eafe848".to_string(),
        };
        compare_json_payload(
            ready,
            r#"{"op":"ready","resumed":false,"sessionId":"la3kfsdf5eafe848"}"#.to_string()
            );
    }

    #[test]
    fn should_serialize_a_player_update_response() {
        let update = PlayerUpdate {
            op: Opcode::PlayerUpdate,
            guild_id: Id::<GuildMarker>::new(987654321),
            state: PlayerUpdateState{
                time: 1710214147839,
                position: 534,
                connected: true,
                ping: 0,
            },
        };
        compare_json_payload(
            update,
            r#"{"op":"playerUpdate","guildId":"987654321","state":{"time":1710214147839,"position":534,"connected":true,"ping":0}}"#.to_string()
            );
    }

    #[test]
    fn should_serialize_track_start_event() {
        let track_start_event = Event {
            op: Opcode::Event,
            r#type: EventType::TrackStartEvent,
            guild_id: Id::<GuildMarker>::new(987654321).to_string(),
            data: EventData::TrackStartEvent(
                TrackStart { track: Track {
                    encoded: "QAAAzgMAMUJsZWVkIEl0IE91dCBbT2ZmaWNpYWwgTXVzaWMgVmlkZW9dIC0gTGlua2luIFBhcmsAC0xpbmtpbiBQYXJrAAAAAAAClCgAC09udXVZY3FoekNFAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9T251dVljcWh6Q0UBADRodHRwczovL2kueXRpbWcuY29tL3ZpL09udXVZY3FoekNFL21heHJlc2RlZmF1bHQuanBnAAAHeW91dHViZQAAAAAAAAAA".to_string(),
                    info: TrackInfo {
                        identifier: "OnuuYcqhzCE".to_string(),
                        is_seekable: true,
                        author: "Linkin Park".to_string(),
                        length: 169000,
                        is_stream: false,
                        position: 0,
                        title: "Bleed It Out [Official Music Video] - Linkin Park".to_string(),
                        uri:Some("https://www.youtube.com/watch?v=OnuuYcqhzCE".to_string()),
                        source_name:"youtube".to_string(),
                        artwork_url:Some("https://i.ytimg.com/vi/OnuuYcqhzCE/maxresdefault.jpg".to_string()),
                        isrc: None
                    }
                } })

        };
        compare_json_payload(
            track_start_event.clone(),
            r#"{"op":"event","guildId":"987654321","type":"TrackStartEvent","track":{"encoded":"QAAAzgMAMUJsZWVkIEl0IE91dCBbT2ZmaWNpYWwgTXVzaWMgVmlkZW9dIC0gTGlua2luIFBhcmsAC0xpbmtpbiBQYXJrAAAAAAAClCgAC09udXVZY3FoekNFAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9T251dVljcWh6Q0UBADRodHRwczovL2kueXRpbWcuY29tL3ZpL09udXVZY3FoekNFL21heHJlc2RlZmF1bHQuanBnAAAHeW91dHViZQAAAAAAAAAA","info":{"identifier":"OnuuYcqhzCE","isSeekable":true,"author":"Linkin Park","length":169000,"isStream":false,"position":0,"title":"Bleed It Out [Official Music Video] - Linkin Park","uri":"https://www.youtube.com/watch?v=OnuuYcqhzCE","artworkUrl":"https://i.ytimg.com/vi/OnuuYcqhzCE/maxresdefault.jpg","isrc":null,"sourceName":"youtube"},"pluginInfo":{},"userData":{}}}"#.to_string()
            );
    }
}


#[cfg(test)]
mod lavalink_outgoing_model_tests {
    use crate::model::Play;
    use crate::http::UpdatePlayerTrack;

    use twilight_model::id::{
        Id,
        marker::GuildMarker,
    };

    use super::outgoing::{
            OutgoingEvent, VoiceUpdate, Voice,
        };


    // For some of the outgoing we have fields that don't get deserialized. We only need
    // to check weather the serialization is working.
    fn compare_json_payload<T: serde::Serialize + std::fmt::Debug + std::cmp::PartialEq>
        (data_struct: T, json_payload: String) {

        let serialized = serde_json::to_string(&data_struct).unwrap();
        let expected_serialized = json_payload;
        assert_eq!(serialized, expected_serialized);
    }

    #[test]
    fn should_serialize_an_outgoing_voice_update() {
        let voice = VoiceUpdate {
            guild_id: Id::<GuildMarker>::new(987654321),
            voice: Voice{
                token: String::from("863ea8ef2ads8ef2"),
                endpoint: String::from("eu-centra654863.discord.media:443"),
                session_id: String::from("asdf5w1efa65feaf315e8a8effsa1e5f"),
            },
        };
        compare_json_payload(
            voice,
            r#"{"voice":{"token":"863ea8ef2ads8ef2","endpoint":"eu-centra654863.discord.media:443","sessionId":"asdf5w1efa65feaf315e8a8effsa1e5f"}}"#.to_string()
            );
    }

    #[test]
    fn should_serialize_an_outgoing_play() {
        let play = OutgoingEvent::Play(Play{
            track: UpdatePlayerTrack {
                encoded: Some("QAAAzgMAMUJsZWVkIEl0IE91dCBbT2ZmaWNpYWwgTXVzaWMgVmlkZW9dIC0gTGlua2luIFBhcmsAC0xpbmtpbiBQYXJrAAAAAAAClCgAC09udXVZY3FoekNFAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9T251dVljcWh6Q0UBADRodHRwczovL2kueXRpbWcuY29tL3ZpL09udXVZY3FoekNFL21heHJlc2RlZmF1bHQuanBnAAAHeW91dHViZQAAAAAAAAAA".to_string()),
            },
            position: None,
            end_time: Some(None),
            volume: None,
            paused: None,
            guild_id: Id::<GuildMarker>::new(987654321),
            no_replace: true,
        });
        compare_json_payload(
            play,
            r#"{"track":{"encoded":"QAAAzgMAMUJsZWVkIEl0IE91dCBbT2ZmaWNpYWwgTXVzaWMgVmlkZW9dIC0gTGlua2luIFBhcmsAC0xpbmtpbiBQYXJrAAAAAAAClCgAC09udXVZY3FoekNFAAEAK2h0dHBzOi8vd3d3LnlvdXR1YmUuY29tL3dhdGNoP3Y9T251dVljcWh6Q0UBADRodHRwczovL2kueXRpbWcuY29tL3ZpL09udXVZY3FoekNFL21heHJlc2RlZmF1bHQuanBnAAAHeW91dHViZQAAAAAAAAAA"},"endTime":null}"#.to_string()
            );
    }
}
