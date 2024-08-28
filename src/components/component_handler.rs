use async_trait::async_trait;
use std::error::Error;
use twilight_model::application::interaction::message_component::MessageComponentInteractionData;
use twilight_model::channel::message::Component;
use twilight_model::http::interaction::InteractionResponse;

#[async_trait]
pub trait ComponentHandler {
    fn model() -> Component;
    async fn exec(
        component: &Box<MessageComponentInteractionData>,
    ) -> Result<InteractionResponse, Box<dyn Error + Send + Sync>>;
}
