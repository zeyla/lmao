//! Constants, error types, and functions for validating [`Command`]s.

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::application::command::Command;

/// Maximum length of a command's description.
pub const COMMAND_DESCRIPTION_LENGTH_MAX: usize = 100;

/// Minimum length of a command's description.
pub const COMMAND_DESCRIPTION_LENGTH_MIN: usize = 1;

/// Maximum length of a command's name.
pub const COMMAND_NAME_LENGTH_MAX: usize = 32;

/// Minimum length of a command's name.
pub const COMMAND_NAME_LENGTH_MIN: usize = 1;

/// Maximum amount of options a command may have.
pub const COMMAND_OPTIONS_LIMIT: usize = 25;

/// Maximum number of commands an application may have in an individual
/// guild.
pub const GUILD_COMMAND_LIMIT: usize = 100;

/// Maximum number of permission overwrites an application may have in an
/// individual guild command.
pub const GUILD_COMMAND_PERMISSION_LIMIT: usize = 10;

/// Error created when a [`Command`] is invalid.
#[derive(Debug)]
pub struct CommandValidationError {
    /// Type of error that occurred.
    kind: CommandValidationErrorType,
}

impl CommandValidationError {
    /// Constant instance of a [`CommandValidationError`] with type
    /// [`CommandCountInvalid`].
    ///
    /// [`CommandCountInvalid`]: CommandValidationErrorType::CommandCountInvalid
    pub const COMMAND_COUNT_INVALID: CommandValidationError = CommandValidationError {
        kind: CommandValidationErrorType::CommandCountInvalid,
    };

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &CommandValidationErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        CommandValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }

    /// Create an error of type [`CommandOptionsRequiredFirst`] with a provided
    /// index.
    ///
    /// [`CommandOptionsRequiredFirst`]: CommandValidationErrorType::CommandOptionsRequiredFirst
    #[must_use = "creating an error has no effect if left unused"]
    pub const fn command_option_required_first(index: usize) -> Self {
        Self {
            kind: CommandValidationErrorType::CommandOptionsRequiredFirst { index },
        }
    }
}

impl Display for CommandValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            CommandValidationErrorType::CommandCountInvalid => {
                f.write_str("more than ")?;
                Display::fmt(&GUILD_COMMAND_LIMIT, f)?;

                f.write_str(" commands were set")
            }
            CommandValidationErrorType::CommandOptionsRequiredFirst { .. } => {
                f.write_str("optional command options must be added after required")
            }
            CommandValidationErrorType::DescriptionInvalid => {
                f.write_str("command description must be between ")?;
                Display::fmt(&COMMAND_DESCRIPTION_LENGTH_MIN, f)?;
                f.write_str(" and ")?;
                Display::fmt(&COMMAND_DESCRIPTION_LENGTH_MAX, f)?;

                f.write_str(" characters")
            }
            CommandValidationErrorType::NameInvalid => {
                f.write_str("command name must be between ")?;
                Display::fmt(&COMMAND_NAME_LENGTH_MIN, f)?;
                f.write_str(" and ")?;
                Display::fmt(&COMMAND_NAME_LENGTH_MAX, f)?;

                f.write_str(" characters")
            }
            CommandValidationErrorType::PermissionsCountInvalid => {
                f.write_str("more than ")?;
                Display::fmt(&GUILD_COMMAND_PERMISSION_LIMIT, f)?;

                f.write_str(" permission overwrites were set")
            }
        }
    }
}

impl Error for CommandValidationError {}

/// Type of [`CommandValidationError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum CommandValidationErrorType {
    /// Too many commands have been provided.
    ///
    /// The maximum number of commands is defined by
    /// [`GUILD_COMMAND_LIMIT`].
    CommandCountInvalid,
    /// Required command options have to be passed before optional ones.
    CommandOptionsRequiredFirst {
        /// Index of the option that failed validation.
        index: usize,
    },
    /// Command description is invalid.
    DescriptionInvalid,
    /// Command name is invalid.
    NameInvalid,
    /// More than 10 permission overwrites were set.
    PermissionsCountInvalid,
}

