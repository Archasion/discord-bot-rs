use async_trait::async_trait;
use twilight_model::application::interaction::Interaction;
use twilight_model::channel::message::Component;

pub(crate) mod placeholder;

pub(crate) async fn handle_component(
    ctx: crate::Context,
    cmd: &Interaction,
    custom_id: &str,
) -> anyhow::Result<()> {
    let handler: Box<dyn ComponentHandler> = match custom_id {
        "placeholder" => Box::new(placeholder::PlaceholderComponent { cmd }),
        _ => anyhow::bail!("unknown component custom id: {}", custom_id),
    };
    handler.exec(ctx).await
}

/// Trait for implementing message components.
/// See the [`Component`] enum for supported components.
#[async_trait]
pub(crate) trait ComponentHandler: Send {
    fn model() -> anyhow::Result<Component>
    where
        Self: Sized;
    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()>;
}
