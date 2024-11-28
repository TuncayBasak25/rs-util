#[derive(Debug, Clone, Copy)]
pub struct EntityId(pub(super) usize);

impl EntityId {
    pub(super) fn get(&self) -> usize {
        self.0
    }
}