use crate::json_to_vec;
use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::id::GuildId;

#[derive(Serialize)]
struct UpdateCurrentUserNickFields<'a> {
    nick: Cow<'a, str>,
}

/// Changes the user's nickname in a guild.
pub struct UpdateCurrentUserNick<'a> {
    fields: UpdateCurrentUserNickFields<'a>,
    fut: Option<Pending<'a, ()>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> UpdateCurrentUserNick<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, nick: impl Into<Cow<'a, str>>) -> Self {
        Self {
            fields: UpdateCurrentUserNickFields { nick: nick.into() },
            fut: None,
            guild_id,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            json_to_vec(&self.fields)?,
            Route::UpdateNickname {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateCurrentUserNick<'_>, ());
