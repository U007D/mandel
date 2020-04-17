use num::Num;

#[derive(Clone, Debug, PartialEq)]
pub struct Point<N: Num>(pub N, pub N);
