use crate::{ConversationInfo, Database, ManagerError};
use csmlinterpreter::data::Client;

pub fn delete_state_key(
    _client: &Client,
    _type: &str,
    _key: &str,
    _db: &Database,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::state::delete_state_key as delete;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(_db)?;

        return delete(_client, _type, _key, db);
    }

    #[cfg(feature = "dynamo")]
    if cfg!(feature = "dynamo") {
        use crate::db_interactions::db_interactions_dynamo::state::delete_state_key as delete;
        use crate::db_interactions::db_interactions_dynamo::get_db;

        let db: &dynamodb::apis::client::APIClient = get_db(_db)?;

        return delete(_client, _type, _key, db);
    }


    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

// pub fn get_state_type(
//     client: &Client,
//     _type: &str,
//     db: &mongodb::Database,
// ) -> Result<mongodb::Cursor, Error> {
//     let state = db.collection("state");

//     let filter = doc! {
//         "client": bson::to_bson(client)?,
//         "type": _type,
//     };
//     let cursor = state.find(filter, None)?;

//     Ok(cursor)
// }

pub fn get_state_key(
    _client: &Client,
    _type: &str,
    _key: &str,
    _db: &Database,
) -> Result<Option<serde_json::Value>, ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::state::get_state_key;
        use crate::db_interactions::db_interactions_mongo::get_db;

        let db: &mongodb::Database = get_db(_db)?;

        return get_state_key(_client, _type, _key, db);
    }

    #[cfg(feature = "dynamo")]
    if cfg!(feature = "dynamo") {
        use crate::db_interactions::db_interactions_dynamo::state::get_state_key;
        use crate::db_interactions::db_interactions_dynamo::get_db;

        let db: &dynamodb::apis::client::APIClient = get_db(_db)?;

        return get_state_key(_client, _type, _key, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn set_state_items(
    _data: &mut ConversationInfo,
    _type: &str,
    _interaction_order: i32,
    _keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), ManagerError> {
    // Document
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") {
        use crate::db_interactions::db_interactions_mongo::state::set_state_items as set_items;

        return set_items(_data, _type, _keys_values);
    }

    #[cfg(feature = "dynamo")]
    if cfg!(feature = "dynamo") {
        use crate::db_interactions::db_interactions_dynamo::state::format_state_body;
        use crate::db_interactions::db_interactions_dynamo::state::set_state_items as set_items;

        use crate::db_interactions::db_interactions_dynamo::get_db;

        println!("format");
        let state_body = format_state_body(_data, _type, _interaction_order, _keys_values);
        let db: &dynamodb::apis::client::APIClient = get_db(&_data.db)?;

        println!("set_items");
        return set_items(&_data.client, state_body, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
