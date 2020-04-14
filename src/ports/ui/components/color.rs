use num::Num;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Color<N: Num>(pub N, pub N, pub N, pub N);
