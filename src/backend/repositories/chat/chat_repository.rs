use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
use axum::http::StatusCode;
use wasm_bindgen::JsValue;
use worker::D1Database;

use crate::backend::{
    errors::{application_error::ApplicationError, database_query_error::DatabaseQueryError, process_error::ProcessError}, repositories::chat::chat_message_repository::ChatMessageRepository, types::chat::{Chat, ChatMessage}
};

/// A database repository for interacting with the `chats` table.
///
/// Contains the utility functions for the `Chat` struct.
///
/// It will be accessable in the context element in the handler functions.
///
pub struct ChatRepository {
    /// Database service pointer to execute queries.
    ///
    /// # Type
    /// - `D1Database` -> D1Database instance to interact with the `chats` table.
    db: D1Database,
}

impl ChatRepository {
    /// Uses the global `D1Database` service by referencing it.
    ///
    /// # Returns
    ///
    /// A new instantiated `ChatRepository` object.
    pub fn new(db: D1Database) -> Self {
        ChatRepository { db }
    }

    /// Creates a new instance of a `Chat` struct in the database.
    ///
    /// # Arguments
    ///
    /// - **chat** -> The `Chat` object that holds necessary data to create the entry in the
    /// database.
    ///
    /// # Returns
    ///
    /// => Returned data from the database query as a `Chat` object WHEN the query is successful.
    /// => Returns an error as a `DatabaseQueryError<Chat>` which implements the `ApplicationError` trait WHEN any issue occurs.
    pub async fn create_chat(&self, chat: Chat) -> Result<Chat, impl ApplicationError> {
        let insertion_result = match self.db.prepare("INSERT INTO chats (id, number_of_messages, game_id) VALUES (1?, 2? ,3?) RETURNING *;").bind(&[
            JsValue::from(chat.id.clone()),
            JsValue::from(chat.number_of_messages),
            JsValue::from(chat.game_id),
        ]) {
            Ok(query_context) => query_context.first::<Chat>(None).await,
            Err(err) => return Err(DatabaseQueryError::<Chat>::new(err.to_string(), None, StatusCode::INTERNAL_SERVER_ERROR))
        };

        match insertion_result {
            Ok(fetched_chat) => match fetched_chat {
                Some(chat) => Ok(chat),
                None => {
                    return Err(DatabaseQueryError::new(
                        format!(
                            "Attempt to save the chat object with id ['{}'] failed!",
                            chat.id
                        ),
                        None,
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            },
            Err(error) => {
                return Err(DatabaseQueryError::new(
                    error.to_string(),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        }
    }

    /// Takes in either a `chat_id` or a `game_id` arguments, when both are passed only the
    /// `game_id` will determine after which column an entry in the database will be deleted.
    ///
    /// Then removes the specific instance from the database.
    ///
    /// # Arguments
    ///
    /// - `chat_id` -> Identifier of the `Chat` object.
    /// - `game_id` -> `Game` and `Chat` entries have a 1 : 1 relation, that's why a `Chat` entry
    /// can be deleted by using the identifier of the game it is assigned to.
    ///
    /// # Returns
    ///
    /// => Ok(()) WHEN the removal of the `Chat` object was successful.
    /// => Err(impl ApplicationError) WHEN any issue is being faced.
    pub async fn delete_chat(
        &self,
        chat_id: Option<&str>,
        game_id: Option<&str>,
    ) -> Result<(), impl ApplicationError> {
        // check if the a game id was passed and then if a chat id is available
        let mut query_string = "DELETE FROM chats WHERE ".to_string();
        let mut query_bindings: Vec<JsValue> = vec![];

        if let Some(received_game_id) = game_id {
            query_string.push_str("game_id = ?;");
            query_bindings.push(JsValue::from(received_game_id));
        } else if let Some(received_chat_id) = chat_id {
            query_string.push_str("id = ?;");
            query_bindings.push(JsValue::from(received_chat_id));
        } else {
            return Err(DatabaseQueryError::<Chat>::new(
                "An invalid function input was passed to the 'delete_chat' method! Either pass the 'chat_id' or 'game_id' argument after which a 'Chat' entry will be deleted!".to_string(),
                None,
                StatusCode::BAD_REQUEST
            ));
        }

        // execute the required query
        let deletion_result = match self.db.prepare(query_string).bind(&query_bindings) {
            Ok(prep_query) => prep_query.run().await,
            Err(error) => return Err(DatabaseQueryError::new(error.to_string(), None, StatusCode::INTERNAL_SERVER_ERROR))
        };

        match deletion_result {
            Ok(_) => Ok(()),
            Err(error) => Err(DatabaseQueryError::new(error.to_string(), None, StatusCode::INTERNAL_SERVER_ERROR))
        }
    }

    /// A *mutation* method which should update a `Chat` object.
    ///
    /// Saves a new `ChatMessage` to the `Chat` object and updates the ***`number_of_messages`***
    /// field of the `Chat` instance.
    ///
    /// # Arguments
    ///
    /// - `chat_id` => A mandatory `id` of the `Chat` instance.
    /// - `chat_message` => The new `ChatMessage` object that is going to be stored in the
    /// database.
    /// - `chat_message_repo` => The database repository which provides functionality to interact
    /// with the `chat_messages` table.
    ///
    /// # Returns
    ///
    /// 1.) Ok(ChatMessage), WHEN all queries went well and the final operation returned the same
    ///   stored `ChatMessage` object.
    pub async fn add_new_message_to_chat(
        &self,
        chat_id: &str,
        chat_message: ChatMessage,
        chat_message_repo: &ChatMessageRepository,
    ) -> Result<ChatMessage, Box<dyn ApplicationError>> {
        // get the 'number_of_messages' and increment by 1
        let updated_num_of_mess = match self.get_number_of_messages_of_chat(None, Some(chat_id)).await {
            Ok(number_of_messages) => number_of_messages + 1,
            Err(err) => {
                return Err(err);
            }
        };

        // update the chat instance with the new number_of_messages
        let modification_result = match self.db.prepare("UPDATE chats SET number_of_messages = 1? WHERE id = 2? RETURNING *;").bind(&[
            JsValue::from(updated_num_of_mess), JsValue::from(chat_id)
        ]) {
            Ok(returned_data) => returned_data.first::<Chat>(None).await,
            Err(err) => {
                return Err(Box::new(DatabaseQueryError::<Chat>::new(err.to_string(), None, StatusCode::INTERNAL_SERVER_ERROR)));
            }
        };

        // just check if an error occured in the result data progression
        match modification_result {
            Ok(_) => (),
            Err(err) => {
                return Err(Box::new(ProcessError::<Chat>::new(err.to_string(), "ChatRepository::add_new_message_to_chat".to_string(), None)));
            }
        };

        // create new 'ChatMessage' instance
        let chat_message_insertion_result = match chat_message_repo.save_message(&chat_message).await {
            Ok(message) => message,
            Err(err) => {
                return Err(err)
            }
        };

        Ok(chat_message_insertion_result)
    }

    /// A ***modification*** method to udpate the `number_of_messages` property of a `Chat` struct
    /// after the `ChatMessage` has been deleted.
    ///
    /// # Arguments
    ///
    /// -> `chat_id` => ID of the `Chat` object
    /// -> `message_id` => ID of the `ChatMessage` instance
    /// -> `chat_message_repo` => The `ChatMessageRepository` for the `chat_messages` table.
    ///
    /// # Returns
    ///
    /// 1.) `Ok(ChatMessage)`, WHEN the `Chat` has been updated and the `ChatMessage` was deleted.
    /// 2.) `Err(Box<dyn ApplicationError>)`, WHEN in the process of operation a problem occurs.
    pub async fn remove_message_from_chat(
        &self,
        chat_id: &str,
        message_id: &str,
        chat_message_repo: &ChatMessageRepository
    ) -> Result<ChatMessage, Box<dyn ApplicationError>> {
        // update the 'number_of_messages' -> decrement by one
        let nr_of_mes_fetch_result = match self.get_number_of_messages_of_chat(None, Some(chat_id)).await {
            Ok(res) => {
                if res == 0 { return Err(Box::new(ProcessError::<ChatMessage>::new(format!("The number of messages of the chat with the id ['{}'] can't be negative!", chat_id), "ChatRepository::remove_message_from_chat".to_string(), None)));
                }
                res - 1
            },
            Err(err) => return Err(err)
        };

        // update the 'number_of_messages'
        let modification_result = match self.update_number_of_messages_of_chat(nr_of_mes_fetch_result, Some(chat_id), None).await {
            Ok(new_nr) => new_nr,
            Err(err) => return Err(err)
        };

        if modification_result != nr_of_mes_fetch_result {
            return Err(Box::new(ProcessError::<ChatMessage>::new("The 'number_of_messages' property hasn't changed after a successful operation!".to_string(), "ChatRepository::remove_message_from_chat".to_string(), None)))
        }

        // remove the 'ChatMessage' from the 'Chat' queue
        let removal_result = match chat_message_repo.delete_message_by_id(message_id).await {
            Ok(deleted_message) => deleted_message,
            Err(err) => return Err(err)
        };

        Ok(removal_result)
    }

    /// A ***modification*** method to update the `number_of_messages` column of a `Chat` entry in
    /// the `chats` table.
    ///
    /// # Arguments
    ///
    /// -> `updated_number_of_messages` => The changed number of the `ChatMessages` in the `Chat`
    /// itself.
    /// -> `chat_id` => The identifier of the `Chat` object.
    /// -> `game_id` => ID of the `Game` instance which holds the `Chat` object.
    ///
    /// # Returns
    ///
    /// 1.) Ok(usize), WHEN the operation was sucessful.
    /// 2.) Err(Box<dyn ApplicationError>), WHEN any kind of issue occurs.
    pub async fn update_number_of_messages_of_chat(&self, updated_number_of_messages: usize, chat_id: Option<&str>, game_id: Option<&str>) -> Result<usize, Box<dyn ApplicationError>> {
        // temporary variables
        let mut query_string = "UPDATE chats SET number_of_messages = 1? WHERE".to_string();
        let mut query_bindings: Vec<JsValue> = vec![JsValue::from(updated_number_of_messages)];

        // filter the selection arguments
        if let Some(recv_game_id) = game_id {
            query_string.push_str(" game_id = 2? ");
            query_bindings.push(JsValue::from(recv_game_id));
        } else if let Some(recv_chat_id) = chat_id {
            query_string.push_str(" id = 2? ");
            query_bindings.push(JsValue::from(recv_chat_id));
        } else {
            return Err(Box::new(ProcessError::<ChatMessage>::new("An invalid data input was passed to the 'update_number_of_messages_of_chat' function! At least pass either 'chat_id' or 'game_id'!".to_string(), "ChatRepository::update_number_of_messages_of_chat".to_string(), None)));
        }

        query_string.push_str("RETURNING number_of_messages;");

        // execute query
        let modification_result = match self.db.prepare(query_string).bind(&query_bindings) {
            Ok(result) => result.first::<usize>(None).await,
            Err(err) => return Err(Box::new(DatabaseQueryError::<Chat>::new(err.to_string(), None, StatusCode::INTERNAL_SERVER_ERROR)))

        };

        // unwrap the result
        match modification_result {
            Ok(successful_result) => match successful_result {
                Some(new_number) => Ok(new_number),
                None => return Err(Box::new(ProcessError::<Chat>::new(match chat_id {
                    Some(id) => format!("The 'Chat' object with the id ['{}'] couldn't be found, therefore the 'number_of_messages' couldn't be updated!", id),
                    None => match game_id {
                        Some(id) => format!("The 'Chat' object belonging to the game with the id ['{}'] couldn't be found, therefore the 'number_of_messages' couldn't be updated!", id),
                        None => format!("The 'Chat' instance couldn't be found!")
                    }
                }, "ChatRepository::update_number_of_messages_of_chat".to_string(), None)))
            },
            Err(err) => return Err(Box::new(ProcessError::<Chat>::new(err.to_string(), "ChatRepository::update_number_of_messages_of_chat".to_string(), None)))
        }
    }

    /// Fetches the `number_of_messages` property of a `Chat` struct by either using the `game_id` or `chat_id` argument.
    /// Here by is the `game_id` argument prefered.
    ///
    /// # Arguments
    ///
    /// - ***`game_id`*** => The identifier of the `Game` struct that holds the `Chat` as
    /// property.
    /// - ***`chat_id`*** => The own identifier of the `Chat` instance.
    ///
    /// # Returns
    ///
    /// => 1.) Ok(usize), WHEN the data could be fetched.
    /// => 2.) Err(Box<impl ApplicationError>), WHEN any kind of error occurs.
    pub async fn get_number_of_messages_of_chat(&self, game_id: Option<&str>, chat_id: Option<&str>) -> Result<usize, Box<dyn ApplicationError>> {
        let mut query_string = "SELECT number_of_messages FROM chats WHERE ".to_string();
        let mut query_bindings: Vec<JsValue> = vec![];

        // filtering after the arguments passed to the method
        if let Some(recv_game_id) = game_id {
            query_string.push_str("game_id = ?;");
            query_bindings.push(JsValue::from(recv_game_id));
        } else if let Some(recv_chat_id) = chat_id {
            query_string.push_str("id = ?;");
            query_bindings.push(JsValue::from(recv_chat_id));
        } else {
            return Err(Box::new(ProcessError::<Chat>::new("Invalid data input! Atleast provide one argument like 'game_id' or 'chat_id'!".to_string(), "ChatRepository::get_number_of_messages_of_chat".to_string(), None)));
        }

        let fetch_query_result = match self.db.prepare(query_string).bind(&query_bindings) {
            Ok(received_data) => received_data.first::<usize>(None).await,
            Err(err) => return Err(Box::new(DatabaseQueryError::<Chat>::new(err.to_string(), None, StatusCode::INTERNAL_SERVER_ERROR)))
        };

        match fetch_query_result {
            Ok(recv_data) => match recv_data {
                Some(number_of_messages) => Ok(number_of_messages),
                None => {
                    return Err(Box::new(DatabaseQueryError::<Chat>::new(match chat_id {
                        Some(id) => format!("The 'Chat' instance with the id ['{}'] couldn't be found in the database!", id),
                        None => match game_id {
                            Some(id) => format!("A 'Chat' entry with the game id ['{}'] wasn't in the database!", id),
                            None => "No 'Chat' object was found!".to_string()
                        }
                    }, None, StatusCode::NOT_FOUND)))
                }
            },
            Err(err) => {
                return Err(Box::new(ProcessError::<Chat>::new(format!("During extracting the 'number_of_messages' of a 'Chat' object from the returned data an error! Error: {}", err.to_string()), "ChatRepository::get_number_of_messages_of_chat".to_string(), None)));
            }
        }
    }


    /// Fetches a `Card` entry form the database.
    ///
    /// Decides if the chat will be queried after the `game_id` or `chat_id` arguments there are
    /// passed to the method. The `game_id` will be considered before the `chat_id`!
    ///
    /// # Arguments
    ///
    /// -> `game_id` => The identifier of the game the chat belongs to.
    /// -> `chat_id` => Identifier of the instance itself.
    /// -> `chat_message_repo` => The repository for the `ChatMessage` to interact with the
    ///     database.
    ///
    /// # Returns
    ///
    /// -> Ok(chat), WHEN all operations succeed and a `CHAT` instance was found.
    /// -> Err(Box(dyn ApplicationError)), WHEN any kind of issue occurs.
    ///
    pub async fn get_chat(&self, chat_id: Option<&str>, game_id: Option<&str>, chat_message_repo: &ChatMessageRepository) -> Result<Chat, Box<dyn ApplicationError>> {
        let mut query_string = "SELECT * FROM chats ".to_string();
        let mut query_bindings: Vec<JsValue> = vec![];

        // again first filter after the 'game_id' first
        if let Some(recv_game_id) = game_id {
            query_string.push_str("WHERE game_id = ?;");
            query_bindings.push(JsValue::from(recv_game_id));
        } else if let Some(recv_chat_id) = chat_id {
            query_string.push_str("WHERE chat_id = ?;");
            query_bindings.push(JsValue::from(recv_chat_id));
        }

        let fetch_query_result = match self.db.prepare(query_string).bind(&query_bindings) {
            Ok(returned_data) => returned_data.first::<Chat>(None).await,
            Err(error) => return Err(Box::new(DatabaseQueryError::<Chat>::new(error.to_string(), None, StatusCode::INTERNAL_SERVER_ERROR)))
        };

        match fetch_query_result {

            Ok(returned_chat) => match returned_chat {
                Some(mut chat) => {
                    // fetch all messages for the chat here
                    chat.messages = match chat_message_repo.get_all_messages_in_chat(&chat.id).await {
                        Ok(messages) => messages,
                        Err(error) => {
                            return Err(error);
                        }
                    };

                    Ok(chat)
                },
                None => {
                    return Err(Box::new(DatabaseQueryError::<Chat>::new(format!("Couldn't find a 'Chat' instance with a game id {:?} or id {:?}!", game_id, chat_id), None, StatusCode::NOT_FOUND)))
                }
            }
            Err(err) => {
                return Err(Box::new(ProcessError::<Chat>::new(err.to_string(), "ChatRepository::get_chat".to_string(),  None)))
            }
        }
    }
}
    }
}
