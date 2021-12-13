//! Formatters for creating mentions.

use super::timestamp::Timestamp;
use std::fmt::{Display, Formatter, Result as FmtResult};
use twilight_model::{
    channel::{
        CategoryChannel, Channel, Group, GuildChannel, PrivateChannel, TextChannel, VoiceChannel,
    },
    guild::{Emoji, Member, Role},
    id::{marker, Id},
    user::{CurrentUser, User},
};

/// Formatter to mention a resource that implements `std::fmt::Display`.
///
/// # Examples
///
/// Mention a `Id<marker::User>`:
///
/// ```rust
/// use twilight_mention::Mention;
/// use twilight_model::id::{marker, Id};
///
/// assert_eq!("<@123>", Id::<marker::User>::new(123).expect("non zero").mention().to_string());
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MentionFormat<T>(T);

/// Mention a channel. This will format as `<#ID>`.
impl Display for MentionFormat<Id<marker::Channel>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<#")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Display for MentionFormat<Id<marker::Emoji>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<:emoji:")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention a role. This will format as `<@&ID>`.
impl Display for MentionFormat<Id<marker::Role>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<@&")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention a user. This will format as `<t:UNIX>` if a style is not specified or
/// `<t:UNIX:STYLE>` if a style is specified.
impl Display for MentionFormat<Timestamp> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<t:")?;
        Display::fmt(&self.0.unix(), f)?;

        if let Some(style) = self.0.style() {
            f.write_str(":")?;
            Display::fmt(&style, f)?;
        }

        f.write_str(">")
    }
}

/// Mention a user. This will format as `<@ID>`.
impl Display for MentionFormat<Id<marker::User>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("<@")?;
        Display::fmt(&self.0, f)?;

        f.write_str(">")
    }
}

/// Mention a resource, such as an emoji or user.
///
/// This will create a mention that will link to a user if it exists.
///
/// Look at the implementations list to see what you can mention.
///
/// # Examples
///
/// Mention a channel ID:
///
/// ```rust
/// use twilight_mention::Mention;
/// use twilight_model::id::{marker, Id};
///
/// let id = Id::<marker::Channel>::new(123).expect("non zero");
/// assert_eq!("<#123>", id.mention().to_string());
/// ```
pub trait Mention<T> {
    /// Mention a resource by using its ID.
    fn mention(&self) -> MentionFormat<T>;
}

impl<T, M: Mention<T>> Mention<T> for &'_ M {
    fn mention(&self) -> MentionFormat<T> {
        (*self).mention()
    }
}

/// Mention a channel ID. This will format as `<#ID>`.
impl Mention<Id<marker::Channel>> for Id<marker::Channel> {
    fn mention(&self) -> MentionFormat<Id<marker::Channel>> {
        MentionFormat(*self)
    }
}

/// Mention a guild category channel. This will format as `<#ID>`.
impl Mention<Id<marker::Channel>> for CategoryChannel {
    fn mention(&self) -> MentionFormat<Id<marker::Channel>> {
        MentionFormat(self.id)
    }
}

/// Mention a channel. This will format as `<#ID>`.
impl Mention<Id<marker::Channel>> for Channel {
    fn mention(&self) -> MentionFormat<Id<marker::Channel>> {
        MentionFormat(self.id())
    }
}

