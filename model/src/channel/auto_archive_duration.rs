use crate::visitor::U16EnumVisitor;
use serde::{
    de::{Deserialize, Deserializer},
    ser::{Serialize, Serializer},
};


#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AutoArchiveDuration {
    Hour,
    Day,
    ThreeDays,
    Week,
    Unknown {
        value: u16,
    },
}

impl AutoArchiveDuration {
    /// Retrieve the length of the duration in seconds, uesd by the API
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::channel::AutoArchiveDuration;
    ///
    /// assert_eq!(60, AutoArchiveDuration::Hour.number());
    /// ```
    pub fn number(self) -> u16 {
        match self {
            Self::Hour => 60,
            Self::Day => 1440,
            Self::ThreeDays => 4320,
            Self::Week => 10080,
            Self::Unknown { value } => value,
        }
    }
}

impl From<u16> for AutoArchiveDuration {
    fn from(value: u16) -> Self {
        match value {
            60 => Self::Hour,
            1440 => Self::Day,
            4320 => Self::ThreeDays,
            10080 => Self::Week,
            value => Self::Unknown { value },
        }
    }
}

impl<'de> Deserialize<'de> for AutoArchiveDuration {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_u16(U16EnumVisitor::new("auto archive duration")).map(u16::into)
    }
}

impl Serialize for AutoArchiveDuration {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u16(self.number())
    }
}

#[cfg(test)]
mod tests {
    use super::AutoArchiveDuration;
    use serde_test::Token;

    const MAP: &[(AutoArchiveDuration, u16)] = &[
        (AutoArchiveDuration::Hour, 60),
        (AutoArchiveDuration::Day, 1440),
        (AutoArchiveDuration::ThreeDays, 4320),
        (AutoArchiveDuration::Week, 10080),
    ];

    #[test]
    fn test_variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(kind, &[Token::U16(*num)]);
            assert_eq!(*kind, AutoArchiveDuration::from(*num));
            assert_eq!(*num, kind.number());
        }
    }

    #[test]
    fn test_unknown_conversion() {
        assert_eq!(
            AutoArchiveDuration::Unknown { value: 250 },
            AutoArchiveDuration::from(250)
        );
    }
}
