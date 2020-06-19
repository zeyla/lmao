use super::allowed_mentions::{AllowedMentions, AllowedMentionsBuilder, Unspecified};
use crate::json_to_vec;
use crate::request::prelude::*;
use reqwest::{
    multipart::{Form, Part},
    Body,
};
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::{
    channel::{embed::Embed, Message},
    id::ChannelId,
};

/// The error created when a messsage can not be created as configured.
#[derive(Clone, Debug)]
pub enum CreateMessageError {
    /// Returned when the content is over 2000 UTF-16 characters.
    ContentInvalid,
    /// Returned when the length of the embed is over 6000 characters.
    EmbedTooLarge {
        /// The source of the error.
        source: EmbedValidationError,
    },
}

impl Display for CreateMessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::ContentInvalid => f.write_str("the message content is invalid"),
            Self::EmbedTooLarge { .. } => f.write_str("the embed's contents are too long"),
        }
    }
}

impl Error for CreateMessageError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ContentInvalid => None,
            Self::EmbedTooLarge { source } => Some(source),
        }
    }
}

#[derive(Default, Serialize)]
pub(crate) struct CreateMessageFields {
    content: Option<String>,
    embed: Option<Embed>,
    nonce: Option<u64>,
    payload_json: Option<Vec<u8>>,
    tts: Option<bool>,
    pub(crate) allowed_mentions: Option<AllowedMentions>,
}

/// # Example
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::ChannelId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let channel_id = ChannelId(123);
/// let message = client
///     .create_message(channel_id)
///     .content("Twilight is best pony")?
///     .tts(true)
///     .await?;
/// # Ok(()) }
/// ```
pub struct CreateMessage<'a> {
    attachments: HashMap<String, Body>,
    channel_id: ChannelId,
    pub(crate) fields: CreateMessageFields,
    fut: Option<Pending<'a, Message>>,
    http: &'a Client,
}

impl<'a> CreateMessage<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            attachments: HashMap::new(),
            channel_id,
            fields: CreateMessageFields {
                allowed_mentions: http.default_allowed_mentions(),
                ..CreateMessageFields::default()
            },
            fut: None,
            http,
        }
    }

    /// Set the content of the message.
    ///
    /// The maximum length is 2000 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns [`CreateMessageError::ContentInvalid`] if the content length is
    /// too long.
    ///
    /// [`CreateMessageError::ContentInvalid`]: enum.CreateMessageError.html#variant.ContentInvalid
    pub fn content(self, content: impl Into<String>) -> Result<Self, CreateMessageError> {
        self._content(content.into())
    }

    fn _content(mut self, content: String) -> Result<Self, CreateMessageError> {
        if !validate::content_limit(&content) {
            return Err(CreateMessageError::ContentInvalid);
        }

        self.fields.content.replace(content);

        Ok(self)
    }

    /// Set the embed of the message.
    ///
    /// Embed total character length must not exceed 6000 characters. Additionally, the internal
    /// fields also have character limits. Refer to [the discord docs] for more information.
    ///
    /// # Examples
    ///
    /// See [`EmbedBuilder`] for an example.
    ///
    /// # Errors
    ///
    /// Returns [`CreateMessageError::EmbedTooLarge`] if the embed is too large.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/channel#embed-limits
    /// [`EmbedBuilder`]: ../../../../../twilight_builders/embed/struct.EmbedBuilder.html
    /// [`CreateMessageError::EmbedTooLarge`]: enum.CreateMessageError.html#variant.EmbedTooLarge
    pub fn embed(mut self, embed: Embed) -> Result<Self, CreateMessageError> {
        validate::embed(&embed).map_err(|source| CreateMessageError::EmbedTooLarge { source })?;

        self.fields.embed.replace(embed);

        Ok(self)
    }

    /// Return a new [`AllowedMentionsBuilder`].
    ///
    /// [`AllowedMentionsBuilder`]: ../allowed_mentions/struct.AllowedMentionsBuilder.html
    pub fn allowed_mentions(
        self,
    ) -> AllowedMentionsBuilder<'a, Unspecified, Unspecified, Unspecified> {
        AllowedMentionsBuilder::for_builder(self)
    }

    /// Attach a new file to the message.
    ///
    /// If the file is an image, see [Discord Docs/Image Data] for more information. It must then
    /// be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}` is the image
    /// MIME type and `{data}` is the base64-encoded image.
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub fn attachment(mut self, name: impl Into<String>, file: impl Into<Body>) -> Self {
        self.attachments.insert(name.into(), file.into());

        self
    }

    /// Insert multiple attachments into the message.
    pub fn attachments<N: Into<String>, F: Into<Body>>(
        mut self,
        attachments: impl IntoIterator<Item = (N, F)>,
    ) -> Self {
        for (name, file) in attachments {
            self = self.attachment(name, file);
        }

        self
    }

    /// Attach a nonce to the message, for optimistic message sending.
    pub fn nonce(mut self, nonce: u64) -> Self {
        self.fields.nonce.replace(nonce);

        self
    }

    /// JSON encoded body of any additional request fields.  See [Discord Docs/Create Message]
    ///
    /// [Discord Docs/Create Message]: https://discord.com/developers/docs/resources/channel#create-message-params
    pub fn payload_json(mut self, payload_json: impl Into<Vec<u8>>) -> Self {
        self.fields.payload_json.replace(payload_json.into());

        self
    }

    /// Specify true if the message is TTS.
    pub fn tts(mut self, tts: bool) -> Self {
        self.fields.tts.replace(tts);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(
            if self.attachments.is_empty() {
                Request::from((
                    json_to_vec(&self.fields)?,
                    Route::CreateMessage {
                        channel_id: self.channel_id.0,
                    },
                ))
            } else {
                let mut form = Form::new();

                for (index, (name, file)) in self.attachments.drain().enumerate() {
                    form = form.part(format!("{}", index), Part::stream(file).file_name(name));
                }

                Request::from((
                    json_to_vec(&self.fields)?,
                    form,
                    Route::CreateMessage {
                        channel_id: self.channel_id.0,
                    },
                ))
            },
        )));

        Ok(())
    }
}

poll_req!(CreateMessage<'_>, Message);
