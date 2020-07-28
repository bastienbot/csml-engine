/*
 * CSML engine microservices
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MemoryModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "interaction_id")]
    pub interaction_id: String,
    #[serde(rename = "client")]
    pub client: crate::models::ClientModel,
    #[serde(rename = "memory_order")]
    pub memory_order: i32,
    #[serde(rename = "interaction_order")]
    pub interaction_order: i32,
    #[serde(rename = "flow_id")]
    pub flow_id: String,
    #[serde(rename = "step_id")]
    pub step_id: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: serde_json::Value,
    #[serde(rename = "private")]
    pub private: bool,
    #[serde(rename = "expires_at")]
    pub expires_at: Option<String>,
}

impl MemoryModel {
    pub fn new(
        id: String,
        interaction_id: String,
        client: crate::models::ClientModel,
        memory_order: i32,
        interaction_order: i32,
        flow_id: String,
        step_id: String,
        key: String,
        value: serde_json::Value,
        private: bool,
        expires_at: Option<String>,
    ) -> MemoryModel {
        MemoryModel {
            id,
            interaction_id,
            client,
            memory_order,
            interaction_order,
            flow_id,
            step_id,
            _type: None,
            key,
            value,
            private,
            expires_at,
        }
    }
}