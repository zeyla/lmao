use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{id::GuildId, voice::VoiceRegion};

/// Get voice region data for the guild.
///
/// Can return VIP servers if the guild is VIP-enabled.
#[must_use = "requests must be configured and executed"]
pub struct GetGuildVoiceRegions<'a> {
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildVoiceRegions<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self { guild_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<VoiceRegion>> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetGuildVoiceRegions<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetGuildVoiceRegions {
            guild_id: self.guild_id.get(),
        }))
    }
}
