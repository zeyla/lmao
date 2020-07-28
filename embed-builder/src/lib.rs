//! # twilight-embed-builder
//!
//! Builders for creating an embed, useful when creating or updating messages.
//!
//! If uploading an image as an attachment, set as the image or thumbnail with
//! `attachment://{filename}.{extension}`. Refer to [the discord docs] for more information.
//!
//! # Examples
//!
//! Build a simple embed:
//!
//! ```rust,no_run
//! use twilight_embed_builder::{EmbedBuilder, EmbedFieldBuilder};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let embed = EmbedBuilder::new()
//!     .description("Here's a list of reasons why Twilight is the best pony:")?
//!     .field(EmbedFieldBuilder::new("Wings", "She has wings.")?.inline())
//!     .field(EmbedFieldBuilder::new("Horn", "She can do magic, and she's really good at it.")?.inline())
//!     .build();
//! # Ok(()) }
//! ```
//!
//! Build an embed with an image:
//!
//! ```rust,no_run
//! use twilight_embed_builder::EmbedBuilder;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//! let embed = EmbedBuilder::new()
//!     .description("Here's a cool image of Twilight Sparkle")?
//!     .image("attachment://bestpony.png")
//!     .build();
//!
//! # Ok(()) }
//! ```
//!
//! [the discord docs]: https://discord.com/developers/docs/resources/channel#create-message-using-attachments-within-embeds

#![deny(
    clippy::all,
    clippy::pedantic,
    future_incompatible,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    unsafe_code,
    unused,
    warnings
)]

pub mod author;
pub mod builder;
pub mod field;
pub mod footer;

pub use self::{
    author::{EmbedAuthorBuilder, EmbedAuthorNameError},
    builder::{
        EmbedBuildError, EmbedBuilder, EmbedColorError, EmbedDescriptionError, EmbedTitleError,
    },
    field::{EmbedFieldBuilder, EmbedFieldError},
    footer::EmbedFooterBuilder,
};

#[cfg(test)]
mod tests {
    use super::*;
    use twilight_model::channel::embed::{Embed, EmbedField, EmbedFooter};

    #[test]
    fn builder_test() {
        let embed = EmbedBuilder::new()
            .color(0x004_3FF)
            .unwrap()
            .description("Description")
            .unwrap()
            .timestamp("123")
            .footer(EmbedFooterBuilder::new("Warn").unwrap().icon_url("icon"))
            .field(EmbedFieldBuilder::new("name", "title").unwrap().inline())
            .build()
            .unwrap();

        let expected = Embed {
            author: None,
            color: Some(17407),
            description: Some("Description".to_string()),
            fields: [EmbedField {
                inline: true,
                name: "name".to_string(),
                value: "title".to_string(),
            }]
            .to_vec(),
            footer: Some(EmbedFooter {
                icon_url: Some("icon".to_string()),
                proxy_icon_url: None,
                text: "Warn".to_string(),
            }),
            image: None,
            kind: "rich".to_string(),
            provider: None,
            thumbnail: None,
            timestamp: Some("123".to_string()),
            title: None,
            url: None,
            video: None,
        };

        assert_eq!(embed, expected);
    }
}
