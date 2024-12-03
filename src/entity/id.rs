#[derive(Debug, Clone, Copy)]
pub struct EntityId(pub(super) usize);

impl EntityId {
    pub(super) fn get(&self) -> usize {
        self.0
    }
}

macro_rules! test {
    ($T:ident) => {
        pub struct $T {
            x: f32,
            y: f32
        }
    };
}

test!(Yupi);