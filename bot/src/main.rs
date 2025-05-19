mod commands;
mod components;
mod modals;

use std::env;
use std::sync::Arc;

use anyhow::Context as _;
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::InteractionData;

/// The context contains anything that needs to be shared between
/// the event handlers. In this case, we only need the HTTP client.
///
/// You can add more fields to this struct as needed (such as a database).
#[derive(Clone)]
pub(crate) struct Context {
    pub(crate) http: Arc<HttpClient>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").context("get DISCORD_TOKEN env")?;

    // Use intents to only receive guild message events.
    let mut shard = Shard::new(
        ShardId::ONE,
        token.clone(),
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
    );

    // HTTP is separate from the gateway, so create a new client.
    let http = Arc::new(HttpClient::new(token));

    // Since we only care about new messages, make the cache only
    // cache new messages.
    let cache = DefaultInMemoryCache::builder()
        .resource_types(ResourceType::MESSAGE)
        .build();

    // Create the state with the HTTP client
    let ctx = Context { http: http.clone() };

    // Process each event as they come in.
    while let Some(item) = shard
        .next_event(
            // We only care about the `Ready` and `InteractionCreate` events.
            EventTypeFlags::from_bits_retain(
                EventTypeFlags::READY.bits() | EventTypeFlags::INTERACTION_CREATE.bits(),
            ),
        )
        .await
    {
        let Ok(event) = item else {
            tracing::warn!(source = ?item.unwrap_err(), "error receiving event");
            continue;
        };

        // Update the cache with the event.
        cache.update(&event);
        tokio::spawn(handle_event(event, ctx.clone()));
    }

    Ok(())
}

#[tracing::instrument(skip(ctx))]
async fn handle_event(event: Event, ctx: Context) -> anyhow::Result<()> {
    match event {
        Event::Ready(client) => {
            tracing::info!(
                "the client has logged in as @{} ({})",
                client.user.name,
                client.user.id
            );

            // Publish commands every time the bot starts
            // to ensure they are always up to date.
            let global_commands = ctx
                .http
                .interaction(client.application.id)
                .set_global_commands(commands::models()?.as_slice())
                .await
                .context("publish global commands")?
                .models()
                .await
                .context("get global commands")?;

            tracing::info!("published {} global commands", global_commands.len());
        },
        Event::InteractionCreate(interaction) => {
            match &interaction.data {
                Some(InteractionData::ApplicationCommand(command)) => {
                    commands::handle_command(ctx, &interaction, &command.name)
                        .await
                        .with_context(|| format!("execute command: {}", command.name))?;
                },
                Some(InteractionData::MessageComponent(component)) => {
                    components::handle_component(ctx, &interaction, &component.custom_id)
                        .await
                        .with_context(|| format!("execute component: {}", component.custom_id))?;
                },
                Some(InteractionData::ModalSubmit(modal)) => {
                    modals::handle_modal(ctx, &interaction, &modal.custom_id)
                        .await
                        .with_context(|| format!("execute modal: {}", modal.custom_id))?;
                },
                _ => anyhow::bail!("unsupported interaction type"),
            };
        },
        _ => {},
    }

    Ok(())
}
