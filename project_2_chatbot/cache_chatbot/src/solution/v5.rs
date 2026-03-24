use kalosm::language::*;
use file_chatbot::solution::file_library;

use crate::solution::Cache;

pub struct ChatbotV5 {
    model: Llama,
    cache: Cache<Chat<Llama>>,
}

impl ChatbotV5 {
    pub fn new(model: Llama) -> ChatbotV5 {
        return ChatbotV5 {
            model: model,
            cache: Cache::new(3),
        };
    }

    pub async fn chat_with_user(&mut self, username: String, message: String) -> String {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("chat_with_user: {username} is not in the cache!");
                // The cache does not have the chat. What should you do?
                let mut new_chat: Chat<Llama> = self.model
                    .chat()
                    .with_system_prompt("The assistant will act like a pirate");

                match file_library::load_chat_session_from_file(filename) {
                    None => {}
                    Some(session) => {
                        new_chat = new_chat.with_session(session);
                    }
                }

                let response = new_chat.add_message(message).await.unwrap();

                {
                    let updated_session = new_chat.session().unwrap();
                    file_library::save_chat_session_to_file(filename, &updated_session);
                }

                self.cache.insert_chat(username.clone(), new_chat);

                return response;
            }
            Some(chat_session) => {
                println!("chat_with_user: {username} is in the cache! Nice!");
                // The cache has this chat. What should you do?
                let response = chat_session.add_message(message).await.unwrap();

                {
                    let updated_session = chat_session.session().unwrap();
                    file_library::save_chat_session_to_file(filename, &updated_session);
                }

                return response;
            }
        }
    }

    pub fn get_history(&mut self, username: String) -> Vec<String> {
        let filename = &format!("{}.txt", username);
        let cached_chat = self.cache.get_chat(&username);

        match cached_chat {
            None => {
                println!("get_history: {username} is not in the cache!");
                // TODO: The cache does not have the chat. What should you do?
                // Your code goes here.
                return Vec::new();
            }
            Some(chat_session) => {
                println!("get_history: {username} is in the cache! Nice!");
                // TODO: The cache has this chat. What should you do?
                // Your code goes here.
                return Vec::new();

            }
        }
    }
}