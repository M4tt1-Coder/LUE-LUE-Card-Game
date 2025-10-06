use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
use axum::http::StatusCode;
use wasm_bindgen::JsValue;
use worker::D1Database;

use crate::backend::{
    errors::{application_error::ApplicationError, database_query_error::DatabaseQueryError},
    types::chat::ChatMessage,
};

/// A database repository for interacting with the `chat_messages` table.
///
/// Contains the utility functions for the `ChatMessage` struct.
///
/// It will be accessable in the context element in the handler functions.
pub struct ChatMessageRepository {
    /// Database service pointer to execute queries.
    ///
    /// # Type
    /// - `&'a D1Database` -> A reference to the D1Database instance.
    db: D1Database,
}

impl ChatMessageRepository {
    /// Returns a fresh instance of `ChatMessageRepository` struct.
    ///
    /// # Arguments
    ///
    /// - `db` -> Database service to execute queries.
    pub fn new(db: D1Database) -> Self {
        ChatMessageRepository { db }
    }

    /// Deletes all messages in a specific chat by its ID.
    ///
    /// # Arguments
    ///
    /// - `chat_id` -> Identifier of the chat whose messages are to be deleted.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the operation is successful.
    /// - `Err(DatabaseQueryError<ChatMessage>)` if an error occurs during the operation.
    ///
    /// # Example
    /// ```rust
    ///   let result = chat_message_repository.delete_all_messages_in_chat("chat123").await;
    ///   match result {
    ///      Ok(_) => println!("All messages in chat deleted successfully."),
    ///      Err(e) => eprintln!("Error deleting messages: {}", e),
    ///   }
    /// ```
    pub async fn delete_all_messages_in_chat(
        &self,
        chat_id: &str,
    ) -> Result<(), Box<dyn ApplicationError>> {
        let query_result = self
            .db
            .prepare("DELETE FROM chat_messages WHERE chat_id = ?;")
            .bind(&[JsValue::from(chat_id)])
            .unwrap()
            .run()
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Deletes all messages from the `chat_messages` table.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the operation is successful.
    /// - `Err(DatabaseQueryError<ChatMessage>)` if an error occurs during the
    /// operation.
    ///
    /// # Example
    /// ```rust
    ///  let result = chat_message_repository.delete_all_messages().await;
    ///  match result {
    ///    Ok(_) => println!("All messages deleted successfully."),
    ///    Err(e) => eprintln!("Error deleting messages: {}", e),
    ///  }
    /// ```
    ///
    pub async fn delete_all_messages(&self) -> Result<(), Box<dyn ApplicationError>> {
        let query_result = self
            .db
            .prepare("DELETE FROM chat_messages;")
            .bind(&[])
            .unwrap()
            .run()
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Deletes a message from the `chat_messages` table by its ID.
    ///
    /// # Arguments
    ///
    /// - `message_id` -> Identifier of the message to be deleted.
    ///
    /// # Returns
    ///
    /// - `Ok(ChatMessage)` containing the deleted message if the operation is successful.
    /// - `Err(DatabaseQueryError<ChatMessage>)` if an error occurs during the
    pub async fn delete_message_by_id(
        &self,
        message_id: &str,
    ) -> Result<ChatMessage, Box<dyn ApplicationError>> {
        let query_result = match self
            .db
            .prepare("DELETE FROM chat_messages WHERE id = ? RETURNING *;")
            .bind(&[JsValue::from(message_id)])
        {
            Ok(prepared) => prepared.first::<ChatMessage>(None).await,
            Err(err) => {
                return Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                    err.to_string(),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR,
                )));
            }
        };

