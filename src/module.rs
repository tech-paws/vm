//! Module interface.

/// Module interface.
pub trait Module {
    /// Initialize module, e.g. run process or server
    fn init(&mut self);

    /// Shutdown module, e.g. stop process, or stop server, free resources
    fn shutdown(&mut self);

    /// Progress, put here some computations
    fn step(&mut self);

    /// Rendering
    fn render(&mut self);
}
