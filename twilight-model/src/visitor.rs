/// Deserializers for optional nullable fields.
///
/// Some booleans in the Discord API are null when true, and not present when
/// false. `serde` doesn't have a way of natively handling this, so we need some
/// custom (de)serialization magic. [`RoleTags`] in particular has these fields.
///
/// [`RoleTags`]: crate::guild::RoleTags
pub mod null_boolean {
    use serde::{
        de::{Deserializer, Error as DeError, Visitor},
        ser::Serializer,
    };
    use std::fmt::{Formatter, Result as FmtResult};

    struct NullBooleanVisitor;

    impl<'de> Visitor<'de> for NullBooleanVisitor {
        type Value = bool;

        fn expecting(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.write_str("null")
        }

        fn visit_none<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(true)
        }

        // `visit_none` is used by `serde_json` when a present `null` value is
        // encountered, but other implementations - such as `simd_json` - may
        // use `visit_unit` instead.
        fn visit_unit<E: DeError>(self) -> Result<Self::Value, E> {
            Ok(true)
        }
    }

    // Clippy will say this bool can be taken by value, but we need it to be
    // passed by reference because that's what serde does.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn serialize<S: Serializer>(_: &bool, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_none()
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
        deserializer.deserialize_option(NullBooleanVisitor)
    }
}
