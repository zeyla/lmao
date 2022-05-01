//! Used for building commands to send to Discord.

pub mod permissions;

mod command_type;
mod option;

pub use self::{
    command_type::CommandType,
    option::{
        BaseCommandOptionData, ChannelCommandOptionData, ChoiceCommandOptionData, CommandOption,
        CommandOptionChoice, CommandOptionType, CommandOptionValue, Number,
        NumberCommandOptionData, OptionsCommandOptionData,
    },
};

use crate::{
    guild::Permissions,
    id::{
        marker::{ApplicationMarker, CommandMarker, CommandVersionMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

/// Data sent to Discord to create a command.
///
/// [`CommandOption`]s that are required must be listed before optional ones.
/// Command names must be lower case, matching the Regex `^[\w-]{1,32}$`. See
/// [Discord Docs/Application Command Object].
///
/// This struct has an [associated builder] in the [`twilight-util`] crate.
///
/// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
/// [associated builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/struct.CommandBuilder.html
/// [Discord Docs/Application Command Object]: https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-structure
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Command {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    #[deprecated = "use `default_member_permissions` and `dm_permission` instead"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,
    /// Default permissions required for a member to run the command.
    ///
    /// Setting this [`Permissions::empty()`] will prohibit anyone from running
    /// the command, except for guild administrators.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permissions>,
    /// Whether the command is available in DMs.
    ///
    /// This is only relevant for globally-scoped commands. By default, commands
    /// are visible in DMs.
    pub dm_permission: Option<bool>,
    /// Description of the command.
    ///
    /// For [`User`] and [`Message`] commands, this will be an empty string.
    ///
    /// [`User`]: CommandType::User
    /// [`Message`]: CommandType::Message
    pub description: String,
    /// Guild ID of the command, if not global.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<CommandMarker>>,
    #[serde(rename = "type")]
    pub kind: CommandType,
    pub name: String,
    #[serde(default)]
    pub options: Vec<CommandOption>,
    /// Autoincrementing version identifier.
    pub version: Id<CommandVersionMarker>,
}