/// Mention the current user. This will format as `<@ID>`.
impl Mention<Id<marker::User>> for CurrentUser {
    fn mention(&self) -> MentionFormat<Id<marker::User>> {
        MentionFormat(self.id)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<Id<marker::Emoji>> for Id<marker::Emoji> {
    fn mention(&self) -> MentionFormat<Id<marker::Emoji>> {
        MentionFormat(*self)
    }
}

/// Mention an emoji. This will format as `<:emoji:ID>`.
impl Mention<Id<marker::Emoji>> for Emoji {
    fn mention(&self) -> MentionFormat<Id<marker::Emoji>> {
        MentionFormat(self.id)
    }
}

/// Mention a group. This will format as `<#ID>`.
impl Mention<Id<marker::Channel>> for Group {
    fn mention(&self) -> MentionFormat<Id<marker::Channel>> {
        MentionFormat(self.id)
    }
}

/// Mention a guild channel. This will format as `<#ID>`.
impl Mention<Id<marker::Channel>> for GuildChannel {
    fn mention(&self) -> MentionFormat<Id<marker::Channel>> {
        MentionFormat(self.id())
    }
}

/// Mention a member's user. This will format as `<@ID>`.
impl Mention<Id<marker::User>> for Member {
    fn mention(&self) -> MentionFormat<Id<marker::User>> {
        MentionFormat(self.user.id)
    }
}

/// Mention a private channel. This will format as `<#ID>`.
impl Mention<Id<marker::Channel>> for PrivateChannel {
    fn mention(&self) -> MentionFormat<Id<marker::Channel>> {
        MentionFormat(self.id)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<Id<marker::Role>> for Id<marker::Role> {
    fn mention(&self) -> MentionFormat<Id<marker::Role>> {
        MentionFormat(*self)
    }
}

/// Mention a role ID. This will format as `<@&ID>`.
impl Mention<Id<marker::Role>> for Role {
    fn mention(&self) -> MentionFormat<Id<marker::Role>> {
        MentionFormat(self.id)
    }
}

/// Mention a guild text channel. This will format as `<#ID>`.
impl Mention<Id<marker::Channel>> for TextChannel {
    fn mention(&self) -> MentionFormat<Id<marker::Channel>> {
        MentionFormat(self.id)
    }
}

/// Mention a timestamp. This will format as `<t:UNIX>` if a style is not
/// specified or `<t:UNIX:STYLE>` if a style is specified.
impl Mention<Self> for Timestamp {
    fn mention(&self) -> MentionFormat<Self> {
        MentionFormat(*self)
    }
}

/// Mention a user ID. This will format as `<&ID>`.
impl Mention<Id<marker::User>> for Id<marker::User> {
    fn mention(&self) -> MentionFormat<Id<marker::User>> {
        MentionFormat(*self)
    }
}

/// Mention a user. This will format as `<&ID>`.
impl Mention<Id<marker::User>> for User {
    fn mention(&self) -> MentionFormat<Id<marker::User>> {
        MentionFormat(self.id)
    }
}

/// Mention a guild voice channel. This will format as `<#ID>`.
impl Mention<Id<marker::Channel>> for VoiceChannel {
    fn mention(&self) -> MentionFormat<Id<marker::Channel>> {
        MentionFormat(self.id)
    }
}

#[cfg(test)]
mod tests {
    use crate::timestamp::{Timestamp, TimestampStyle};

    use super::{Mention, MentionFormat};
    use static_assertions::assert_impl_all;
    use std::fmt::{Debug, Display};
    use twilight_model::{
        channel::{
            CategoryChannel, Channel, Group, GuildChannel, PrivateChannel, TextChannel,
            VoiceChannel,
        },
        guild::{Emoji, Member, Role},
        id::{marker, Id},
        user::{CurrentUser, User},
    };

    assert_impl_all!(MentionFormat<()>: Clone, Copy, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<Id<marker::Channel>>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<Id<marker::Emoji>>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<Id<marker::Role>>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MentionFormat<Id<marker::User>>: Clone, Copy, Debug, Display, Eq, PartialEq, Send, Sync);
    assert_impl_all!(Id<marker::Channel>: Mention<Id<marker::Channel>>);
    assert_impl_all!(&'static Id<marker::Channel>: Mention<Id<marker::Channel>>);
    assert_impl_all!(CategoryChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(&'static CategoryChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(Channel: Mention<Id<marker::Channel>>);
    assert_impl_all!(&'static Channel: Mention<Id<marker::Channel>>);
    assert_impl_all!(CurrentUser: Mention<Id<marker::User>>);
    assert_impl_all!(&'static CurrentUser: Mention<Id<marker::User>>);
    assert_impl_all!(Id<marker::Emoji>: Mention<Id<marker::Emoji>>);
    assert_impl_all!(&'static Id<marker::Emoji>: Mention<Id<marker::Emoji>>);
    assert_impl_all!(Emoji: Mention<Id<marker::Emoji>>);
    assert_impl_all!(&'static Emoji: Mention<Id<marker::Emoji>>);
    assert_impl_all!(Group: Mention<Id<marker::Channel>>);
    assert_impl_all!(&'static Group: Mention<Id<marker::Channel>>);
    assert_impl_all!(GuildChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(&'static GuildChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(Member: Mention<Id<marker::User>>);
    assert_impl_all!(&'static Member: Mention<Id<marker::User>>);
    assert_impl_all!(PrivateChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(&'static PrivateChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(Id<marker::Role>: Mention<Id<marker::Role>>);
    assert_impl_all!(&'static Id<marker::Role>: Mention<Id<marker::Role>>);
    assert_impl_all!(Role: Mention<Id<marker::Role>>);
    assert_impl_all!(&'static Role: Mention<Id<marker::Role>>);
    assert_impl_all!(TextChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(&'static TextChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(Id<marker::User>: Mention<Id<marker::User>>);
    assert_impl_all!(&'static Id<marker::User>: Mention<Id<marker::User>>);
    assert_impl_all!(User: Mention<Id<marker::User>>);
    assert_impl_all!(&'static User: Mention<Id<marker::User>>);
    assert_impl_all!(VoiceChannel: Mention<Id<marker::Channel>>);
    assert_impl_all!(&'static VoiceChannel: Mention<Id<marker::Channel>>);

    #[test]
    fn test_mention_format_channel_id() {
        assert_eq!(
            "<#123>",
            Id::<marker::Channel>::new(123)
                .expect("non zero")
                .mention()
                .to_string()
        );
    }

    #[test]
    fn test_mention_format_emoji_id() {
        assert_eq!(
            "<:emoji:123>",
            Id::<marker::Emoji>::new(123)
                .expect("non zero")
                .mention()
                .to_string()
        );
    }

    #[test]
    fn test_mention_format_role_id() {
        assert_eq!(
            "<@&123>",
            Id::<marker::Role>::new(123)
                .expect("non zero")
                .mention()
                .to_string()
        );
    }

    /// Test that a timestamp with a style displays correctly.
    #[test]
    fn test_mention_format_timestamp_styled() {
        let timestamp = Timestamp::new(1_624_047_064, Some(TimestampStyle::RelativeTime));

        assert_eq!("<t:1624047064:R>", timestamp.mention().to_string());
    }

    /// Test that a timestamp without a style displays correctly.
    #[test]
    fn test_mention_format_timestamp_unstyled() {
        let timestamp = Timestamp::new(1_624_047_064, None);

        assert_eq!("<t:1624047064>", timestamp.mention().to_string());
    }

    #[test]
    fn test_mention_format_user_id() {
        assert_eq!(
            "<@123>",
            Id::<marker::User>::new(123)
                .expect("non zero")
                .mention()
                .to_string()
        );
    }
}
