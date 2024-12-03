use std::cell::Cell;

use crate::{HitBox, V2};

use super::{Entity, Flag};



pub struct EntityManager {
    count: Cell<usize>,
    entities: Cell<Vec<Entity>>,
    flags: Cell<Vec<Flag>>,

}