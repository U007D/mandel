use num_traits::Num;
use std::convert::TryFrom;

// Newtype required to implement sound `TryFrom`/`TryInto` conversion impls on tuples (orphan rules)
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pair<T>(pub T, pub T)
where
    T: Num + Copy;

impl<T> Pair<T>
where
    T: Num + Copy,
{
    pub fn as_tuple(&self) -> (T, T) {
        (self.0, self.1)
    }
}

impl<T> From<(T, T)> for Pair<T>
where
    T: Num + Copy,
{
    fn from((a, b): (T, T)) -> Self {
        Self(a, b)
    }
}

impl<T, U> TryFrom<Pair<U>> for (T, T)
where
    T: Num + TryFrom<U>,
    U: Num + Copy,
{
    type Error = <T as TryFrom<U>>::Error;

    fn try_from(pair: Pair<U>) -> Result<Self, Self::Error> {
        Ok((T::try_from(pair.0)?, T::try_from(pair.1)?))
    }
}
