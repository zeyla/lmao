use crate::{
    channel::{thread::ThreadMetadata, ChannelType, Message},
    datetime::Timestamp,
    guild::{Permissions, Role},
    id::{ChannelId, MessageId, RoleId, UserId},
    user::User,
};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::HashMap;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CommandInteractionDataResolved {
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub channels: HashMap<ChannelId, InteractionChannel>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub members: HashMap<UserId, InteractionMember>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub messages: HashMap<MessageId, Message>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub roles: HashMap<RoleId, Role>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub users: HashMap<UserId, User>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InteractionChannel {
    pub id: ChannelId,
    #[serde(rename = "type")]
    pub kind: ChannelType,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,
    pub permissions: Permissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_metadata: Option<ThreadMetadata>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct InteractionMember {
    pub joined_at: Timestamp,
    pub nick: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,
    #[serde(default)]
    pub roles: Vec<RoleId>,
}

#[cfg(test)]
mod tests {
    use super::{CommandInteractionDataResolved, InteractionChannel, InteractionMember};
    use crate::{
        channel::{
            message::{
                sticker::{MessageSticker, StickerFormatType, StickerId},
                MessageFlags, MessageType,
            },
            ChannelType, Message,
        },
        datetime::{Timestamp, TimestampParseError},
        guild::{PartialMember, Permissions, Role},
        id::{ChannelId, GuildId, MessageId, RoleId, UserId},
        user::{PremiumType, User, UserFlags},
    };
    use serde_test::Token;
    use std::str::FromStr;

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_data_resolved() -> Result<(), TimestampParseError> {
        let joined_at = Timestamp::from_str("2021-08-10T12:18:37.000000+00:00")?;
        let timestamp = Timestamp::from_str("2020-02-02T02:02:02.020000+00:00")?;

        let value = CommandInteractionDataResolved {
            channels: IntoIterator::into_iter([(
                ChannelId::new(100).expect("non zero"),
                InteractionChannel {
                    id: ChannelId::new(100).expect("non zero"),
                    kind: ChannelType::GuildText,
                    name: "channel name".into(),
                    parent_id: None,
                    permissions: Permissions::empty(),
                    thread_metadata: None,
                },
            )])
            .collect(),
            members: IntoIterator::into_iter([(
                UserId::new(300).expect("non zero"),
                InteractionMember {
                    joined_at,
                    nick: None,
                    premium_since: None,
                    roles: Vec::new(),
                },
            )])
            .collect(),
            messages: IntoIterator::into_iter([(
                MessageId::new(4).expect("non zero"),
                Message {
                    activity: None,
                    application: None,
                    application_id: None,
                    attachments: Vec::new(),
                    author: User {
                        accent_color: None,
                        avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                        banner: None,
                        bot: false,
                        discriminator: 1,
                        email: None,
                        flags: None,
                        id: UserId::new(3).expect("non zero"),
                        locale: None,
                        mfa_enabled: None,
                        name: "test".to_owned(),
                        premium_type: None,
                        public_flags: None,
                        system: None,
                        verified: None,
                    },
                    channel_id: ChannelId::new(2).expect("non zero"),
                    components: Vec::new(),
                    content: "ping".to_owned(),
                    edited_timestamp: None,
                    embeds: Vec::new(),
                    flags: Some(MessageFlags::empty()),
                    guild_id: Some(GuildId::new(1).expect("non zero")),
                    id: MessageId::new(4).expect("non zero"),
                    interaction: None,
                    kind: MessageType::Regular,
                    member: Some(PartialMember {
                        avatar: None,
                        deaf: false,
                        joined_at,
                        mute: false,
                        nick: Some("member nick".to_owned()),
                        permissions: None,
                        premium_since: None,
                        roles: Vec::new(),
                        user: None,
                    }),
                    mention_channels: Vec::new(),
                    mention_everyone: false,
                    mention_roles: Vec::new(),
                    mentions: Vec::new(),
                    pinned: false,
                    reactions: Vec::new(),
                    reference: None,
                    sticker_items: vec![MessageSticker {
                        format_type: StickerFormatType::Png,
                        id: StickerId::new(1).expect("non zero"),
                        name: "sticker name".to_owned(),
                    }],
                    referenced_message: None,
                    thread: None,
                    timestamp,
                    tts: false,
                    webhook_id: None,
                },
            )])
            .collect(),
            roles: IntoIterator::into_iter([(
                RoleId::new(400).expect("non zero"),
                Role {
                    color: 0,
                    hoist: true,
                    icon: None,
                    id: RoleId::new(400).expect("non zero"),
                    managed: false,
                    mentionable: true,
                    name: "test".to_owned(),
                    permissions: Permissions::ADMINISTRATOR,
                    position: 12,
                    tags: None,
                    unicode_emoji: None,
                },
            )])
            .collect(),
            users: IntoIterator::into_iter([(
                UserId::new(300).expect("non zero"),
                User {
                    accent_color: None,
                    avatar: Some("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned()),
                    banner: None,
                    bot: false,
                    discriminator: 1,
                    email: Some("address@example.com".to_owned()),
                    flags: Some(UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER),
                    id: UserId::new(300).expect("non zero"),
                    locale: Some("en-us".to_owned()),
                    mfa_enabled: Some(true),
                    name: "test".to_owned(),
                    premium_type: Some(PremiumType::Nitro),
                    public_flags: Some(
                        UserFlags::PREMIUM_EARLY_SUPPORTER | UserFlags::VERIFIED_DEVELOPER,
                    ),
                    system: None,
                    verified: Some(true),
                },
            )])
            .collect(),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "CommandInteractionDataResolved",
                    len: 5,
                },
                Token::Str("channels"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("100"),
                Token::Struct {
                    name: "InteractionChannel",
                    len: 4,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("100"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("name"),
                Token::Str("channel name"),
                Token::Str("permissions"),
                Token::Str("0"),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("members"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("300"),
                Token::Struct {
                    name: "InteractionMember",
                    len: 3,
                },
                Token::Str("joined_at"),
                Token::Str("2021-08-10T12:18:37.000000+00:00"),
                Token::Str("nick"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("messages"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("4"),
                Token::Struct {
                    name: "Message",
                    len: 18,
                },
                Token::Str("attachments"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("author"),
                Token::Struct {
                    name: "User",
                    len: 7,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("3"),
                Token::Str("username"),
                Token::Str("test"),
                Token::StructEnd,
                Token::Str("channel_id"),
                Token::NewtypeStruct { name: "ChannelId" },
                Token::Str("2"),
                Token::Str("content"),
                Token::Str("ping"),
                Token::Str("edited_timestamp"),
                Token::None,
                Token::Str("embeds"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("flags"),
                Token::Some,
                Token::U64(0),
                Token::Str("guild_id"),
                Token::Some,
                Token::NewtypeStruct { name: "GuildId" },
                Token::Str("1"),
                Token::Str("id"),
                Token::NewtypeStruct { name: "MessageId" },
                Token::Str("4"),
                Token::Str("type"),
                Token::U8(0),
                Token::Str("member"),
                Token::Some,
                Token::Struct {
                    name: "PartialMember",
                    len: 7,
                },
                Token::Str("deaf"),
                Token::Bool(false),
                Token::Str("joined_at"),
                Token::Str("2021-08-10T12:18:37.000000+00:00"),
                Token::Str("mute"),
                Token::Bool(false),
                Token::Str("nick"),
                Token::Some,
                Token::Str("member nick"),
                Token::Str("permissions"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("user"),
                Token::None,
                Token::StructEnd,
                Token::Str("mention_everyone"),
                Token::Bool(false),
                Token::Str("mention_roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("mentions"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("pinned"),
                Token::Bool(false),
                Token::Str("sticker_items"),
                Token::Seq { len: Some(1) },
                Token::Struct {
                    name: "MessageSticker",
                    len: 3,
                },
                Token::Str("format_type"),
                Token::U8(1),
                Token::Str("id"),
                Token::NewtypeStruct { name: "StickerId" },
                Token::Str("1"),
                Token::Str("name"),
                Token::Str("sticker name"),
                Token::StructEnd,
                Token::SeqEnd,
                Token::Str("timestamp"),
                Token::Str("2020-02-02T02:02:02.020000+00:00"),
                Token::Str("tts"),
                Token::Bool(false),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("roles"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("400"),
                Token::Struct {
                    name: "Role",
                    len: 8,
                },
                Token::Str("color"),
                Token::U32(0),
                Token::Str("hoist"),
                Token::Bool(true),
                Token::Str("id"),
                Token::NewtypeStruct { name: "RoleId" },
                Token::Str("400"),
                Token::Str("managed"),
                Token::Bool(false),
                Token::Str("mentionable"),
                Token::Bool(true),
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("permissions"),
                Token::Str("8"),
                Token::Str("position"),
                Token::I64(12),
                Token::StructEnd,
                Token::MapEnd,
                Token::Str("users"),
                Token::Map { len: Some(1) },
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("300"),
                Token::Struct {
                    name: "User",
                    len: 14,
                },
                Token::Str("accent_color"),
                Token::None,
                Token::Str("avatar"),
                Token::Some,
                Token::Str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
                Token::Str("banner"),
                Token::None,
                Token::Str("bot"),
                Token::Bool(false),
                Token::Str("discriminator"),
                Token::Str("0001"),
                Token::Str("email"),
                Token::Some,
                Token::Str("address@example.com"),
                Token::Str("flags"),
                Token::Some,
                Token::U64(131_584),
                Token::Str("id"),
                Token::NewtypeStruct { name: "UserId" },
                Token::Str("300"),
                Token::Str("locale"),
                Token::Some,
                Token::Str("en-us"),
                Token::Str("mfa_enabled"),
                Token::Some,
                Token::Bool(true),
                Token::Str("username"),
                Token::Str("test"),
                Token::Str("premium_type"),
                Token::Some,
                Token::U8(2),
                Token::Str("public_flags"),
                Token::Some,
                Token::U64(131_584),
                Token::Str("verified"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
                Token::MapEnd,
                Token::StructEnd,
            ],
        );

        Ok(())
    }
}
