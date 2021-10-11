use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::Message,
    id::{ApplicationId, MessageId},
};

/// Get a followup message of an interaction.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_http::request::AuditLogReason;
/// use twilight_model::id::{ApplicationId, MessageId};
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// client.set_application_id(ApplicationId::new(1).expect("non zero"));
///
/// let response = client
///     .followup_message("token here", MessageId::new(2).expect("non zero"))?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetFollowupMessage<'a> {
    application_id: ApplicationId,
    http: &'a Client,
    message_id: MessageId,
    interaction_token: &'a str,
}

impl<'a> GetFollowupMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        interaction_token: &'a str,
        message_id: MessageId,
    ) -> Self {
        Self {
            application_id,
            http,
            message_id,
            interaction_token,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for GetFollowupMessage<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetFollowupMessage {
            application_id: self.application_id.get(),
            interaction_token: self.interaction_token,
            message_id: self.message_id.get(),
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::GetFollowupMessage;
    use crate::{
        client::Client,
        request::{IntoRequest, Request},
        routing::Route,
    };
    use static_assertions::assert_impl_all;
    use std::error::Error;
    use twilight_model::id::{ApplicationId, MessageId};

    assert_impl_all!(GetFollowupMessage<'_>: Send, Sync);

    #[test]
    fn test_request() -> Result<(), Box<dyn Error>> {
        const TOKEN: &str = "token";

        fn application_id() -> ApplicationId {
            ApplicationId::new(1).expect("non zero")
        }

        fn message_id() -> MessageId {
            MessageId::new(2).expect("non zero")
        }

        let client = Client::new("token".to_owned());
        client.set_application_id(application_id());

        let actual = client
            .followup_message(TOKEN, message_id())?
            .into_request()?;

        let expected = Request::from_route(&Route::GetFollowupMessage {
            application_id: application_id().get(),
            interaction_token: TOKEN,
            message_id: message_id().get(),
        });

        assert!(expected.body().is_none());
        assert_eq!(expected.path(), actual.path());
        assert_eq!(expected.ratelimit_path(), actual.ratelimit_path());

        Ok(())
    }
}
