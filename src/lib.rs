extern crate cairo;

use std::error::Error;
use std::fs::File;
use cairo::{Context, ImageSurface, Format};

// I don't think we really need full doubles but the cairo api uses them

#[derive(Debug, Copy, Clone)]
pub enum Geometry {
    Rect {
        width: f64,
        height: f64,
    },
    Circle {
        radius: f64,
    },
}


#[derive(Debug, Copy, Clone)]
pub struct Paint {
    // FIXME: we probably want a third coordinate so things can be on top of others without
    // specifying draw order explicitly.
    pub translation: [f64;2],
    pub color: [f64;4],
    pub fill: bool,
    // rotation, line caps, line joins, other things...
    // FIXME: i think texture data should be here as well, rather than geometry. that way you can upload
    // it once and bind it to multiple geometries.
}

impl Default for Paint {
    fn default() -> Paint {
        Paint {
            translation: [0.0, 0.0],
            color: [0.0, 0.0, 0.0, 1.0],
            fill: false,
        }
    }
}

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
    geometries: Vec<(Geometry, Option<PaintHandle>)>,
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

    // FIXME: as mentioned above, add code for uploading data to gpu buffers here in gpu code
    pub fn add_geometry(&mut self, geom: Geometry) -> GeometryHandle {
        self.geometries.push((geom, None));
        return Handle { idx: self.geometries.len()-1 };
    }
    
    pub fn add_paint(&mut self, paint: Paint) -> PaintHandle {
        self.paints.push(paint);
        return Handle { idx: self.paints.len()-1 };
    }

    pub fn set_paint(&mut self, geometry_handle: GeometryHandle, paint_handle: PaintHandle) {
        // FIXME: return Result instead of panic?
        // also these asserts give less information than just letting the indices panic
        assert!(paint_handle.idx < self.paints.len(), "Paint out of bounds");
        assert!(geometry_handle.idx < self.geometries.len(), "Geometry out of bounds");

        self.geometries[geometry_handle.idx].1 = Some(paint_handle);
    }

    pub fn render(&mut self) -> Result<(), Box<Error>> {
        // in a GPU renderer we'd sort by geometry type or shader type or whatever and then just
        // say like "render all squares, render all circles, etc"
        // maybe just have an ubershader that does everything.

        let default_paint = Default::default();
        for (g, maybe_handle) in &self.geometries {
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

    /// Draw a rectangle centered at the paint's transform coordinates with the given `width` and
    /// `height`.
    fn render_rect(&self, width: f64, height: f64, paint: &Paint ) {
        let w2 = width/2.0;
        let h2 = height/2.0;
        let x = paint.translation[0] - h2;
        let y = paint.translation[1] - w2;

        let c = &paint.color;
        self.context.set_source_rgba(c[0], c[1], c[2], c[3]);

        self.context.rectangle(x, y, width, height); 

        if paint.fill {
            self.context.fill();
        }
        else {
            self.context.stroke();
        }
        //self.context.paint();
    }

    /// Draw a circle centered at the `paint`'s transform coordinates with the given radius.
    fn render_circle(&self, radius: f64, paint: &Paint ) {
        let c = &paint.color;
        self.context.set_source_rgba(c[0], c[1], c[2], c[3]);

        use std::f64::consts::PI;
        self.context.arc(paint.translation[0], paint.translation[1], radius, 0.0, 2.0*PI);

        if paint.fill {
            self.context.fill();
        }
        else {
            self.context.stroke();
        }
    }
}
