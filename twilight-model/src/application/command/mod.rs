//! Commands user's may natively interact with.
//!
//! It is highly recommended to use the associated [`CommandBuilder`] in the
//! [`twilight-util`] to create [`Command`]s; [`CommandOption`] is especially
//! verbose.
//!
//! [`CommandBuilder`]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/index.html
//! [`twilight-util`]: https://docs.rs/twilight-util

pub mod permissions;

mod option;

pub use self::option::{
    CommandOption, CommandOptionChoice, CommandOptionChoiceData, CommandOptionType,
    CommandOptionValue,
};

use crate::{
    guild::Permissions,
    id::{
        marker::{ApplicationMarker, CommandMarker, CommandVersionMarker, GuildMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Command user's may execute.
///
/// The description and name may be localized in any [available locale],
/// see [Discord Docs/Localization].
///
/// [`ChatInput`] command names and options, and their localizations, must match
/// the Regex `^[-_\p{L}\p{N}\p{sc=Deva}\p{sc=Thai}]{1,32}$`.
///
/// [available locale]: https://discord.com/developers/docs/reference#locales
/// [`ChatInput`]: CommandType::ChatInput
/// [Discord Docs/Localization]: https://discord.com/developers/docs/interactions/application-commands#localization
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Command {
    /// Parent application ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<Id<ApplicationMarker>>,
    /// Default permissions required for a member to run the command.
    ///
    /// Defaults to no required permission.
    ///
    /// Setting this [`Permissions::empty()`] will prohibit anyone except admins
    /// from running the command.
    pub default_member_permissions: Option<Permissions>,
    /// Whether the command is available in DMs.
    ///
    /// Applicable for globally-scoped commands.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,
    /// Description of the command. Must be 100 characters or less.
    ///
    /// Ignored for [`Message`] and [`User`] commands.
    ///
    /// [`User`]: CommandType::User
    /// [`Message`]: CommandType::Message
    pub description: String,
    /// Localization dictionary for the [`description`] field.
    ///
    /// Defaults to no localizations.
    ///
    /// Keys must be valid locales and values must be 100 characters or less.
    ///
    /// [`description`]: Self::description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localizations: Option<HashMap<String, String>>,
    /// Guild ID of the command.
    ///
    /// Defaults to being globally-scoped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<Id<GuildMarker>>,
    /// Unique command ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<CommandMarker>>,
    /// Type of command.
    #[serde(rename = "type")]
    pub kind: CommandType,
    /// Name of the command. Must be 32 characters or less.
    pub name: String,
    /// Localization dictionary for the [`name`] field.
    ///
    /// Defaults to no localizations.
    ///
    /// Keys must be valid locales and values must be 32 characters or less.
    ///
    /// [`name`]: Self::name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localizations: Option<HashMap<String, String>>,
    #[serde(default)]
    /// List of command options.
    ///
    /// Applicable for commands of type [`ChatInput`].
    ///
    /// Required options must be listed before optional ones.
    ///
    /// [`ChatInput`]: CommandType::ChatInput
    pub options: Vec<CommandOption>,
    /// Autoincrementing version identifier.
    pub version: Id<CommandVersionMarker>,
}

/// Type of a [`Command`].
// Keep in sync with `twilight-validate::command`!
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[non_exhaustive]
#[serde(from = "u8", into = "u8")]
pub enum CommandType {
    /// Slash command.
    ///
    /// Text-based command that appears when a user types `/`.
    ChatInput,
    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps on a user.
    User,
    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps on a message.
    Message,
    /// Variant value is unknown to the library.
    Unknown(u8),
}

impl From<u8> for CommandType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::ChatInput,
            2 => Self::User,
            3 => Self::Message,
            unknown => Self::Unknown(unknown),
        }
    }
}

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        match value {
            CommandType::ChatInput => 1,
            CommandType::User => 2,
            CommandType::Message => 3,
            CommandType::Unknown(unknown) => unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Command, CommandOption, CommandOptionChoice, CommandOptionChoiceData, CommandOptionType,
        CommandOptionValue, CommandType,
    };
    use crate::{channel::ChannelType, guild::Permissions, id::Id};
    use serde::{Deserialize, Serialize};
    use serde_test::{assert_tokens, Token};
    use static_assertions::assert_impl_all;
    use std::collections::HashMap;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        CommandType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize,
        Send,
        Sync
    );

    #[test]
    fn type_variants() {
        assert_tokens(&CommandType::ChatInput, &[Token::U8(1)]);
        assert_tokens(&CommandType::User, &[Token::U8(2)]);
        assert_tokens(&CommandType::Message, &[Token::U8(3)]);
        assert_tokens(&CommandType::Unknown(99), &[Token::U8(99)]);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn command_option_full() {
        let value = Command {
            application_id: Some(Id::new(100)),
            default_member_permissions: Some(Permissions::ADMINISTRATOR),
            dm_permission: Some(false),
            description: "this command is a test".into(),
            description_localizations: Some(HashMap::from([(
                "en-US".into(),
                "this command is a test".into(),
            )])),
            guild_id: Some(Id::new(300)),
            id: Some(Id::new(200)),
            kind: CommandType::ChatInput,
            name: "test command".into(),
            name_localizations: Some(HashMap::from([("en-US".into(), "test command".into())])),
            options: Vec::from([CommandOption {
                autocomplete: None,
                channel_types: None,
                choices: None,
                description: "sub command group desc".to_owned(),
                description_localizations: None,
                kind: CommandOptionType::SubCommandGroup,
                max_length: None,
                max_value: None,
                min_length: None,
                min_value: None,
                name: "sub command group name".to_owned(),
                name_localizations: None,
                options: Some(Vec::from([CommandOption {
                    autocomplete: None,
                    channel_types: None,
                    choices: None,
                    description: "sub command desc".to_owned(),
                    description_localizations: None,
                    kind: CommandOptionType::SubCommand,
                    max_length: None,
                    max_value: None,
                    min_length: None,
                    min_value: None,
                    name: "sub command name".to_owned(),
                    name_localizations: None,
                    options: Some(Vec::from([
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "attachment desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Attachment,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "attachment name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "boolean desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Boolean,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "boolean name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: Some(true),
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: Some(Vec::new()),
                            choices: None,
                            description: "channel desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Channel,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "channel name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: Some(Vec::from([ChannelType::GuildText])),
                            choices: None,
                            description: "channel desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Channel,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "channel name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: Some(true),
                            channel_types: None,
                            choices: Some(Vec::new()),
                            description: "integer desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Integer,
                            max_length: None,
                            max_value: Some(CommandOptionValue::Integer(100)),
                            min_length: None,
                            min_value: Some(CommandOptionValue::Integer(0)),
                            name: "integer name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "mentionable desc".to_owned(),
                            description_localizations: Some(HashMap::from([(
                                "en-GB".to_owned(),
                                "mentionable desc (but british)".to_owned(),
                            )])),
                            kind: CommandOptionType::Mentionable,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "mentionable name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: Some(false),
                            channel_types: None,
                            choices: Some(Vec::from([CommandOptionChoice::Number(
                                CommandOptionChoiceData {
                                    name: "number choice".to_owned(),
                                    name_localizations: Some(HashMap::from([(
                                        "en-US".to_owned(),
                                        "number choice (but american)".to_owned(),
                                    )])),
                                    value: 10.0,
                                },
                            )])),
                            description: "number desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Number,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "number name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "role desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::Role,
                            max_length: None,
                            max_value: None,
                            min_length: None,
                            min_value: None,
                            name: "role name".to_owned(),
                            name_localizations: Some(HashMap::from([(
                                "de-DE".to_owned(),
                                "role name (but german)".to_owned(),
                            )])),
                            options: None,
                            required: None,
                        },
                        CommandOption {
                            autocomplete: None,
                            channel_types: None,
                            choices: None,
                            description: "string desc".to_owned(),
                            description_localizations: None,
                            kind: CommandOptionType::String,
                            max_length: Some(6000),
                            max_value: None,
                            min_length: Some(0),
                            min_value: None,
                            name: "string name".to_owned(),
                            name_localizations: None,
                            options: None,
                            required: None,
                        },
                    ])),
                    required: None,
                }])),
                required: None,
            }]),
            version: Id::new(1),
        };

        assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Command",
                    len: 12,
                },
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("100"),
                Token::Str("default_member_permissions"),
                Token::Some,
                Token::Str("8"),
                Token::Str("dm_permission"),
                Token::Some,
                Token::Bool(false),
                Token::Str("description"),
                Token::Str("this command is a test"),
                Token::Str("description_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-US"),
                Token::Str("this command is a test"),
                Token::MapEnd,
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("300"),
                Token::Str("id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("200"),
                Token::Str("type"),
                Token::U8(CommandType::ChatInput.into()),
                Token::Str("name"),
                Token::Str("test command"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-US"),
                Token::Str("test command"),
                Token::MapEnd,
                Token::Str("options"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("sub command group desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::SubCommandGroup as u8),
                Token::Str("name"),
                Token::Str("sub command group name"),
                Token::Str("options"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("sub command desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::SubCommand as u8),
                Token::Str("name"),
                Token::Str("sub command name"),
                Token::Str("options"),
                Token::Some,
                Token::Seq { len: Some(9) },
                Token::Struct {
                    name: "CommandOption",
                    len: 3,
                },
                Token::Str("description"),
                Token::Str("attachment desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Attachment as u8),
                Token::Str("name"),
                Token::Str("attachment name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("boolean desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Boolean as u8),
                Token::Str("name"),
                Token::Str("boolean name"),
                Token::Str("required"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("channel_types"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("channel desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Channel as u8),
                Token::Str("name"),
                Token::Str("channel name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("channel_types"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::U8(ChannelType::GuildText.into()),
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("channel desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Channel as u8),
                Token::Str("name"),
                Token::Str("channel name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 7,
                },
                Token::Str("autocomplete"),
                Token::Some,
                Token::Bool(true),
                Token::Str("choices"),
                Token::Some,
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("integer desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Integer as u8),
                Token::Str("max_value"),
                Token::Some,
                Token::I64(100),
                Token::Str("min_value"),
                Token::Some,
                Token::I64(0),
                Token::Str("name"),
                Token::Str("integer name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("mentionable desc"),
                Token::Str("description_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-GB"),
                Token::Str("mentionable desc (but british)"),
                Token::MapEnd,
                Token::Str("type"),
                Token::U8(CommandOptionType::Mentionable as u8),
                Token::Str("name"),
                Token::Str("mentionable name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 5,
                },
                Token::Str("autocomplete"),
                Token::Some,
                Token::Bool(false),
                Token::Str("choices"),
                Token::Some,
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "CommandOptionChoiceData",
                    len: 3,
                },
                Token::Str("name"),
                Token::Str("number choice"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("en-US"),
                Token::Str("number choice (but american)"),
                Token::MapEnd,
                Token::Str("value"),
                Token::F64(10.0),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("description"),
                Token::Str("number desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Number as u8),
                Token::Str("name"),
                Token::Str("number name"),
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 4,
                },
                Token::Str("description"),
                Token::Str("role desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::Role as u8),
                Token::Str("name"),
                Token::Str("role name"),
                Token::Str("name_localizations"),
                Token::Some,
                Token::Map { len: Some(1) },
                Token::Str("de-DE"),
                Token::Str("role name (but german)"),
                Token::MapEnd,
                Token::StructEnd,
                Token::Struct {
                    name: "CommandOption",
                    len: 5,
                },
                Token::Str("description"),
                Token::Str("string desc"),
                Token::Str("type"),
                Token::U8(CommandOptionType::String as u8),
                Token::Str("max_length"),
                Token::Some,
                Token::U16(6000),
                Token::Str("min_length"),
                Token::Some,
                Token::U16(0),
                Token::Str("name"),
                Token::Str("string name"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("version"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::StructEnd,
            ],
        );
    }
}
