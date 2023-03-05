use super::{
    AfkTimeout, DefaultMessageNotificationLevel, Emoji, ExplicitContentFilter, GuildFeature,
    MfaLevel, NSFWLevel, Permissions, PremiumTier, Role, SystemChannelFlags, VerificationLevel,
};
use crate::{
    id::{
        marker::{ApplicationMarker, ChannelMarker, GuildMarker, UserMarker},
        Id,
    },
    util::image_hash::ImageHash,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(
    feature = "rkyv",
    derive(rkyv::Archive, rkyv::Deserialize, rkyv::Serialize)
)]
pub struct PartialGuild {
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub afk_channel_id: Option<Id<ChannelMarker>>,
    pub afk_timeout: AfkTimeout,
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub application_id: Option<Id<ApplicationMarker>>,
    pub banner: Option<ImageHash>,
    pub default_message_notifications: DefaultMessageNotificationLevel,
    pub description: Option<String>,
    pub discovery_splash: Option<ImageHash>,
    pub emojis: Vec<Emoji>,
    pub explicit_content_filter: ExplicitContentFilter,
    pub features: Vec<GuildFeature>,
    pub icon: Option<ImageHash>,
    pub id: Id<GuildMarker>,
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
    /// Whether the premium progress bar is enabled in the guild.
    pub premium_progress_bar_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u64>,
    pub premium_tier: PremiumTier,
    /// ID of the where moderators of Community guilds receive notices from
    /// Discord.
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub public_updates_channel_id: Option<Id<ChannelMarker>>,
    pub roles: Vec<Role>,
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub rules_channel_id: Option<Id<ChannelMarker>>,
    pub splash: Option<ImageHash>,
    pub system_channel_flags: SystemChannelFlags,
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub system_channel_id: Option<Id<ChannelMarker>>,
    pub verification_level: VerificationLevel,
    pub vanity_url_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "rkyv", with(crate::id::IdNiche))]
    pub widget_channel_id: Option<Id<ChannelMarker>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_enabled: Option<bool>,
}

#[cfg(test)]
mod tests {
    use crate::{
        guild::{AfkTimeout, GuildFeature},
        test::image_hash,
    };

    use super::{
        DefaultMessageNotificationLevel, ExplicitContentFilter, MfaLevel, NSFWLevel, PartialGuild,
        Permissions, PremiumTier, SystemChannelFlags, VerificationLevel,
    };
    use crate::id::Id;
    use serde_test::Token;

    #[allow(clippy::too_many_lines)]
    #[test]
    fn partial_guild() {
        let value = PartialGuild {
            afk_channel_id: Some(Id::new(2)),
            afk_timeout: AfkTimeout::FIFTEEN_MINUTES,
            application_id: Some(Id::new(3)),
            banner: Some(image_hash::BANNER),
            default_message_notifications: DefaultMessageNotificationLevel::Mentions,
            description: Some("a description".to_owned()),
            discovery_splash: Some(image_hash::SPLASH),
            emojis: Vec::new(),
            explicit_content_filter: ExplicitContentFilter::MembersWithoutRole,
            features: Vec::from([GuildFeature::AnimatedBanner]),
            icon: Some(image_hash::ICON),
            id: Id::new(1),
            max_members: Some(25_000),
            max_presences: Some(10_000),
            member_count: Some(12_000),
            mfa_level: MfaLevel::Elevated,
            name: "the name".to_owned(),
            nsfw_level: NSFWLevel::Default,
            owner_id: Id::new(5),
            owner: Some(false),
            permissions: Some(Permissions::SEND_MESSAGES),
            preferred_locale: "en-us".to_owned(),
            premium_progress_bar_enabled: true,
            premium_subscription_count: Some(3),
            premium_tier: PremiumTier::Tier1,
            public_updates_channel_id: None,
            roles: Vec::new(),
            rules_channel_id: Some(Id::new(6)),
            splash: Some(image_hash::SPLASH),
            system_channel_flags: SystemChannelFlags::SUPPRESS_PREMIUM_SUBSCRIPTIONS,
            system_channel_id: Some(Id::new(7)),
            verification_level: VerificationLevel::Medium,
            vanity_url_code: Some("twilight".to_owned()),
            widget_channel_id: Some(Id::new(8)),
            widget_enabled: Some(true),
        };

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "PartialGuild",
                    len: 35,
                },
                Token::Str("afk_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("2"),
                Token::Str("afk_timeout"),
                Token::NewtypeStruct { name: "AfkTimeout" },
                Token::U16(900),
                Token::Str("application_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("3"),
                Token::Str("banner"),
                Token::Some,
                Token::Str(image_hash::BANNER_INPUT),
                Token::Str("default_message_notifications"),
                Token::U8(1),
                Token::Str("description"),
                Token::Some,
                Token::Str("a description"),
                Token::Str("discovery_splash"),
                Token::Some,
                Token::Str(image_hash::SPLASH_INPUT),
                Token::Str("emojis"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("explicit_content_filter"),
                Token::U8(1),
                Token::Str("features"),
                Token::Seq { len: Some(1) },
                Token::Str("ANIMATED_BANNER"),
                Token::SeqEnd,
                Token::Str("icon"),
                Token::Some,
                Token::Str(image_hash::ICON_INPUT),
                Token::Str("id"),
                Token::NewtypeStruct { name: "Id" },
                Token::Str("1"),
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
                Token::Str("premium_progress_bar_enabled"),
                Token::Bool(true),
                Token::Str("premium_subscription_count"),
                Token::Some,
                Token::U64(3),
                Token::Str("premium_tier"),
                Token::U8(1),
                Token::Str("public_updates_channel_id"),
                Token::None,
                Token::Str("roles"),
                Token::Seq { len: Some(0) },
                Token::SeqEnd,
                Token::Str("rules_channel_id"),
                Token::Some,
                Token::NewtypeStruct { name: "Id" },
                Token::Str("6"),
                Token::Str("splash"),
                Token::Some,
                Token::Str(image_hash::SPLASH_INPUT),
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
