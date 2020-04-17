use num::Num;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color<N: Num>(pub N, pub N, pub N, pub N);

impl<N: Num> Default for Color<N> {
    fn default() -> Self {
        Self(N::zero(), N::zero(), N::zero(), N::one())
    }
}
