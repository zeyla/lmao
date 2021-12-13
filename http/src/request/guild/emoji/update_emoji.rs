use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    guild::Emoji,
    id::{marker, Id},
};

#[derive(Serialize)]
struct UpdateEmojiFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roles: Option<&'a [Id<marker::Role>]>,
}

/// Update an emoji in a guild, by id.
#[must_use = "requests must be configured and executed"]
pub struct UpdateEmoji<'a> {
    emoji_id: Id<marker::Emoji>,
    fields: UpdateEmojiFields<'a>,
    guild_id: Id<marker::Guild>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> UpdateEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<marker::Guild>,
        emoji_id: Id<marker::Emoji>,
    ) -> Self {
        Self {
            fields: UpdateEmojiFields {
                name: None,
                roles: None,
            },
            emoji_id,
            guild_id,
            http,
            reason: None,
        }
    }

    /// Change the name of the emoji.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.fields.name = Some(name);

        self
    }

    /// Change the roles that the emoji is whitelisted to.
    pub const fn roles(mut self, roles: &'a [Id<marker::Role>]) -> Self {
        self.fields.roles = Some(roles);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Emoji> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateEmoji<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateEmoji {
            emoji_id: self.emoji_id.get(),
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}
