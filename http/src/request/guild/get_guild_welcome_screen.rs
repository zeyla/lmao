use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{id::GuildId, invite::WelcomeScreen};

/// Get the guild's welcome screen.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildWelcomeScreen<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildWelcomeScreen<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<WelcomeScreen> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetGuildWelcomeScreen<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildWelcomeScreen {
            guild_id: self.guild_id.get(),
        }))
    }
}
