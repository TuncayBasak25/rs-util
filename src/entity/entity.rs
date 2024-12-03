use crate::{ecs::Flag, V2};

use super::EntityId;

#[derive(Debug, Clone)]
pub struct Entity {
    id: EntityId,
    pos: V2,
    speed: V2,
}

impl Entity {
    pub(super) fn create(id: usize) -> Self {
        Entity {
            id: EntityId(id),
            pos: V2::null(),
            speed: V2::null()
        }
    }

    pub(super) fn reset(&mut self) {
        self.pos = V2::null();
        self.speed = V2::null();
    }

    pub(super) fn real_id(&self) -> usize {
        self.id.0
    }

    pub fn id(&self) -> EntityId {
        self.id
    }
}
