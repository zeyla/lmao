use super::RequestReactionType;
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{marker, Id};

/// Remove all reactions of a specified emoji from a message.
#[must_use = "requests must be configured and executed"]
pub struct DeleteAllReaction<'a> {
    channel_id: Id<marker::Channel>,
    emoji: &'a RequestReactionType<'a>,
    http: &'a Client,
    message_id: Id<marker::Message>,
}

impl<'a> DeleteAllReaction<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<marker::Channel>,
        message_id: Id<marker::Message>,
        emoji: &'a RequestReactionType<'a>,
    ) -> Self {
        Self {
            channel_id,
            emoji,
            http,
            message_id,
        }
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

impl TryIntoRequest for DeleteAllReaction<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::DeleteMessageSpecificReaction {
            channel_id: self.channel_id.get(),
            message_id: self.message_id.get(),
            emoji: self.emoji,
        }))
    }
}
