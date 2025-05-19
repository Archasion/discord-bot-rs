use anyhow::Context;
use async_trait::async_trait;
use builders::component::ActionRowBuilder;
use builders::modal::{ModalBuilder, TextInputBuilder};
use twilight_model::application::interaction::{Interaction, InteractionData};
use twilight_model::channel::message::component::TextInputStyle;
use twilight_model::http::interaction::{InteractionResponse, InteractionResponseType};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::modals::ModalHandler;

pub(crate) struct PlaceholderModal<'a> {
    pub(crate) cmd: &'a Interaction,
}

#[async_trait]
impl ModalHandler for PlaceholderModal<'_> {
    fn model() -> anyhow::Result<InteractionResponse> {
        let text_input =
            TextInputBuilder::new("Placeholder", "placeholder", TextInputStyle::Paragraph)
                .max_length(256)
                .required(true)
                .build()?;
        let action_row = ActionRowBuilder::new().add_component(text_input).build()?;

        ModalBuilder::new("Placeholder", "placeholder")
            .add_component(action_row)
            .build()
    }

    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()> {
        let Some(InteractionData::ModalSubmit(data)) = &self.cmd.data else {
            anyhow::bail!("expected modal interaction");
        };
        // The text input is required, so we can unwrap it.
        let input = data.components[0].components[0].value.as_ref().unwrap();
        let response = InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                InteractionResponseDataBuilder::new()
                    .content(format!("> {input}"))
                    .build(),
            ),
        };

        ctx.http
            .interaction(self.cmd.application_id)
            .create_response(self.cmd.id, &self.cmd.token, &response)
            .await
            .context("create interaction response")?;

        Ok(())
    }
}
