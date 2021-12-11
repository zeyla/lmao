use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{ApplicationId, MessageId};

/// Delete a followup message created from a interaction.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::{MessageId, ApplicationId};
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// client
///     .delete_followup_message("token here", MessageId::new(2).expect("non zero"))?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct DeleteFollowupMessage<'a> {
    http: &'a Client,
    message_id: MessageId,
    token: &'a str,
    application_id: ApplicationId,
}

impl<'a> DeleteFollowupMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        token: &'a str,
        message_id: MessageId,
    ) -> Self {
        Self {
            http,
            message_id,
            token,
            application_id,
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

impl TryIntoRequest for DeleteFollowupMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::builder(&Route::DeleteWebhookMessage {
            message_id: self.message_id.get(),
            thread_id: None,
            token: self.token,
            webhook_id: self.application_id.get(),
        })
        .use_authorization_token(false)
        .build())
    }
}

#[cfg(test)]
mod tests {
    use super::DeleteFollowupMessage;
    use crate::{
        client::Client,
        request::{Request, TryIntoRequest},
        routing::Route,
    };
    use std::error::Error;
    use twilight_model::id::{ApplicationId, MessageId};

    #[test]
    fn test_request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("token".to_owned());

        let builder = DeleteFollowupMessage::new(
            &client,
            ApplicationId::new(1).expect("non zero"),
            "token",
            MessageId::new(2).expect("non zero"),
        );
        let actual = builder.try_into_request()?;

        let expected = Request::from_route(&Route::DeleteWebhookMessage {
            message_id: 2,
            thread_id: None,
            token: "token",
            webhook_id: 1,
        });

        assert_eq!(expected.path, actual.path);
        assert!(!actual.use_authorization_token());

        Ok(())
    }
}
