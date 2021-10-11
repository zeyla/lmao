use super::{ThreadValidationError, ThreadValidationErrorType};
use crate::{
    client::Client,
    error::Error,
    request::{validate_inner, IntoRequest, Request, RequestBuilder},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::{thread::AutoArchiveDuration, Channel, ChannelType},
    id::ChannelId,
};

#[derive(Serialize)]
struct CreateThreadFields<'a> {
    auto_archive_duration: AutoArchiveDuration,
    #[serde(skip_serializing_if = "Option::is_none")]
    invitable: Option<bool>,
    #[serde(rename = "type")]
    kind: ChannelType,
    name: &'a str,
}

/// Start a thread that is not connected to a message.
///
/// Values of [`ThreeDays`] and [`Week`] require the guild to be boosted.  The
/// guild's features will indicate if a guild is able to use these settings.
///
/// To make a [`GuildPrivateThread`], the guild must also have the
/// `PRIVATE_THREADS` feature.
///
/// [`GuildPrivateThread`]: twilight_model::channel::ChannelType::GuildPrivateThread
/// [`ThreeDays`]: twilight_model::channel::thread::AutoArchiveDuration::ThreeDays
/// [`Week`]: twilight_model::channel::thread::AutoArchiveDuration::Week
#[must_use = "requests must be configured and executed"]
pub struct CreateThread<'a> {
    channel_id: ChannelId,
    fields: CreateThreadFields<'a>,
    http: &'a Client,
}

impl<'a> CreateThread<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        name: &'a str,
        auto_archive_duration: AutoArchiveDuration,
        kind: ChannelType,
    ) -> Result<Self, ThreadValidationError> {
        if !validate_inner::channel_name(name) {
            return Err(ThreadValidationError {
                kind: ThreadValidationErrorType::NameInvalid,
            });
        }

        if !validate_inner::is_thread(kind) {
            return Err(ThreadValidationError {
                kind: ThreadValidationErrorType::TypeInvalid { kind },
            });
        }

        Ok(Self {
            channel_id,
            fields: CreateThreadFields {
                auto_archive_duration,
                invitable: None,
                kind,
                name,
            },
            http,
        })
    }

    /// Whether non-moderators can add other non-moderators to a thread.
    pub const fn invitable(mut self, invitable: bool) -> Self {
        self.fields.invitable = Some(invitable);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Channel> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for CreateThread<'_> {
    fn into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::CreateThread {
            channel_id: self.channel_id.get(),
        })
        .json(&self.fields)
        .map(RequestBuilder::build)
    }
}
