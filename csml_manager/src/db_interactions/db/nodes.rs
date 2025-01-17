use crate::{ConversationInfo, ManagerError};

pub fn create_node(
    data: &mut ConversationInfo,
    nextflow: Option<String>,
    nextstep: Option<String>,
) -> Result<(), ManagerError> {
    #[cfg(feature = "mongo")]
    if cfg!(feature = "mongo") && std::env::var("ENGINE_DB_TYPE") != Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_mongo::nodes::create_node as create;

        return create(data, nextflow, nextstep);
    }

    #[cfg(feature = "http")]
    if cfg!(feature = "http") && std::env::var("ENGINE_DB_TYPE") == Ok("http".to_owned()) {
        use crate::db_interactions::db_interactions_http_db::nodes::create_node as create;

        return create(data, nextflow, nextstep);
    }

    Err(ManagerError::Manager("db is not init correctly".to_owned()))
}
