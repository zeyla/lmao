use crate::id::{marker, Id};
use serde::{Deserialize, Serialize};

/// Partial guild object that a webhook is following.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct WebhookGuild {
    pub icon: Option<String>,
    pub id: Id<marker::Guild>,
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::WebhookGuild;
    use serde::{Deserialize, Serialize};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::{fmt::Debug, hash::Hash};

    assert_fields!(WebhookGuild: icon, id, name);

    assert_impl_all!(
        WebhookGuild: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Serialize
    );
}
