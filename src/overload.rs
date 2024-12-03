#[macro_export]
macro_rules! overload {
    (arith $self:ident, $rhs:ident, $output:ident, $trait:ident, $method:ident, $op:tt) => {
        impl std::ops::$trait<$rhs> for $self {
            type Output = $output;
            
            fn $method(self, rhs: $rhs) -> Self::Output {
                $output(self.0 $op rhs.0)
            }
        }
    };

    (self: $self:ident, rhs: $rhs:ident, output: $output:ident, op: +) => { overload!(arith $self, $rhs, $output, Add, add, +); };
    (self: $self:ident, rhs: $rhs:ident, output: $output:ident, op: -) => { overload!(arith $self, $rhs, $output, Sub, sub, -); };
    (self: $self:ident, rhs: $rhs:ident, output: $output:ident, op: *) => { overload!(arith $self, $rhs, $output, Mul, mul, *); };
    (self: $self:ident, rhs: $rhs:ident, output: $output:ident, op: /) => { overload!(arith $self, $rhs, $output, Div, div, /); };

    (self: $self:ident, rhs: $rhs:ident, output: $output:ident, $($op:tt)+) => {
		$(
			overload!(self: $self, rhs: $rhs, output: $output, op: $op);
		)*
    };

    (self: $self:ident, rhs: $rhs:ident, $($op:tt)+) => {
		$(
			overload!(self: $self, rhs: $rhs, output: $self, op: $op);
		)*
    };

    (self: $self:ident, $($op:tt)+) => {
		$(
			overload!(self: $self, rhs: $self, output: $self, op: $op);
		)*
    };
}