use crate::{consts::msg, ports::ui::Size};
use coffee::Error as CoffeeError;
use thiserror::Error;

/// In an application setting, an `Error` is an `enum` (as opposed to a newtype in a library
/// context).  This simplifies its definition and use, as it is not going to be consumed externally.
#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "{}: {}: {:?} {}: {:?}",
        msg::ERR_ADAPTER_UI_WINDOW_TOO_LARGE,
        msg::DIMENSIONS,
        0,
        msg::CAUSED_BY,
        1
    )]
    AdapterUiIcedScreenDimsTooLarge(Size<usize>, core::num::TryFromIntError),
    #[error("{}; {}: {:?}", msg::ERR_CONVERSION_OVERFLOW, msg::CAUSED_BY, 0)]
    ConversionOverflow(#[from] core::num::TryFromIntError),
    #[error("{}", msg::ERR_INTERNAL_APP_RETURNED_UNEXPECTEDLY)]
    AppReturnedUnexpectedly(#[from] CoffeeError),
}

// Required by error-check operator when converting `Result`s from `TryFrom` and `TryInto`
impl From<core::convert::Infallible> for Error {
    fn from(_: core::convert::Infallible) -> Self {
        unreachable!()
    }
}
