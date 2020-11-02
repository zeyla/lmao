use crate::request::prelude::*;
use twilight_model::applications::Command;
use twilight_model::applications::CommandOption;
use twilight_model::id::*;

pub struct UpdateGuildCommand<'a> {
    command: Command,
    application_id: ApplicationId,
    guild_id: GuildId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> UpdateGuildCommand<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
        name: String,
        description: String,
    ) -> Self {
        Self {
            command: Command {
                id: Some(command_id),
                application_id,
                name,
                description,
                options: vec![],
            },
            application_id,
            guild_id,
            fut: None,
            http,
        }
    }

    pub fn push_command_option(mut self, option: CommandOption) -> Self {
        self.command.options.push(option);

        self
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from((
            crate::json_to_vec(&self.command)?,
            Route::UpdateGuildCommand {
                application_id: self.application_id.0,
                // This unwrap is safe to do as the command_id will
                // always be filled when you initialize the
                // struct. And it is not possible to change it.
                //
                // TODO: REVIEW-QUESTION: Would it be better to have
                // another command_id outside of the command struct?
                command_id: self.command.id.unwrap().0,
                guild_id: self.guild_id.0,
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(UpdateGuildCommand<'_>, ());
