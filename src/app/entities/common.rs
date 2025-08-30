use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct EntityId(pub(crate) String);

impl From<String> for EntityId {
    fn from(value: String) -> Self {
        Self(value)
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
