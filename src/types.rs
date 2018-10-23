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
