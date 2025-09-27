use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::Json;
        use crate::backend::errors::{
            application_error::ErrorObject, bad_client_request::BadClientRequest,
            invalid_message::InvalidMessageError,
        };

    }
}
use serde::{Deserialize, Serialize};
use std::fmt;

// constants

/// The maximum number of messages that can be stored in a chat.
const MAX_CHAT_MESSAGE_LENGTH: usize = 30; // 30 messages

/// Represents a chat in the game, containing messages exchanged between players.
///
/// The `Chat` struct holds a vector of `ChatMessage` instances and tracks the number of messages.
///
/// # Example usage:
/// ```rust
/// use uuid::Uuid;
/// use chrono::Utc;
/// use your_crate::chat::{Chat, ChatMessage};
/// let mut chat = Chat {
///    messages: Vec::new(),
///    number_of_messages: 0,
///    id: "9fd2151d-432e-4549-99bf-b684b5be9555".to_string()
///    };
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Chat {
    /// Identifier of a chat instance
    pub id: String,
    /// List of all messages sent in the chat of a game
    pub messages: Vec<ChatMessage>,
    /// Total number of all messages
    ///
    /// Maximal number: 50
    pub number_of_messages: usize,
    /// The identifier of the game the chat belongs to.
    pub game_id: String,
}

/// Represents a chat message in the game, containing the player ID and the message content.
///
/// The `ChatMessage` struct holds the ID of the player who sent the message, the content of the
/// message,
/// and the timestamp of when the message was sent.
///
/// # Example usage:
/// ```rust
/// use uuid::Uuid;
/// use chrono::Utc;
/// use your_crate::chat::ChatMessage;
/// let message = ChatMessage {
///    id: Uuid::new_v4().to_string(),
///    player_id: Uuid::new_v4().to_string(),
///    content: String::from("Hello, world!"),
///    sent_at: Utc::now().to_string(),
///    };
/// ```  
#[derive(Deserialize, Serialize, Clone)]
pub struct ChatMessage {
    /// Identifier of the ChatMessage
    pub id: String,
    /// ID of the player, who sent the message
    pub player_id: String,
    /// Content of the message
    pub content: String,
    /// Date string, when the message was sent by the user
    pub sent_at: String, // as chrono::DateTime<chrono::Utc>,
    /// ID of the chat, where the message was sent.
    pub chat_id: String,
}

// Implementation of 'Chat' struct

impl Default for Chat {
    fn default() -> Self {
        Self::new(uuid::Uuid::new_v4().to_string())
    }
}

impl Chat {
    /// Creates a fresh 'Chat' instance.
    ///
    /// # Returns
    /// - 'Chat' object with no messages
    pub fn new(game_id: String) -> Self {
        Chat {
            id: uuid::Uuid::new_v4().to_string(),
            messages: vec![],
            number_of_messages: 0,
            game_id,
        }
    }

    /// Resets the 'Chat' instance.
    ///
    /// Messages will be deleted and number of messages set to null.
    pub fn reset(&mut self) {
        self.number_of_messages = 0;
        self.messages = vec![];
    }

    /// Tries adding a message to the 'Chat' instance of a game.
    ///
    /// When the message vector is full then the oldest massage is deleted and the new message was
    /// added.
    ///
    /// # Errors
    ///
    /// When the message itself is too short or has no content then it isn't stored or saved
    ///
    /// # Returns
    ///
    /// Result<(), ApplicationError> - When the message was invalid.
    #[cfg(feature = "ssr")]
    pub fn add_chat_message(&mut self, message: ChatMessage) -> Result<(), InvalidMessageError> {
        // message needs to be long enough
        if message.content.is_empty() {
            println!("The message is too short to be added to the chat!");
            return Err(InvalidMessageError {
                message: "Too short message content! Must not be of length 0!".to_string(),
                origin_message: message,
            });
        }

        // check if the maximum number of messages was reached
        if self.number_of_messages >= MAX_CHAT_MESSAGE_LENGTH {
            println!("Maximum number of chat messages has been reached! Deleting oldest message to add the new one.");
            self.messages.remove(0);
            self.messages.push(message);
            return Ok(());
        }

        // add the message to the chat in the normal case
        self.number_of_messages += 1;
        self.messages.push(message);

        Ok(())
    }
}

// ----- Implementation of the 'ErrorObject' trait for the 'Chat' struct -----

#[cfg(feature = "ssr")]
impl<'a> ErrorObject<'a> for Chat {}

impl fmt::Display for Chat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "
            ID of the Chat: {},
            Number of Messages: {},
            Messages: {:?}
            ",
            self.id, self.number_of_messages, self.messages
        )
    }
}

// Implementation of 'ChatMessage' struct

impl ChatMessage {
    /// Creates new 'ChatMessage' instance.
    ///
    /// # Returns
    ///
    /// New 'ChatMessage' object with the player id, message body and when it was sent.
    ///
    /// # Errors
    ///
    /// Returns 'BadClientRequest' if the a client provided invalid data.
    #[cfg(feature = "ssr")]
    pub fn new(
        id: String,
        player_id: String,
        content: String,
        sent_at: String,
        chat_id: String,
    ) -> Result<Self, BadClientRequest<ChatMessage>> {
        if content.is_empty() || player_id.is_empty() || sent_at.is_empty() {
            return Err::<ChatMessage, BadClientRequest<_>>(BadClientRequest {
                bad_data: Json(ChatMessage {
                    id: id.clone(),
                    player_id: player_id.clone(),
                    sent_at: sent_at.clone(),
                    content: content.clone(),
                    chat_id: chat_id.clone(),
                }),
                message: format!(
                    "The provided data by player with id: {} for a chat message was not valid!",
                    &player_id
                ),
            });
        };
        Ok(ChatMessage {
            id,
            player_id,
            content,
            sent_at,
            chat_id,
        })
    }
}
impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[
           Id: {},
           PlayerID: {},
           Content: {},
           Sent at: {},
           Chat ID: {}
            ]",
            self.id, self.player_id, self.content, self.sent_at, self.chat_id
        )
    }
}

impl fmt::Debug for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[
           Id: {},
           PlayerID: {},
           Content: {},
           Sent at: {}
           Chat ID: {}
            ]",
            self.id, self.player_id, self.content, self.sent_at, self.chat_id
        )
    }
}

#[cfg(feature = "ssr")]
impl<'a> ErrorObject<'a> for ChatMessage {}
