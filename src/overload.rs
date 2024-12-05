use std::{num::NonZero, ops::Div};

macro_rules! impl_arith{
    (tuple_1 $self:ident $op:tt $rhs:ident => $output:ident $trait:ident $method:ident) => {
        impl std::ops::$trait<$rhs> for $self {
          type Output = $output;

          fn $method(self, rhs: $rhs) -> Self::Output {
            $output(self.0 $op rhs.0)
          }
        }
    };

    (tuple_1 $self:ident + $rhs:ident => $output:ident) => { impl_arith!(tuple_1 $self + $rhs => $output Add add); };
    (tuple_1 $self:ident - $rhs:ident => $output:ident) => { impl_arith!(tuple_1 $self - $rhs => $output Sub sub); };
    (tuple_1 $self:ident * $rhs:ident => $output:ident) => { impl_arith!(tuple_1 $self * $rhs => $output Mul mul); };
    (tuple_1 $self:ident / $rhs:ident => $output:ident) => { impl_arith!(tuple_1 $self / $rhs => $output Div div); };

    (tuple_1 $self:ident ($($op:tt)+) $rhs:ident => $output:ident) => {
      $(
        impl_arith!(tuple_1 $self $op $rhs => $output);
      )+
    };

    (tuple_1 $self:ident $op:tt ($($rhs:ident)+) => $output:ident) => {
      $(
        impl_arith!(tuple_1 $self $op $rhs => $output);
      )+
    };

    (tuple_1 $self:ident ($($op:tt)+) ($($rhs:ident)+) => $output:ident) => {
      $(
        impl_arith!(tuple_1 $self ($($op)+) $rhs => $output);
      )+
    };

    (tuple_1 $self:ident + $rhs:ident <=> $output:ident) => {
      impl_arith!(tuple_1 $self + $rhs => $output Add add);
      impl_arith!(tuple_1 $rhs + $self => $output Add add);
    };
    (tuple_1 $self:ident - $rhs:ident <=> $output:ident) => {
      impl_arith!(tuple_1 $self - $rhs => $output Sub sub);
      impl_arith!(tuple_1 $rhs - $self => $output Sub sub);
    };
    (tuple_1 $self:ident * $rhs:ident <=> $output:ident) => {
      impl_arith!(tuple_1 $self * $rhs => $output Mul mul);
      impl_arith!(tuple_1 $rhs * $self => $output Mul mul);
    };
    (tuple_1 $self:ident / $rhs:ident <=> $output:ident) => {
      impl_arith!(tuple_1 $self / $rhs => $output Div div);
      impl_arith!(tuple_1 $rhs / $self => $output Div div);
    };

    (tuple_1 $self:ident ($($op:tt)+) $rhs:ident <=> $output:ident) => {
      $(
        impl_arith!(tuple_1 $self $op $rhs <=> $output);
      )+
    };

    (tuple_1 $self:ident $op:tt ($($rhs:ident)+) <=> $output:ident) => {
      $(
        impl_arith!(tuple_1 $self $op $rhs <=> $output);
      )+
    };

    (tuple_1 $self:ident ($($op:tt)+) ($($rhs:ident)+) <=> $output:ident) => {
      $(
        impl_arith!(tuple_1 $self ($($op)+) $rhs <=> $output);
      )+
    };
}


#[derive(Debug, Clone, Copy)]
pub struct Scalar(f32);

pub struct PossiblyUndefined(f32);

impl Div<NonZero> for Scalar {
	type Output = Scalar;

	fn div(self, rhs: Self) -> Self::Output {
		Scalar(self.0 / rhs.0)
	}
}

impl PossiblyUndefined {
	pub fn trust_me(self) -> Scalar {
		Scalar(self.0)
	}

	pub fn check(self) -> Option<Scalar> {
		if self.0.is_infinite() {
			None
		}
		else {
			Some(Scalar(self.0))
		}
	}
}