extern crate cairo;

use std::error::Error;
use std::fs::File;
use cairo::{Context, ImageSurface, Format};

mod traits;
mod types;

pub use traits::Renderer;
pub use types::{Geometry, Paint};

// FIXME
// you can have problems with stale handles.
#[derive(Debug, Copy, Clone)]
pub struct Handle {
    idx: usize
}

type GeometryHandle = Handle;
type PaintHandle = Handle;

#[derive(Debug)]
pub struct CairoRenderer {
    surface: ImageSurface,
    context: Context,
    output_file: File,
    geometries: Vec<(Geometry, Option<Handle>)>,
    paints: Vec<Paint>,
    // in a gpu renderer we'd also have things that hold gpu buffers for the geometry and paint
    // data
}
impl CairoRenderer {
    pub fn new(output_file: File) -> CairoRenderer {
        let surface = ImageSurface::create(
            Format::ARgb32,
            1280,
            720,
        ).unwrap();

        let context = Context::new(&surface);
        // center coordinate system, with +x pointing left and +y pointing up
        context.translate(1280.0/2.0, 720.0/2.0);
        context.scale(1.0, -1.0);
        CairoRenderer {
            surface,
            context,
            output_file,
            geometries: Vec::new(),
            paints: Vec::new(),
        }
    }

    /// Draw a rectangle centered at the paint's transform coordinates with the given `width` and
    /// `height`.
    fn render_rect(&self, width: f64, height: f64, paint: &Paint ) {
        let w2 = width/2.0;
        let h2 = height/2.0;
        let x = paint.translation[0];
        let y = paint.translation[1];

        let c = &paint.color;
        self.context.set_source_rgba(c[0], c[1], c[2], c[3]);

        self.context.move_to(x-w2, y-h2);
        self.context.rel_line_to(width, 0.0);
        self.context.rel_line_to(0.0, height);
        self.context.rel_line_to(-width, 0.0);
        self.context.close_path();

        if paint.fill {
            self.context.fill();
        }
        else {
            self.context.stroke();
        }
    }

    /// Draw a circle centered at the `paint`'s transform coordinates with the given radius.
    fn render_circle(&self, radius: f64, paint: &Paint ) {
        let c = &paint.color;
        self.context.set_source_rgba(c[0], c[1], c[2], c[3]);

        use std::f64::consts::PI;
        self.context.new_path();
        self.context.arc(paint.translation[0], paint.translation[1], radius, 0.0, 2.0*PI);
        self.context.close_path();

        if paint.fill {
            self.context.fill();
        }
        else {
            self.context.stroke();
        }
    }
}

impl Renderer for CairoRenderer {
    type Error = Box<Error>;
    type GeometryHandle = GeometryHandle;
    type PaintHandle = PaintHandle;

    fn add_geometry(&mut self, geom: Geometry) -> Result<GeometryHandle, Box<Error>> {
        self.geometries.push((geom, None));
        Ok(Handle { idx: self.geometries.len()-1 })
    }
    
    fn add_paint(&mut self, paint: Paint) -> Result<PaintHandle, Box<Error>> {
        self.paints.push(paint);
        Ok(Handle { idx: self.paints.len()-1 })
    }

    fn set_paint(&mut self, geometry_handle: GeometryHandle, paint_handle: PaintHandle) -> Result<(), Box<Error>> {
        // assert!(paint_handle.idx < self.paints.len(), "Paint out of bounds");
        // assert!(geometry_handle.idx < self.geometries.len(), "Geometry out of bounds");

        // FIXME: add actual errors for bad handles

        self.geometries[geometry_handle.idx].1 = Some(paint_handle);

        Ok(())
    }

    fn render(&mut self) -> Result<(), Box<Error>> {
        // in a GPU renderer we'd sort by geometry type or shader type or whatever and then just
        // say like "render all squares, render all circles, etc"
        // or maybe just have an ubershader that does everything.

        let default_paint = Default::default();
        for (g, maybe_handle) in &self.geometries {
            self.context.move_to(0.0, 0.0);
            let p = maybe_handle.map_or(&default_paint, |h| &self.paints[h.idx]);
            match *g {
                Geometry::Rect { width, height } => {
                    self.render_rect(width, height, p)
                },
                Geometry::Circle { radius } => {
                    self.render_circle(radius, p)}
                ,
            }
        }

        self.surface.write_to_png(&mut self.output_file)?;
        Ok(())
    }
}
