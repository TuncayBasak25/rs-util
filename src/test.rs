

macro_rules! return_if_zero {
	($binding:ident) => {
		if $binding.is_zero() {
            return;
        }
        $binding = $binding.unchecked_as_non_zero();
	};
}

pub enum Number {
	Real(Scalar),
    Negative(Negative),
    Positive(Positive),
    NonZeroNegative(NonZeroNegative),
    NonZeroPositive(NonZeroPositive),
	Zero(Zero),
	NonZero(NonZero),
}

pub struct V2(Number, Number);

impl Scalar {
	pub const fn unchecked_as_non_zero(self) -> NonZero {
        NonZero(self.0)
	}

    pub fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Zero(f32);
auto_overload!(self: Zero, rhs: NonZero, output: NonZero, +-);
auto_overload!(self: Zero, rhs: Scalar, output: Scalar, +-);

#[derive(Debug, Clone, Copy)]
pub struct Scalar(f32);
auto_overload!(self: Scalar, +-*);

#[derive(Debug, Clone, Copy)]
pub struct NonZero(f32);
auto_overload!(self: NonZero, */);
auto_overload!(self: NonZero, rhs: NonZero, output: Scalar, +-);
auto_overload!(self: Scalar, rhs: NonZero, +-*/);


#[cfg(test)]
mod tests {

    use crate::{NonZero, Scalar};

    #[test]
    fn test() {
        let a = Scalar(2.0);
        let b = Scalar(5.0);

        return_if_zero!(b);

		let b = b * b;

        assert!(b.0 == 25.0);
    }
}