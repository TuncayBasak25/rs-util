#[derive(Debug, Clone, Copy)]
pub struct Component(usize);

impl Component {
    const NULL: Component = Component(0);
    const FREE: Component = Component(1);
    const POSITION: Component = Component(1 << 1);
    const VELOCITY: Component = Component(1 << 2);
    const ACCELERATION: Component = Component(1 << 3);
    const HITBOX: Component = Component(1 << 4);

    pub fn add_component(&mut self, component: Component) {
        self.0 |= component.0
    }

    pub fn remove_component(&mut self, component: Component) {
        self.0 &= !component.0
    }

    pub fn has_component(&self, component: Component) -> bool {
        self.0 & component.0 != 0
    }

    pub fn val(&self) -> usize {
        self.0
    }
}