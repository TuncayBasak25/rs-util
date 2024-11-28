use std::usize;

use super::{EntityId, Entity};


pub struct EntityManager {
    count: usize,
    entities: Vec<Entity>,
    free_entities: Vec<usize>,
    used_entities: Vec<usize>
}

impl EntityManager {
    pub fn init() -> Self {
        Self {
            count: 0,
            entities: Vec::new(),
            free_entities: Vec::new(),
            used_entities: Vec::new()
        }
    }

    fn allocate_new(&mut self) -> Entity {
        let entity = Entity::create(self.count);
        self.entities.push(entity.clone());
        self.used_entities.push(self.count);
        self.count += 1;
        entity
    }

    pub fn create(&mut self) -> Entity {
        match self.free_entities.pop() {
            Some(free_id) => {
                self.used_entities[free_id] = free_id;
                self.entities[free_id].clone()
            },
            None => self.allocate_new(),
        }        
    }

    pub fn destroy(&mut self, mut entity: Entity) {
        let id = entity.real_id();
        if id == self.used_entities[id] {
            entity.reset();
            self.used_entities[id] = usize::MAX;
            self.free_entities.push(id);
            self.entities[id] = entity
        }
    }

    pub fn get(&self, id: EntityId) -> Option<Entity> {
        if id.get() == self.used_entities[id.get()] {
            Some(self.entities[id.get()].clone())
        }
        else {
            None
        }
    } 

    pub fn update(&mut self, entity: Entity) -> Option<Entity> {
        let id = entity.real_id();
        if id == self.used_entities[id] {
            self.entities[id] = entity.clone();
            Some(entity)
        }
        else {
            None
        }
    }

    pub fn get_all(&self) -> Vec<EntityId> {
        let mut all_entities = Vec::with_capacity(self.used_entities.capacity());
        for &id in &self.used_entities {
            all_entities.push(EntityId(id));
        }
        all_entities
    }
}


