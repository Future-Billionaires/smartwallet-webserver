use dotenvy::dotenv;
use openai_rust;
use openai_rust::{chat, Client};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    body: String,
    sender: String
}

impl ChatMessage {
    fn new(body: String, sender: String) -> Self {
        Self {
            body,
            sender
        }
    }
}

#[post("/send_chat_message", data="<chat_message>", format="application/json")]
pub async fn send_chat_message(chat_message: Json<ChatMessage>) -> Result<Json<ChatMessage>, Custom<String>>{
    dotenv().ok();

    let client = Client::new(&std::env::var("OPENAI_API_KEY").expect("No environment variable: OPENAI_API_KEY"));
    let args = chat::ChatArguments::new(
        "ft:gpt-3.5-turbo-1106:personal::8QSq2WRB",
        vec![chat::Message {
                role: "system".to_owned(),
                content: "You are an analytical personal finance manager".to_owned()
            },
            chat::Message {
                role: "user".to_owned(),
                content: chat_message.sender.clone() + ": "+ &*chat_message.body.clone()
            }
        ]
    );

    let ai_response = client.create_chat(args).await;

    match ai_response {
        Ok(ai_message) => Ok(Json(ChatMessage::new(ai_message.choices[0].message.content.clone(), "Finance GPT".to_owned()))),
        Err(e) => Err(Custom(Status::InternalServerError, e.to_string()))
    }
}