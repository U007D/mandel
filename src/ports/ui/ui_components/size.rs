use num::Num;

#[derive(Clone, Debug, PartialEq)]
pub struct Size<N: Num>(N, N);
