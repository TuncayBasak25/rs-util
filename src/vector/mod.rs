#[derive(Debug, Clone, Copy)]
pub struct V2 {
    
};

impl V2 {
    pub fn null() -> Self {
        V2(0.0, 0.0)
    }

    pub fn x(&self) -> f32 {
        self.0
    }

    pub fn y(&self) -> f32 {
        self.1
    }
}