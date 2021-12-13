use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::Channel,
    id::{marker, Id},
};

/// Get a channel by its ID.
///
/// # Examples
///
/// Get channel `100`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let channel_id = Id::new(100).expect("non zero");
///
/// let channel = client.channel(channel_id).exec().await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetChannel<'a> {
    channel_id: Id<marker::Channel>,
    http: &'a Client,
}

impl<'a> GetChannel<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<marker::Channel>) -> Self {
        Self { channel_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Channel> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetChannel<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetChannel {
            channel_id: self.channel_id.get(),
        }))
    }
}
