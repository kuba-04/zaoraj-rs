use core::str;
use std::fmt::Pointer;
use std::string::ToString;

use actix_http::body::MessageBody;
use actix_web::{FromRequest, HttpResponse};
use actix_web::http::header::ContentType;
use actix_web::web::{Data, Json};
use async_openai::Client;
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestUserMessage, ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest, CreateChatCompletionResponse, Prompt};
use serde_derive::Deserialize;

const MODEL: &str = "gpt-3.5-turbo";

#[derive(Deserialize)]
pub struct FormData {
    message: String,
}

pub async fn complain(
    form: Json<FormData>,
    ai_client: Data<Client<OpenAIConfig>>,
    prompt: Data<Prompt>,
) -> HttpResponse {
    let messages = construct_prompt_message_from(form, prompt);
    let request = construct_request_from(messages);
    let chat  = ai_client.chat();
    let response = chat.create(request).await.expect("Failed to parse response");
    let riposte = construct_riposte_from(&response);

    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(riposte)
}

fn construct_riposte_from(response: &CreateChatCompletionResponse) -> String {
    let riposte = response
        .choices.get(0)
        .expect("Failed to parse the choice from response");
    riposte.clone().message.content.unwrap()
}

fn construct_request_from(messages: Vec<ChatCompletionRequestMessage>) -> CreateChatCompletionRequest {
    CreateChatCompletionRequest {
        model: MODEL.to_string(),
        frequency_penalty: None,
        logit_bias: None,
        logprobs: None,
        top_logprobs: None,
        max_tokens: None,
        n: None,
        presence_penalty: None,
        response_format: None,
        seed: None,
        stop: None,
        stream: None,
        messages: messages,
        temperature: None,
        top_p: None,
        tools: None,
        tool_choice: None,
        user: None,
        function_call: None,
        stream_options: None,
        functions: None,
    }
}

fn construct_prompt_message_from(form: Json<FormData>, prompt: Data<Prompt>) -> Vec<ChatCompletionRequestMessage> {
    let prompt = match prompt.as_ref() {
        Prompt::String(s) => Some(s.clone()),
        _ => None
    }.expect("Failed to extract string from prompt");

    let message = prompt + form.0.message.as_str();

    let user_message = ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage {
        content: ChatCompletionRequestUserMessageContent::Text(message),
        name: None
    });

    let mut messages: Vec<ChatCompletionRequestMessage> = Vec::new();
    messages.push(user_message);
    messages
}