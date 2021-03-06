//! Transformation context

use draw_state::DrawState;
use vecmath::{
    identity,
    Matrix2d,
    Scalar
};

/// Transform property
#[derive(Copy)]
pub struct Transform(pub Matrix2d);

/// View transform property
#[derive(Copy)]
pub struct ViewTransform(pub Matrix2d);

/// Drawing 2d context.
#[derive(Copy, Clone)]
pub struct Context {
    /// View transformation.
    pub view: Matrix2d,
    /// Current transformation.
    pub transform: Matrix2d,
    /// Current draw state settings.
    pub draw_state: DrawState,
}

quack! {
    c: Context[]
    get:
        fn () -> Transform [] { Transform(c.transform) }
        fn () -> ViewTransform [] { ViewTransform(c.view) }
    set:
        fn (val: Transform) [] { c.transform = val.0 }
        fn (val: ViewTransform) [] { c.view = val.0 }
    action:
}


fn default_draw_state() -> DrawState {
    use draw_state::block::{ CullFace, RasterMethod };
    use draw_state::BlendPreset;

    let mut draw_state = DrawState::new();
    draw_state.primitive.method = RasterMethod::Fill(CullFace::Nothing);
    draw_state.blend(BlendPreset::Alpha)
}

impl Context {
    /// Creates a new drawing context.
    #[inline(always)]
    pub fn new() -> Context {
        Context {
            view: identity(),
            transform: identity(),
            draw_state: default_draw_state(),
        }
    }

    /// Creates a new drawing context in absolute coordinates.
    ///
    /// This function assumes the default coordinate system
    /// being centered with x axis pointing to the right
    /// and y axis pointing up.
    ///
    /// Returns a drawing context
    /// with origin in the upper left corner
    /// and x axis pointing to the right
    /// and y axis pointing down.
    #[inline(always)]
    pub fn abs(w: Scalar, h: Scalar) -> Context {
        let sx = 2.0 / w;
        let sy = -2.0 / h;
        let mat = [[ sx, 0.0, -1.0 ],
                   [ 0.0,  sy, 1.0 ]];
        Context {
            view: mat,
            transform: mat,
            draw_state: default_draw_state(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Context;
    use std::num::Float;

    #[test]
    fn test_context() {
        use RelativeTransform;

        let c = Context::new();
        {
            let d = c.trans(20.0, 40.0);
            let d = d.trans(10.0, 10.0);
            let transform = d.transform;
            assert_eq!(transform[0][2], 30.0);
            assert_eq!(transform[1][2], 50.0);
        }

        let transform = c.transform;
        assert_eq!(transform[0][2], 0.0);
        assert_eq!(transform[1][2], 0.0);

        let c = c.rot_deg(90.0);
        let transform = c.transform;
        assert!((transform[0][0] - 0.0).abs() < 0.00001);
        assert!((transform[0][1] + 1.0).abs() < 0.00001);
    }

    #[test]
    fn test_scale() {
        use RelativeTransform;

        let c = Context::new();
        let c = c.scale(2.0, 3.0);
        let transform = c.transform;
        assert!((transform[0][0] - 2.0).abs() < 0.00001);
        assert!((transform[1][1] - 3.0).abs() < 0.00001);
    }
}
