//! Test code for piet.

// Right now, this is just code to generate sample images.

use kurbo::{Affine, BezPath, Line, Vec2};

use piet::{FillRule, FontBuilder, RenderContext, TextLayout, TextLayoutBuilder};

// Note: this could be a Shape.
fn star(center: Vec2, inner: f64, outer: f64, n: usize) -> BezPath {
    let mut result = BezPath::new();
    let d_th = std::f64::consts::PI / (n as f64);
    for i in 0..n {
        let outer_pt = center + outer * Vec2::from_angle(d_th * ((i * 2) as f64));
        if i == 0 {
            result.moveto(outer_pt);
        } else {
            result.lineto(outer_pt);
        }
        result.lineto(center + inner * Vec2::from_angle(d_th * ((i * 2 + 1) as f64)));
    }
    result.closepath();
    result
}

fn draw_picture_0(rc: &mut impl RenderContext) {
    rc.clear(0xFF_FF_FF);
    let brush = rc.solid_brush(0x00_00_80_FF);
    rc.stroke(Line::new((10.0, 10.0), (100.0, 50.0)), &brush, 1.0, None);

    let mut path = BezPath::new();
    path.moveto((50.0, 10.0));
    path.quadto((60.0, 50.0), (100.0, 90.0));
    let brush = rc.solid_brush(0x00_80_00_FF);
    rc.stroke(path, &brush, 1.0, None);

    let mut path = BezPath::new();
    path.moveto((10.0, 20.0));
    path.curveto((10.0, 80.0), (100.0, 80.0), (100.0, 60.0));
    let brush = rc.solid_brush(0x00_00_80_C0);
    rc.fill(path, &brush, FillRule::NonZero);

    let font = rc.new_font_by_name("Segoe UI", 12.0).build();
    let layout = rc.new_text_layout(&font, "Hello piet!").build();
    let w: f64 = layout.width().into();
    let brush = rc.solid_brush(0x80_00_00_C0);
    rc.draw_text(&layout, (80.0, 10.0), &brush);

    rc.stroke(Line::new((80.0, 12.0), (80.0 + w, 12.0)), &brush, 1.0, None);

    rc.with_save(|rc| {
        rc.transform(Affine::rotate(0.1));
        rc.draw_text(&layout, (80.0, 10.0), &brush);
    });

    let clip_path = star(Vec2::new(90.0, 45.0), 10.0, 30.0, 24);
    rc.clip(clip_path, FillRule::NonZero);
    let layout = rc.new_text_layout(&font, "Clipped text").build();
    rc.draw_text(&layout, (80.0, 50.0), &brush);
}

/// Draw a test picture, by number.
///
/// Hopefully there will be a suite of test pictures. For now, there is just the one.
pub fn draw_test_picture(rc: &mut impl RenderContext, number: usize) {
    match number {
        0 => draw_picture_0(rc),
        _ => eprintln!(
            "Don't have test picture {} yet. Why don't you make it?",
            number
        ),
    }
}