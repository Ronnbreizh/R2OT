/// Contain the main application
/// Main window, input control etc
mod application;
/// Module containing the principal UI of the software
/// and who ask for tabs to draw themselves
pub mod ui;
/// Contain the sub application behavior
mod subapp;

pub use application::Application as R2ot;
pub use subapp::SubApp;