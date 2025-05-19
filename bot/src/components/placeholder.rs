use anyhow::Context;
use async_trait::async_trait;
use builders::component::ButtonBuilder;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::component::ButtonStyle;
use twilight_model::channel::message::Component;

use crate::components::ComponentHandler;
use crate::modals::placeholder::PlaceholderModal;
use crate::modals::ModalHandler;

pub(crate) struct PlaceholderComponent<'a> {
    pub(crate) cmd: &'a Interaction,
}

#[async_trait]
impl ComponentHandler for PlaceholderComponent<'_> {
    fn model() -> anyhow::Result<Component> {
        ButtonBuilder::new("placeholder", ButtonStyle::Primary)
            .label("Placeholder")
            .build()
    }

    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()> {
        // Respond to the interaction with a modal.
        ctx.http
            .interaction(self.cmd.application_id)
            .create_response(self.cmd.id, &self.cmd.token, &PlaceholderModal::model()?)
            .await
            .context("create interaction response")?;

        Ok(())
    }
}
