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
impl_arith!(tuple_1 Scalar (+-*) Scalar => Scalar);

#[derive(Debug, Clone, Copy)]
pub struct Positive(f32);
impl_arith!(tuple_1 Positive (+*/) Positive => Positive);
impl_arith!(tuple_1 Positive - Positive => Scalar);
impl_arith!(tuple_1 Positive (+-*) Scalar <=> Scalar);
impl_arith!(tuple_1 Scalar (/) Positive => Scalar);

#[derive(Debug, Clone, Copy)]
pub struct Negative(f32);
impl_arith!(tuple_1 Negative (+) Negative => Negative);
impl_arith!(tuple_1 Negative (-) Negative => Scalar);
impl_arith!(tuple_1 Negative (*/) Negative => Positive);
impl_arith!(tuple_1 Negative (+-*) Scalar <=> Scalar);
impl_arith!(tuple_1 Scalar (/) Negative => Scalar);
impl_arith!(tuple_1 Negative (-) Positive => Negative);
impl_arith!(tuple_1 Negative (+) Positive <=> Scalar);
impl_arith!(tuple_1 Negative (*/) Positive <=> Negative);
impl_arith!(tuple_1 Positive (-) Negative => Positive);

#[derive(Debug, Clone, Copy)]
pub struct NonZero(f32);
impl_arith!(tuple_1 NonZero (+-) NonZero => Scalar);
impl_arith!(tuple_1 NonZero (*/) NonZero => NonZero);
impl_arith!(tuple_1 NonZero (+-*) Scalar <=> Scalar);
impl_arith!(tuple_1 Scalar (/) NonZero => Scalar);
impl_arith!(tuple_1 NonZero (+-) (Positive Negative) <=> Scalar);
impl_arith!(tuple_1 NonZero (*/) (Positive Negative) <=> NonZero);


#[derive(Debug, Clone, Copy)]
pub struct Zero(f32);



#[cfg(test)]
mod tests {
    use super::{NonZero, Scalar};


    #[test]
    fn test() {
      let a = Scalar(5.0);
      let b = NonZero(2.0);

      let c = a / b;

      assert!(c.0 == 2.5);
    }
}