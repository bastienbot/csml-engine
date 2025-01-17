use crate::data::csml_flow::CsmlFlow;
use crate::data::position::Position;
use crate::error_format::*;
use crate::Interval;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// DATA STRUCTURE
////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CsmlBot {
    pub id: String,
    pub name: String,
    pub fn_endpoint: Option<String>,
    pub flows: Vec<CsmlFlow>,
    pub default_flow: String,
}

////////////////////////////////////////////////////////////////////////////////
// STATIC FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl CsmlBot {
    pub fn new(
        id: &str,
        name: &str,
        fn_endpoint: Option<String>,
        flows: Vec<CsmlFlow>,
        default_flow: &str,
    ) -> Self {
        Self {
            id: id.to_owned(),
            name: name.to_owned(),
            fn_endpoint,
            flows,
            default_flow: default_flow.to_owned(),
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// METHOD FUNCTIONS
////////////////////////////////////////////////////////////////////////////////

impl CsmlBot {
    pub fn get_flow(&self, name: &str) -> Result<String, Vec<ErrorInfo>> {
        for flow in self.flows.iter() {
            if flow.name == name {
                return Ok(flow.content.to_owned());
            }
        }

        Err(vec![gen_error_info(
            Position::new(Interval::new_as_u32(0, 0)),
            format!("{} {}", ERROR_INVALID_FLOW, name),
        )])
    }
}
