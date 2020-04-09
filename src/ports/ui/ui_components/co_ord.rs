use num::Num;

#[derive(Clone, Debug, PartialEq)]
pub struct CoOrd<N: Num>(pub N, pub N);
