extern crate draw2d_sketch as draw2d;

use std::fs::File;
use draw2d::{CairoRenderer, Geometry, Paint};

fn main() {
    let mut renderer = CairoRenderer::new(File::create("/tmp/output.png").unwrap());

    let r1 = Geometry::Rect { width: 50.0, height: 50.0 };
    let r2 = Geometry::Rect { width: 100.0, height: 100.0 };
    let r3 = Geometry::Rect { width: 100.0, height: 200.0 };
    let c1 = Geometry::Circle { radius: 50.0 };

    let p1 = Default::default();
    let p2 = Paint { 
        translation: [100.0, 100.0],
        color: [1.0, 0.0, 0.0, 1.0],
        fill: false,
    };
    let p3 = Paint {
        fill: true,
        color: [0.0, 1.0, 0.0, 1.0],
        ..Default::default()
    };

    // note that because we set the green rectangle first, it gets drawn on top of the black
    // square.
    let h_r1 = renderer.add_geometry(r1);
    let h_r2 = renderer.add_geometry(r2);
    let h_r3 = renderer.add_geometry(r3);
    let h_c1 = renderer.add_geometry(c1);

    let h_p1 = renderer.add_paint(p1);
    let h_p2 = renderer.add_paint(p2);
    let h_p3 = renderer.add_paint(p3);

    renderer.set_paint(h_r1, h_p1);
    renderer.set_paint(h_r2, h_p2);
    renderer.set_paint(h_r3, h_p3);
    renderer.set_paint(h_c1, h_p1);

    renderer.render().unwrap();
}
