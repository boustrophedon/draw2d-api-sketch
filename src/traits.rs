use super::{Geometry, Paint};

pub trait Renderer {
    type Error;
    type GeometryHandle;
    type PaintHandle;

    /// Add a piece of geometry to the renderer.
    fn add_geometry(&mut self, geometry: Geometry) -> Result<Self::GeometryHandle, Self::Error>;
    /// Add a paint to the renderer.
    fn add_paint(&mut self, paint: Paint) -> Result<Self::PaintHandle, Self::Error>;
    /// Associate a paint with a geometry.
    fn set_paint(&mut self, geometry_h: Self::GeometryHandle, paint_h: Self::PaintHandle) -> Result<(), Self::Error>;
    /// Render all geometry added to the renderer.
    fn render(&mut self) -> Result<(), Self::Error>;
}
