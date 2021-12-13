use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    guild::Emoji,
    id::{marker, Id},
};

/// Get an emoji for a guild by the the guild's ID and emoji's ID.
///
/// # Examples
///
/// Get emoji `100` from guild `50`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(50).expect("non zero");
/// let emoji_id = Id::new(100).expect("non zero");
///
/// client.emoji(guild_id, emoji_id).exec().await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetEmoji<'a> {
    emoji_id: Id<marker::Emoji>,
    guild_id: Id<marker::Guild>,
    http: &'a Client,
}

impl<'a> GetEmoji<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<marker::Guild>,
        emoji_id: Id<marker::Emoji>,
    ) -> Self {
        Self {
            emoji_id,
            guild_id,
            http,
        }
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

impl TryIntoRequest for GetEmoji<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetEmoji {
            emoji_id: self.emoji_id.get(),
            guild_id: self.guild_id.get(),
        }))
    }
}
