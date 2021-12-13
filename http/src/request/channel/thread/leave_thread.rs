use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{marker, Id};

/// Remove the current user from a thread.
///
/// Requires that the thread is not archived.
#[must_use = "requests must be configured and executed"]
pub struct LeaveThread<'a> {
    channel_id: Id<marker::Channel>,
    http: &'a Client,
}

impl<'a> LeaveThread<'a> {
    pub(crate) const fn new(http: &'a Client, channel_id: Id<marker::Channel>) -> Self {
        Self { channel_id, http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for LeaveThread<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::LeaveThread {
            channel_id: self.channel_id.get(),
        }))
    }
}
