#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scalar(f32);
overload!(Scalar, +-*);
overload!(PartialEq Scalar, NonZero Zero Negative Positive NonZeroNegative NonZeroPositive);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonZero(f32);
overload!(Scalar, rhs: NonZero, +-*/);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Zero(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Negative(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Positive(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonZeroNegative(f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonZeroPositive(f32);

#[cfg(test)]
mod tests {
    use super::{NonZero, Scalar};

    #[test]
    fn test() {
		let a = Scalar(2.0);
		let b = NonZero(2.0);

		assert!(b == a);
    }
}