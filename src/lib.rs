use std::f32::consts::PI;

#[macro_use]
mod overload;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angle(f32);

overload!(self: Angle, + -);

#[allow(non_upper_case_globals)]
pub const rad: fn(f32) -> Angle = |radians| Angle(radians);
#[allow(non_upper_case_globals)]
pub const deg: fn(f32) -> Angle = |degrees| Angle(degrees / 180.0 * PI);
#[allow(non_upper_case_globals)]
pub const turn: fn(f32) -> Angle = |turns| Angle(turns * 2.0 * PI);


#[cfg(test)]
mod tests {
    use crate::{deg, rad, turn};

    #[test]
    fn test() {
        let a = turn(1.0/3.0);
        let b = deg(120.0);
        let c = rad(1.0);

        let s = c + a;

        assert!(s == c + b);
    }
}