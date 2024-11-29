pub struct Entity(usize, Flag);

impl Entity {
    pub(super) fn new(id: usize) -> Self {
        Self(id, Flag::NULL)
    }

    pub fn id(&self) -> usize {
        self.0
    }
}