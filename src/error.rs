use crate::{consts::msg, ports::ui::RectSize};
use thiserror::Error;

/// In an application setting, an `Error` is an `enum` (as opposed to a newtype in a library
/// context).  This simplifies its definition and use, as it is not going to be consumed externally.
#[derive(Debug, Error)]
pub enum Error {
    #[error("{}: {:?}", msg::ERR_ADAPTER_UI_ICED_SCREEN_DIMS_TOO_LARGE, 0)]
    AdapterUiIcedScreenDimsTooLarge(RectSize),
    #[error("{}: {:?}", msg::ERR_FOREIGN_COFFEE, 0)]
    CoffeeForeignError(#[from] coffee::Error),
}
