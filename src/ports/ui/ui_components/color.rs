use num::Num;

#[derive(Clone, Debug, PartialEq)]
pub struct Color<N: Num>(pub N, pub N, pub N, pub N);
