//! Command-line options for xapp.

/// Operating mode
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Default mode
    Default,
    /// List apps mode
    ListApps,
    /// List windows mode
    ListWindows,
    /// Stats mode
    Stats,
}

/// Command-line options
#[derive(Debug, Clone)]
pub struct Options {
    /// Operating mode
    pub mode: Mode,
    /// Verbose output
    pub verbose: bool,
    /// Quiet mode
    pub quiet: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            mode: Mode::Default,
            verbose: false,
            quiet: false,
        }
    }
}
