use crate::{consts::msg, ports::ui::Pair};
use thiserror::Error;

/// In an application setting, an `Error` is an `enum` (as opposed to a newtype in a library
/// context).  This simplifies its definition and use, as it is not going to be consumed externally.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{}: {:?}", msg::ERR_ADAPTER_UI_ICED_SCREEN_DIMS_TOO_LARGE, 0)]
    AdapterUiIcedScreenDimsTooLarge(Pair<usize>, core::num::TryFromIntError),
    // #[error("{}: {:?}", msg::ERR_ADAPTER_UI_ICED_UNSUPPORTED_WINDOW_STATE, 0)]
    // AdapterUiIcedUnsupportedWindowState(WindowState),
    #[error("{}; {}: {:?}", msg::ERR_CONVERSION_OVERFLOW, msg::CAUSED_BY, 0)]
    ConversionOverflow(#[from] core::num::TryFromIntError),
}

// Required by error-check operator when converting `Result`s from `TryFrom` and `TryInto`
impl From<core::convert::Infallible> for Error {
    fn from(_: core::convert::Infallible) -> Self {
        unreachable!()
    }
}
