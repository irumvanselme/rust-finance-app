use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utoipa::ToSchema;

#[derive(Debug, Default, PartialEq, Clone, Eq, Serialize, ToSchema)]
pub struct EntityId(pub(crate) String);

impl From<String> for EntityId {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for EntityId {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<i32> for EntityId {
    fn from(value: i32) -> Self {
        Self(value.to_string())
    }
}

impl From<i64> for EntityId {
    fn from(value: i64) -> Self {
        Self(value.to_string())
    }
}

impl From<&i32> for EntityId {
    fn from(value: &i32) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityRef<EntityT> {
    Id(EntityId),
    Value(EntityT),
}

impl<EntityT> From<EntityId> for EntityRef<EntityT> {
    fn from(value: EntityId) -> Self {
        Self::Id(value)
    }
}
