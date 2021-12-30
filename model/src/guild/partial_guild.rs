use crate::{
    guild::{
        DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, MfaLevel, NSFWLevel,
        Permissions, PremiumTier, Role, SystemChannelFlags, VerificationLevel,
    },
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PartialGuild {
    pub id: Id<GuildMarker>,
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: u64,
    pub application_id: Option<Id<ApplicationMarker>>,
    pub banner: Option<String>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<String>,
    pub emojis: Vec<Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<String>,
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_members: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_presences: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u64>,
    pub mfa_level: MfaLevel,
    pub name: String,
    pub nsfw_level: NSFWLevel,
    pub owner_id: Id<UserMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    pub preferred_locale: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: PremiumTier,
    pub roles: Vec<Role>,
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    pub splash: Option<String>,
    pub system_channel_flags: SystemChannelFlags,
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub verification_level: VerificationLevel,
    pub vanity_url_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_enabled: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, NSFWLevel, PartialGuild,
        Permissions, PremiumTier, SystemChannelFlags, VerificationLevel,
    };
    use crate::id::Id;
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_partial_guild() {
        let value = PartialGuild {
            id: Id::new_checked(1),
            afk_channel_id: Some(Id::new_checked(2)),
            afk_timeout: 900,
            application_id: Some(Id::new_checked(3)),
            banner: Some("banner hash".to_owned()),
            default_message_notifications: DefaultMessageNotificationLevel::Mentions,
            description: Some("a description".to_owned()),
            discovery_splash: Some("discovery splash hash".to_owned()),
            emojis: Vec::new(),
            explicit_content_filter: ExplicitContentFilter::MembersWithoutRole,
            features: vec!["a feature".to_owned()],
            icon: Some("icon hash".to_owned()),
            max_members: Some(25_000),
            max_presences: Some(10_000),
            member_count: Some(12_000),
            mfa_level: MfaLevel::Elevated,
            name: "the name".to_owned(),
            nsfw_level: NSFWLevel::Default,
            owner_id: Id::new_checked(5),
            owner: Some(false),
            permissions: Some(Permissions::SEND_MESSAGES),
            preferred_locale: "en-us".to_owned(),
            premium_subscription_count: Some(3),
            premium_tier: PremiumTier::Tier1,
            roles: Vec::new(),
            rules_channel_id: Some(Id::new_checked(6)),
            splash: Some("splash hash".to_owned()),
            system_channel_flags: SystemChannelFlags::SUPPRESS_PREMIUM_SUBSCRIPTIONS,
            system_channel_id: Some(Id::new_checked(7)),
            verification_level: VerificationLevel::Medium,
            vanity_url_code: Some("twilight".to_owned()),
            widget_channel_id: Some(Id::new_checked(8)),
            widget_enabled: Some(true),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PartialGuild",
                    len: 33,
                },
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
                Token::Str("afk_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("afk_timeout"),
                Token::U64(900),
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("banner"),
                Token::Some,
                Token::Str("banner hash"),
                Token::Str("default_message_notifications"),
                Token::U8(1),
                Token::Str("description"),
                Token::Some,
                Token::Str("a description"),
                Token::Str("discovery_splash"),
                Token::Some,
                Token::Str("discovery splash hash"),
                Token::Str("emojis"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("explicit_content_filter"),
                Token::U8(1),
                Token::Str("features"),
                Token::Seq { len: Some(1) },
                Token::Str("a feature"),
                Token::SeqEnd,
                Token::Str("icon"),
                Token::Some,
                Token::Str("icon hash"),
                Token::Str("max_members"),
                Token::Some,
                Token::U64(25_000),
                Token::Str("max_presences"),
                Token::Some,
                Token::U64(10_000),
                Token::Str("member_count"),
                Token::Some,
                Token::U64(12_000),
                Token::Str("mfa_level"),
                Token::U8(1),
                Token::Str("name"),
                Token::Str("the name"),
                Token::Str("nsfw_level"),
                Token::U8(0),
                Token::Str("owner_id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("5"),
                Token::Str("owner"),
                Token::Some,
                Token::Bool(false),
                Token::Str("permissions"),
                Token::Some,
                Token::Str("2048"),
                Token::Str("preferred_locale"),
                Token::Str("en-us"),
                Token::Str("premium_subscription_count"),
                Token::Some,
                Token::U64(3),
                Token::Str("premium_tier"),
                Token::U8(1),
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("rules_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("splash"),
                Token::Some,
                Token::Str("splash hash"),
                Token::Str("system_channel_flags"),
                Token::U64(2),
                Token::Str("system_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("7"),
                Token::Str("verification_level"),
                Token::U8(2),
                Token::Str("vanity_url_code"),
                Token::Some,
                Token::Str("twilight"),
                Token::Str("widget_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("8"),
                Token::Str("widget_enabled"),
                Token::Some,
                Token::Bool(true),
                Token::StructEnd,
            ],
        );
    }
}
