use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    application::command::Command,
    id::{ApplicationId, GuildId},
};

/// Fetch all commands for a guild, by ID.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildCommands<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildCommands<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
    ) -> Self {
        Self {
            application_id,
            guild_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<Command>> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetGuildCommands<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildCommands {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
        }))
    }
}
