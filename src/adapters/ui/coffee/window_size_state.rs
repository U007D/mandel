#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum WindowSizeState {
    Minimized,
    IntermediateNotResizable,
    IntermediateResizable,
    Maximized,
    FullScreen,
}
