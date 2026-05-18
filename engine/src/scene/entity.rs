use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SceneObjectId(pub String);

impl SceneObjectId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }
}
