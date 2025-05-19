use async_trait::async_trait;
use twilight_model::application::command::Command;
use twilight_model::application::interaction::Interaction;

pub(crate) mod placeholder;

/// Get all application command models.
pub(crate) fn models() -> anyhow::Result<Vec<Command>> {
    Ok(vec![placeholder::PlaceholderCommand::model()?])
}

pub(crate) async fn handle_command(
    ctx: crate::Context,
    cmd: &Interaction,
    cmd_name: &str,
) -> anyhow::Result<()> {
    let handler: Box<dyn CommandHandler> = match cmd_name {
        "placeholder" => Box::new(placeholder::PlaceholderCommand { cmd }),
        _ => anyhow::bail!("unknown command name: {}", cmd_name),
    };
    handler.exec(ctx).await
}

/// Trait for implementing application commands.
#[async_trait]
pub(crate) trait CommandHandler: Send {
    fn model() -> anyhow::Result<Command>
    where
        Self: Sized;
    async fn exec(&self, ctx: crate::Context) -> anyhow::Result<()>;
}