        match query_result {
            Ok(returned_data) => match returned_data {
                Some(message) => Ok(message),
                None => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                    "Message not found".to_string(),
                    None,
                    StatusCode::NOT_FOUND,
                ))),
            },
            Err(err) => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Retrieves all messages in a specific chat by its ID.
    ///
    /// # Arguments
    ///
    /// - `chat_id` -> Identifier of the chat whose messages are to be retrieved.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<ChatMessage>)` containing the list of messages if the operation is successful.
    /// - `Err(DatabaseQueryError<ChatMessage>)` if an error occurs during the
    /// operation.
    ///
    /// # Example
    /// ```rust
    ///  let result = chat_message_repository.get_all_messages_in_chat("chat123").await
    ///  match result {
    ///     Ok(messages) => println!("Retrieved {} messages.", messages.len()),
    ///     Err(e) => eprintln!("Error retrieving messages: {}", e),
    ///  }
    ///  ```
    pub async fn get_all_messages_in_chat(
        &self,
        chat_id: &str,
    ) -> Result<Vec<ChatMessage>, Box<dyn ApplicationError>> {
        let query = "SELECT * FROM chat_messages WHERE chat_id = ? ORDER BY created_at ASC;";
        let params = vec![JsValue::from(chat_id)];

        let query_result = self.db.prepare(query).bind(&params).unwrap().all().await;

        match query_result {
            Ok(fetched_messages) => {
                let messages = match fetched_messages.results::<ChatMessage>() {
                    Ok(msgs) => msgs,
                    Err(err) => {
                        return Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                            err.to_string(),
                            None,
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )));
                    }
                };
                Ok(messages)
            }
            Err(err) => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Retrieves a message from the `chat_messages` table by its ID.
    ///
    /// # Arguments
    ///
    /// - `message_id` -> Identifier of the message to be retrieved.
    ///
    /// # Returns
    ///
    /// - `Ok(ChatMessage)` containing the message if found.
    /// - `Err(DatabaseQueryError<ChatMessage>)` if the message is not found or if an error occurs during the operation.
    ///
    /// # Example
    /// ```rust
    /// let result = chat_message_repository.get_message_by_id("message123").await;
    /// match result {
    ///    Ok(message) => println!("Retrieved message: {:?}", message),
    ///    Err(e) => eprintln!("Error retrieving message: {}", e),
    /// }
    /// ```
    pub async fn get_message_by_id(
        &self,
        message_id: &str,
    ) -> Result<ChatMessage, Box<dyn ApplicationError>> {
        let query_result = match self
            .db
            .prepare("SELECT * FROM chat_messages WHERE id = ?;")
            .bind(&[JsValue::from(message_id)])
        {
            Ok(prepared) => prepared.first::<ChatMessage>(None).await,
            Err(err) => {
                return Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                    err.to_string(),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR,
                )));
            }
        };

        match query_result {
            Ok(fetched_message) => match fetched_message {
                Some(message) => Ok(message),
                None => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                    format!(
                        "The chat message with the id ['{}'] couldn't be found!",
                        message_id
                    ),
                    None,
                    StatusCode::NOT_FOUND,
                ))),
            },
            Err(err) => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Uses all necessary data and adds a new 'ChatMessage' entity to the database.
    ///
    /// # Arguments
    ///
    /// * `message` - A `ChatMessage` instance that holds all important data.
    ///
    /// # Returns
    ///
    /// - The ealier created `ChatMessage` object in the database, when everything went well.
    /// - A `DatabaseQueryError<ChatMessage>` error object in the case something happens.
    pub async fn save_message(
        &self,
        message: &ChatMessage,
    ) -> Result<ChatMessage, Box<dyn ApplicationError>> {
        let query_result = match self.db.prepare("INSERT INTO chat_messages (id, player_id, content, sent_at, chat_id) VALUES (1?, 2?, 3?, 4?, 5?) RETURNING *;")
            .bind(&[
                JsValue::from(&message.id),
                JsValue::from(&message.player_id),
                JsValue::from(&message.content),
                JsValue::from(&message.sent_at),
                JsValue::from(&message.chat_id),
            ]) {
            Ok(prepared) => prepared.first::<ChatMessage>(None).await,
            Err(err) => {
                return Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                    err.to_string(),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR,
                )));
            }
        };

        match query_result {
            Ok(returned_message) => match returned_message {
                Some(message) => Ok(message),
                None => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                    format!(
                        "Failed to add the chat message with ID ['{}'] to the database!",
                        message.id
                    ),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))),
            },
            Err(err) => Err(Box::new(DatabaseQueryError::<ChatMessage>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }
}

    }
}
