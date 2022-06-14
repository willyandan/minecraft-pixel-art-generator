use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub transparent: bool,
}
