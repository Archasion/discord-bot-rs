use async_trait::async_trait;
use twilight_model::application::interaction::Interaction;
use twilight_model::http::interaction::InteractionResponse;

pub(crate) mod placeholder;

pub(crate) async fn handle_modal(
    ctx: crate::Context,
    cmd: &Interaction,
    custom_id: &str,
) -> anyhow::Result<()> {
    let handler: Box<dyn ModalHandler> = match custom_id {
        "placeholder" => Box::new(placeholder::PlaceholderModal { cmd }),
        _ => anyhow::bail!("unknown modal custom id: {}", custom_id),
    };
    handler.exec(ctx).await
}

/// Trait for implementing modals.
#[async_trait]
pub(crate) trait ModalHandler: Send {
    fn model() -> anyhow::Result<InteractionResponse>
    where
        Self: Sized;
    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()>;
}