/// Validate a [`Command`].
///
/// # Errors
///
/// Returns an error with type [`DescriptionInvalid`] if the description is
/// invalid.
///
/// Returns an error with type [`NameInvalid`] if the name is invalid.
///
/// [`DescriptionInvalid`]: CommandValidationErrorType::DescriptionInvalid
/// [`NameInvalid`]: CommandValidationErrorType::NameInvalid
pub fn command(value: &Command) -> Result<(), CommandValidationError> {
    let Command {
        description, name, ..
    } = value;

    self::description(description)?;

    self::name(name)?;

    Ok(())
}

/// Validate the description of a [`Command`].
///
/// The length of the description must be more than
/// [`COMMAND_DESCRIPTION_LENGTH_MIN`] and less than or equal to
/// [`COMMAND_DESCRIPTION_LENGTH_MAX`].
///
/// # Errors
///
/// Returns an error with type [`DescriptionInvalid`] if the description is
/// invalid.
///
/// [`DescriptionInvalid`]: CommandValidationErrorType::DescriptionInvalid
pub fn description(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let len = value.as_ref().chars().count();

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    if (COMMAND_DESCRIPTION_LENGTH_MIN..=COMMAND_DESCRIPTION_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::DescriptionInvalid,
        })
    }
}

/// Validate the name of a [`Command`].
///
/// The length of the name must be more than [`COMMAND_NAME_LENGTH_MIN`] and
/// less than or equal to [`COMMAND_NAME_LENGTH_MAX`].
///
/// # Errors
///
/// Returns an error with type [`NameInvalid`] if the name is invalid.
///
/// [`NameInvalid`]: CommandValidationErrorType::NameInvalid
pub fn name(value: impl AsRef<str>) -> Result<(), CommandValidationError> {
    let len = value.as_ref().chars().count();

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-option-structure
    if (COMMAND_NAME_LENGTH_MIN..=COMMAND_NAME_LENGTH_MAX).contains(&len) {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::NameInvalid,
        })
    }
}

/// Validate the number of guild command permission overwrites.
///
/// The maximum number of commands allowed in a guild is defined by
/// [`GUILD_COMMAND_PERMISSION_LIMIT`].
///
/// # Errors
///
/// Returns an error with type [`PermissionsCountInvalid`] if the permissions
/// are invalid.
///
/// [`PermissionsCountInvalid`]: CommandValidationErrorType::PermissionsCountInvalid
pub const fn guild_permissions(count: usize) -> Result<(), CommandValidationError> {
    // https://discord.com/developers/docs/interactions/application-commands#registering-a-command
    if count <= GUILD_COMMAND_PERMISSION_LIMIT {
        Ok(())
    } else {
        Err(CommandValidationError {
            kind: CommandValidationErrorType::PermissionsCountInvalid,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use twilight_model::{application::command::CommandType, id::Id};

    // This tests [`description`] and [`name`] by proxy.
    #[test]
    fn test_command() {
        let valid_command = Command {
            application_id: Some(Id::new(1).expect("non zero")),
            default_permission: None,
            description: "a".repeat(100),
            guild_id: Some(Id::new(2).expect("non zero")),
            id: Some(Id::new(3).expect("non zero")),
            kind: CommandType::ChatInput,
            name: "b".repeat(32),
            options: Vec::new(),
            version: Id::new(4).expect("non zero"),
        };

        assert!(command(&valid_command).is_ok());

        let invalid_command = Command {
            description: "c".repeat(101),
            name: "d".repeat(33),
            ..valid_command
        };

        assert!(command(&invalid_command).is_err());
    }

    #[test]
    fn test_guild_permissions() {
        assert!(guild_permissions(0).is_ok());
        assert!(guild_permissions(1).is_ok());
        assert!(guild_permissions(10).is_ok());

        assert!(guild_permissions(11).is_err());
    }
}
