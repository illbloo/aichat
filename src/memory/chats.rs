use crate::client::MessageRole;
use crate::memory::MemoryClient;
use anyhow::{Context, Result};
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    pub is_sync: bool,
}

/// Create a new Chat on the memory server.
pub async fn chat_create(client: &MemoryClient, session_id: &str) -> Result<Chat> {
    debug!("Creating chat for session {}", session_id);

    let response = client
        .client
        .post(format!("{}/chats", &client.config.base_url))
        .json(&json!({
            "sessionId": session_id,
        }))
        .send()
        .await?;
    debug!("Chat creation response: {:?}", response);

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_default();
        warn!("Failed to create chat: {}", error);
        return Err(anyhow::anyhow!("Failed to create chat: {}", error));
    }

    let chat = response
        .json::<Chat>()
        .await
        .context("Failed to parse chat")?;
    debug!("Chat created successfully: {}", chat.id);

    Ok(chat)
}

/// Get a Chat from the memory server.
pub async fn chat_get(client: &MemoryClient, chat_id: &str) -> Result<Chat> {
    let response = client
        .client
        .get(format!("{}/chats/{}", &client.config.base_url, chat_id))
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_default();
        warn!("Failed to get chat: {}", error);
        return Err(anyhow::anyhow!("Failed to get chat: {}", error));
    }

    let chat = response
        .json::<Chat>()
        .await
        .context("Failed to parse chat")?;
    Ok(chat)
}

/// List all Chats on the memory server.
pub async fn chat_list(client: &MemoryClient) -> Result<Vec<Chat>> {
    let response = client
        .client
        .get(format!("{}/chats", &client.config.base_url))
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_default();
        warn!("Failed to list chats: {}", error);
        return Err(anyhow::anyhow!("Failed to list chats: {}", error));
    }

    let chats = response
        .json::<Vec<Chat>>()
        .await
        .context("Failed to parse chats")?;
    Ok(chats)
}

/// Writes messages to a Chat on the memory server.
pub async fn chat_add_messages(
    client: &MemoryClient,
    chat_id: &str,
    messages: Vec<ChatMessage>,
) -> Result<()> {
    debug!("Adding {} messages to chat {}", messages.len(), chat_id);

    let response = client
        .client
        .put(format!("{}/chats/{}/messages", &client.config.base_url, chat_id))
        .json(&json!({
            "messages": messages,
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_default();
        warn!("API sync failed: {}", error);
        return Err(anyhow::anyhow!("API sync failed: {}", error));
    }

    Ok(())
}

/// Get all messages from a Chat on the memory server.
pub async fn chat_get_messages(
    client: &MemoryClient,
    chat_id: &str,
) -> Result<Vec<ChatMessage>> {
    let response = client
        .client
        .get(format!("{}/chats/{}/messages", &client.config.base_url, chat_id))
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_default();
        warn!("Failed to get messages: {}", error);
        return Err(anyhow::anyhow!("Failed to get messages: {}", error));
    }

    let messages = response.json::<Vec<ChatMessage>>().await?;
    Ok(messages)
}

pub async fn chat_set_summary(
    client: &MemoryClient,
    chat_id: &str,
    summary: &str,
) -> Result<()> {
    let response = client
        .client
        .put(format!("{}/chats/{}/summary", &client.config.base_url, chat_id))
        .json(&json!({ "summary": summary }))
        .send()
        .await?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_default();
        warn!("Failed to set chat summary: {}", error);
        return Err(anyhow::anyhow!("Failed to set chat summary: {}", error));
    }

    Ok(())
}
