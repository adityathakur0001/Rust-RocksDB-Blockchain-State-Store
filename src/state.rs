use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockState {
    pub height: u64,
    pub state_data: String,
}
