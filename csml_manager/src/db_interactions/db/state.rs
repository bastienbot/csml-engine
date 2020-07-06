use crate::{ConversationInfo, DbInfo, ManagerError};
use csmlinterpreter::data::Client;

pub fn delete_state_key(
    client: &Client,
    _type: &str,
    key: &str,
    db: &DbInfo,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && db.db_type == "mongodb" {
        use crate::db_interactions::db_interactions_mongo::get_db;
        use crate::db_interactions::db_interactions_mongo::state::delete_state_key as delete;

        let db: &mongodb::Database = get_db(db)?;

        return delete(client, _type, key, db);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::get_db;
        use crate::db_interactions::db_interactions_http_db::state::delete_state_key as delete;

        let db: &http_db::apis::client::APIClient = get_db(db)?;

        return delete(client, _type, key, db);
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
    client: &Client,
    _type: &str,
    key: &str,
    db: &DbInfo,
) -> Result<Option<serde_json::Value>, ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && db.db_type == "mongodb" {
        use crate::db_interactions::db_interactions_mongo::get_db;
        use crate::db_interactions::db_interactions_mongo::state::get_state_key;

        let db: &mongodb::Database = get_db(db)?;

        return get_state_key(client, _type, key, db);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::get_db;
        use crate::db_interactions::db_interactions_http_db::state::get_state_key;

        let db: &http_db::apis::client::APIClient = get_db(db)?;

        return get_state_key(client, _type, key, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}

pub fn set_state_items(
    data: &mut ConversationInfo,
    _type: &str,
    interaction_order: i32,
    keys_values: Vec<(&str, &serde_json::Value)>,
) -> Result<(), ManagerError> {
    // Document
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && data.db.db_type == "mongodb" {
        use crate::db_interactions::db_interactions_mongo::state::set_state_items as set_items;

        return set_items(data, _type, keys_values);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::state::format_state_body;
        use crate::db_interactions::db_interactions_http_db::state::set_state_items as set_items;

        use crate::db_interactions::db_interactions_http_db::get_db;

        let state_body = format_state_body(data, _type, interaction_order, keys_values);
        let db: &http_db::apis::client::APIClient = get_db(&data.db)?;

        return set_items(&data.client, state_body, db);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
