use anyhow::Context;
use async_trait::async_trait;
use twilight_model::application::command::{Command, CommandType};
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::component::ActionRow;
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::command::CommandBuilder;
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::commands::CommandHandler;
use crate::components::placeholder::PlaceholderComponent;
use crate::components::ComponentHandler;

pub(crate) struct PlaceholderCommand<'a> {
    pub(crate) cmd: &'a Interaction,
}

#[async_trait]
impl CommandHandler for PlaceholderCommand<'_> {
    fn model() -> anyhow::Result<Command> {
        Ok(CommandBuilder::new(
            "placeholder",
            "This is a placeholder command",
            CommandType::ChatInput,
        )
        .validate()
        .context("validate application command")?
        .build())
    }

    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()> {
        let button_action_row = Component::ActionRow(ActionRow {
            components: vec![PlaceholderComponent::model()?],
        });
        // Create a response to the interaction.
        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content("This is a placeholder command")
                    .components([button_action_row])
                    .build(),
            ),
        };

        // Send the response to Discord.
        ctx.http
            .interaction(self.cmd.application_id)
            .create_response(self.cmd.id, &self.cmd.token, &response)
            .await
            .context("create interaction response")?;

        Ok(())
    }
}
