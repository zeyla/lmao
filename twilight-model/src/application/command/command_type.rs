use serde::{Deserialize, Serialize};

// Keep in sync with `twilight-validate::command`!
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct CommandType(u8);

impl CommandType {
    /// Slash command.
    ///
    /// Text-based command that appears when a user types `/`.
    pub const CHAT_INPUT: Self = Self::new(1);

    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps on a user.
    pub const USER: Self = Self::new(2);

    /// UI-based command.
    ///
    /// Appears when a user right clicks or taps on a message.
    pub const MESSAGE: Self = Self::new(3);

    /// Create a new command type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`CHAT_INPUT`][`Self::CHAT_INPUT`].
    pub const fn new(command_type: u8) -> Self {
        Self(command_type)
    }

    /// Retrieve the value of the command type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::application::command::CommandType;
    ///
    /// assert_eq!(3, CommandType::MESSAGE.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }
}

impl From<u8> for CommandType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<CommandType> for u8 {
    fn from(value: CommandType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::CommandType;
    use serde_test::Token;

    const MAP: &[(CommandType, u8)] = &[
        (CommandType::CHAT_INPUT, 1),
        (CommandType::USER, 2),
        (CommandType::MESSAGE, 3),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "CommandType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, CommandType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
