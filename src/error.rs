use crate::{consts::msg, ports::ui::RectSize};
use std::ffi::OsString;
use thiserror::Error;

/// In an application setting, an `Error` is an `enum` (as opposed to a newtype in a library
/// context).  This simplifies its definition and use, as it is not going to be consumed externally.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{}: {:?}", msg::ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8, 0)]
    ArgNotConvertibleToUtf8(/*#[from]*/ std::ffi::OsString),
    #[error("{}: {:?}", msg::ERR_ADAPTER_UI_ICED_SCREEN_DIMS_TOO_LARGE, 0)]
    AdapterUiIcedScreenDimsTooLarge(RectSize),
}

// TODO: Remove this `From` impl and uncomment `#[from]` attribute above when
//       [`thiserror` issue](https://github.com/dtolnay/thiserror/issues/51) is resolved.
impl From<std::ffi::OsString> for Error {
    fn from(err: OsString) -> Self {
        Self::ArgNotConvertibleToUtf8(err)
    }
}
