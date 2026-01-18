use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestMessage{
    pub endpoint_id: String,
    pub request: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseMessage{
    pub response_code: String,
    pub response: String,
}