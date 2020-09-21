use crate::request::prelude::*;
use twilight_model::id::{EmojiId, GuildId};

/// Delete an emoji in a guild, by id.
pub struct DeleteEmoji<'a> {
    emoji_id: EmojiId,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> DeleteEmoji<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, emoji_id: EmojiId) -> Self {
        Self {
            emoji_id,
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    #[deprecated(note = "you've used the request's reason method which is deprecated; \
                please import the request::AuditLogReason trait")]
    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                headers,
                Route::DeleteEmoji {
                    emoji_id: self.emoji_id.0,
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from(Route::DeleteEmoji {
                emoji_id: self.emoji_id.0,
                guild_id: self.guild_id.0,
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteEmoji<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        let reason = AuditLogReasonError::validate(reason.into())?;
        self.reason.replace(reason);

        Ok(self)
    }
}

poll_req!(DeleteEmoji<'_>, ());
