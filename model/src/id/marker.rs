//! Markers for various resource types, such as channels or users.
//!
//! Markers themselves perform no logical action, and are only used to
//! ensure that IDs of incorrect types aren't used. If IDs were only 64-bit
//! integers then a role's ID may be erroneously used in the place of where
//! a user's ID is required; by using markers it can be ensured that only an
//! ID with a [`Role`] can be used where a role's ID is required.

/// Marker for application IDs.
///
/// Types such as [`Message::application_id`] or [`Guild::application_id`]
/// use this ID marker.
///
/// [`Guild::application_id`]: crate::guild::Guild::application_id
/// [`Message::application_id`]: crate::channel::Message::application_id
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Application;

/// Marker for attachment IDs.
///
/// Types such as [`Attachment`] use this ID marker.
///
/// [`Attachment`]: crate::channel::Attachment
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Attachment;

/// Marker for audit log entry IDs.
///
/// Types such as [`AuditLogEntry`] use this ID marker.
///
/// [`AuditLogEntry`]: crate::guild::audit_log::AuditLogEntry
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct AuditLogEntry;

/// Marker for channel IDs.
///
/// Types such as [`PrivateChannel`] or [`TextChannel`] use this ID marker.
///
/// [`PrivateChannel`]: crate::channel::PrivateChannel
/// [`TextChannel`]: crate::channel::TextChannel
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Channel;

/// Marker for command IDs.
///
/// Types such as [`Command`] use this ID marker.
///
/// [`Command`]: crate::application::command::Command
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Command;

/// Marker for command versions.
///
/// Types such as [`Command`] use this ID marker.
///
/// [`Command`]: crate::application::command::Command
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct CommandVersion;

/// Marker for emoji IDs.
///
/// Types such as [`Emoji`] or [`ReactionType`] use this ID marker.
///
/// [`Emoji`]: crate::guild::Emoji
/// [`ReactionType`]: crate::channel::ReactionType
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Emoji;

/// Marker for generic IDs.
///
/// Types such as [`AuditLogChange::Id`] or [`CommandOptionValue`] use this
/// ID marker.
///
/// [`AuditLogChange::Id`]: crate::guild::audit_log::AuditLogChange::Id
/// [`CommandOptionValue`]: crate::application::interaction::application_command::CommandOptionValue
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Generic;

/// Marker for guild IDs.
///
/// Types such as [`Guild`] or [`Message`] use this ID marker.
///
/// [`Guild`]: crate::guild::Guild
/// [`Message`]: crate::channel::Message
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Guild;

/// Marker for integration IDs.
///
/// Types such as [`GuildIntegration`] or [`RoleTags`] use this ID marker.
///
/// [`GuildIntegration`]: crate::guild::GuildIntegration
/// [`RoleTags`]: crate::guild::RoleTags
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Integration;

/// Marker for interaction IDs.
///
/// Types such as [`Interaction`] or [`MessageInteraction`] use this ID
/// marker.
///
/// [`Interaction`]: crate::application::interaction::Interaction
/// [`MessageInteraction`]: crate::channel::message::MessageInteraction
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Interaction;

/// Marker for message IDs.
///
/// Types such as [`Message`] or [`Reaction`] use this ID marker.
///
/// [`Message`]: crate::channel::Message
/// [`Reaction`]: crate::channel::Reaction
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Message;

/// Marker for OAuth SKU IDs.
///
/// Types such as [`CurrentApplicationInfo`] use this ID marker.
///
/// [`CurrentApplicationInfo`]: crate::oauth::CurrentApplicationInfo
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct OauthSku;

/// Marker for OAuth team IDs.
///
/// Types such as [`Team`] or [`TeamMember`] use this ID marker.
///
/// [`Team`]: crate::oauth::team::Team
/// [`TeamMember`]: crate::oauth::team::TeamMember
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct OauthTeam;

/// Marker for role IDs.
///
/// Types such as [`Member`] or [`Role`] use this ID marker.
///
/// [`Member`]: crate::guild::Member
/// [`Role`]: crate::guild::Role
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Role;

/// Marker for stage IDs.
///
/// Types such as [`StageInstance`] use this ID marker.
///
/// [`StageInstance`]: crate::channel::StageInstance
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Stage;

/// Marker for sticker banner asset IDs.
///
/// Types such as [`StickerPack`] use this ID marker.
///
/// [`StickerPack`]: crate::channel::message::sticker::StickerPack
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct StickerBannerAsset;

/// Marker for sticker IDs.
///
/// Types such as [`Message`] or [`Sticker`] use this ID marker.
///
/// [`Message`]: crate::channel::Message
/// [`Sticker`]: crate::channel::message::sticker::Sticker
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Sticker;

/// Marker for sticker pack IDs.
///
/// Types such as [`Sticker`] or [`StickerPack`] use this ID marker.
///
/// [`Sticker`]: crate::channel::message::sticker::Sticker
/// [`StickerPack`]: crate::channel::message::sticker::StickerPack
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct StickerPack;

/// Marker for sticker pack SKU IDs.
///
/// Types such as [`StickerPack`] use this ID marker.
///
/// [`StickerPack`]: crate::channel::message::sticker::StickerPack
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct StickerPackSku;

/// Marker for user IDs.
///
/// Types such as [`PublicThread`] or [`User`] use this ID marker.
///
/// [`PublicThread`]: crate::channel::thread::PublicThread
/// [`User`]: crate::user::User
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct User;

/// Marker for webhook IDs.
///
/// Types such as [`Webhook`] use this ID marker.
///
/// [`Webhook`]: crate::channel::webhook::Webhook
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct Webhook;
