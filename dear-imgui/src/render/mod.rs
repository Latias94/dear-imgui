//! Rendering system for Dear ImGui
//!
//! This module provides the core rendering functionality, including draw data
//! management and renderer abstractions.

pub mod draw_data;
pub mod renderer;

// Re-export commonly used types
pub use draw_data::*;
pub use renderer::*;
