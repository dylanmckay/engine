

pub mod gl;

/// Specifies which faces should be culled.
pub enum CullingMode
{
    /// Cull front-facing faces.
    Front,
    /// Cull back-facing faces.
    Back,
}
