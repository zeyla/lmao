//! Provides the Snowflake trait for defining extractable information from a Discord Snowflake.

use twilight_model::id::{
    marker::{
        ApplicationMarker, AttachmentMarker, AuditLogEntryMarker, ChannelMarker, CommandMarker,
        CommandVersionMarker, EmojiMarker, GenericMarker, GuildMarker, IntegrationMarker,
        InteractionMarker, MessageMarker, RoleMarker, StageMarker, UserMarker, WebhookMarker,
    },
    Id,
};

/// Snowflake is a trait for defining extractable information from a Snowflake. A Snowflake is a
/// u64 generated by Discord to uniquely identify a resource.
pub trait Snowflake {
    /// Returns the u64 backing the Snowflake.
    fn id(&self) -> u64;

    /// The Unix epoch of the Snowflake in milliseconds, indicating when it was generated.
    ///
    /// Derived from bits 22..63 of the id.
    ///
    /// # Examples
    ///
    /// See when a user was created using [`chrono`](https://docs.rs/chrono):
    ///
    /// ```
    /// use chrono::{Utc, TimeZone};
    /// use twilight_util::snowflake::Snowflake;
    /// use twilight_model::id::{marker::UserMarker, Id};
    ///
    /// let id = Id::<UserMarker>::new(105484726235607040);
    ///
    /// assert_eq!(
    ///     "2015-10-19T01:58:38.546+00:00",
    ///     Utc.timestamp_millis(id.timestamp()).to_rfc3339()
    /// );
    /// ```
    ///
    /// See when a user was created using [`time`](https://docs.rs/time):
    ///
    /// ```
    /// use time::{Duration, format_description::well_known::Rfc3339, OffsetDateTime};
    /// use twilight_util::snowflake::Snowflake;
    /// use twilight_model::id::{marker::UserMarker, Id};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let id = Id::<UserMarker>::new(105484726235607040);
    /// // Convert milliseconds to seconds or nanoseconds.
    /// let dur = Duration::milliseconds(id.timestamp());
    ///
    /// let ts = OffsetDateTime::from_unix_timestamp(dur.whole_seconds())?;
    /// let ts_milli = OffsetDateTime::from_unix_timestamp_nanos(dur.whole_nanoseconds())?;
    ///
    /// assert_eq!("2015-10-19T01:58:38Z", ts.format(&Rfc3339)?);
    /// assert_eq!("2015-10-19T01:58:38.546Z", ts_milli.format(&Rfc3339)?);
    /// # Ok(()) }
    /// ```
    #[allow(clippy::cast_possible_wrap)]
    fn timestamp(&self) -> i64 {
        // Discord's custom epoch, the unix time in milliseconds for the first second of 2015.
        const DISCORD_EPOCH: u64 = 1_420_070_400_000;

        ((self.id() >> 22) + DISCORD_EPOCH) as i64
    }

    /// The id of the internal worker that generated the Snowflake.
    ///
    /// Derived from bits 17..21 of the id.
    #[allow(clippy::cast_possible_truncation)]
    fn worker_id(&self) -> u8 {
        ((self.id() & 0x003E_0000) >> 17) as u8
    }

    /// The id of the internal process that generated the Snowflake.
    ///
    /// Derived from bits 12..16 of the id.
    #[allow(clippy::cast_possible_truncation)]
    fn process_id(&self) -> u8 {
        ((self.id() & 0x1F000) >> 12) as u8
    }

    /// The increment of the Snowflake. For every id that is generated on a process, this number is
    /// incremented.
    ///
    /// Derived from bits 0..11 of the id.
    #[allow(clippy::cast_possible_truncation)]
    fn increment(&self) -> u16 {
        (self.id() & 0xFFF) as u16
    }
}

impl Snowflake for Id<ApplicationMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<AttachmentMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<AuditLogEntryMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<ChannelMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<CommandMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<CommandVersionMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<EmojiMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<GenericMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<GuildMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<IntegrationMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<InteractionMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<MessageMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<RoleMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<StageMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<UserMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

impl Snowflake for Id<WebhookMarker> {
    fn id(&self) -> u64 {
        self.get()
    }
}

#[cfg(test)]
mod tests {
    use super::Snowflake;
    use static_assertions::{assert_impl_all, assert_obj_safe};
    use twilight_model::id::{
        marker::{
            ApplicationMarker, AttachmentMarker, AuditLogEntryMarker, ChannelMarker, CommandMarker,
            CommandVersionMarker, EmojiMarker, GenericMarker, GuildMarker, IntegrationMarker,
            InteractionMarker, MessageMarker, RoleMarker, StageMarker, UserMarker, WebhookMarker,
        },
        Id,
    };

    assert_impl_all!(Id<ApplicationMarker>: Snowflake);
    assert_impl_all!(Id<AttachmentMarker>: Snowflake);
    assert_impl_all!(Id<AuditLogEntryMarker>: Snowflake);
    assert_impl_all!(Id<ChannelMarker>: Snowflake);
    assert_impl_all!(Id<CommandMarker>: Snowflake);
    assert_impl_all!(Id<CommandVersionMarker>: Snowflake);
    assert_impl_all!(Id<EmojiMarker>: Snowflake);
    assert_impl_all!(Id<GenericMarker>: Snowflake);
    assert_impl_all!(Id<GuildMarker>: Snowflake);
    assert_impl_all!(Id<IntegrationMarker>: Snowflake);
    assert_impl_all!(Id<InteractionMarker>: Snowflake);
    assert_impl_all!(Id<MessageMarker>: Snowflake);
    assert_impl_all!(Id<RoleMarker>: Snowflake);
    assert_impl_all!(Id<StageMarker>: Snowflake);
    assert_impl_all!(Id<UserMarker>: Snowflake);
    assert_impl_all!(Id<WebhookMarker>: Snowflake);
    assert_obj_safe!(Snowflake);

    #[test]
    fn test_timestamp() {
        let expected: i64 = 1_445_219_918_546;
        let id = Id::<GenericMarker>::new(105_484_726_235_607_040);

        assert_eq!(expected, id.timestamp())
    }

    #[test]
    fn test_worker_id() {
        let expected: u8 = 8;
        let id = Id::<GenericMarker>::new(762_022_344_856_174_632);

        assert_eq!(expected, id.worker_id())
    }

    #[test]
    fn test_process_id() {
        let expected: u8 = 1;
        let id = Id::<GenericMarker>::new(61_189_081_970_774_016);

        assert_eq!(expected, id.process_id())
    }

    #[test]
    fn test_increment() {
        let expected: u16 = 40;
        let id = Id::<GenericMarker>::new(762_022_344_856_174_632);

        assert_eq!(expected, id.increment())
    }
}
