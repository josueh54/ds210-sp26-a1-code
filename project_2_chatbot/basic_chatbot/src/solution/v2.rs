use kalosm::language::*;

#[allow(dead_code)]
pub struct ChatbotV2 {
    chat_session: Chat<Llama>,
}

impl ChatbotV2 {
    #[allow(dead_code)]
    pub fn new(model: Llama) -> ChatbotV2 {
        let chat_session: Chat<Llama> = model
            .chat()
            .with_system_prompt("The assistant will act like a pirate");

        ChatbotV2 {
            chat_session: chat_session,
        }
    }

    #[allow(dead_code)]
    pub async fn chat_with_user(&mut self, message: String) -> String {
        let response = self.chat_session.add_message(message).await;

        match response {
            Ok(msg) => msg,
            Err(_) => String::from("error!"),
        }
    }
}