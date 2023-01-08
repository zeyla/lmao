use serde::{Deserialize, Serialize};

/// Type of a [`Sticker`].
///
/// [`Sticker`]: super::Sticker
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct StickerType(u8);

impl StickerType {
    /// Official sticker in a pack.
    ///
    /// Part of nitro or in a removed purchasable pack.
    pub const STANDARD: Self = Self::new(1);

    /// Sticker uploaded to a boosted guild for the guild's members.
    pub const GUILD: Self = Self::new(2);

    /// Create a new sticker type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`STANDARD`][`Self::STANDARD`].
    pub const fn new(sticker_type: u8) -> Self {
        Self(sticker_type)
    }

    /// Retrieve the value of the sticker type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::message::sticker::StickerType;
    ///
    /// assert_eq!(2, StickerType::GUILD.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for StickerType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<StickerType> for u8 {
    fn from(value: StickerType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::StickerType;
    use serde_test::Token;

    #[test]
    fn variants() {
        serde_test::assert_tokens(
            &StickerType::STANDARD,
            &[
                Token::NewtypeStruct {
                    name: "StickerType",
                },
                Token::U8(1),
            ],
        );
        serde_test::assert_tokens(
            &StickerType::GUILD,
            &[
                Token::NewtypeStruct {
                    name: "StickerType",
                },
                Token::U8(2),
            ],
        );
        serde_test::assert_tokens(
            &StickerType::new(99),
            &[
                Token::NewtypeStruct {
                    name: "StickerType",
                },
                Token::U8(99),
            ],
        );
    }

    #[test]
    fn conversions() {
        assert_eq!(StickerType::from(1), StickerType::STANDARD);
        assert_eq!(StickerType::from(2), StickerType::GUILD);
        assert_eq!(StickerType::from(99), StickerType::new(99));
    }
}
