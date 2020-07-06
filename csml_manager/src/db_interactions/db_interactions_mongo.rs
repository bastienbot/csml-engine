pub mod conversation;
pub mod interactions;
pub mod memories;
pub mod messages;
pub mod nodes;
pub mod state;

use crate::{Database, DbInfo, ManagerError};

pub fn get_db<'a>(db: &'a DbInfo) -> Result<&'a mongodb::Database, ManagerError> {
    match db.db {
        Database::Mongo(ref mongo) => Ok(mongo),
        _ => Err(ManagerError::Manager("db is not init correctly".to_owned())),
    }
}
