mod dimension;
// mod measurement;
// mod point;
// mod rect;
// mod size;
mod length_traits;
pub(crate) mod rgx_conversions;
mod surround;

pub use self::{dimension::*, length_traits::*, surround::*}; //, measurement::*, point::*, rect::*, size::*,

pub type Size<T = f32, Unit = Unknown> = euclid::Size2D<T, Unit>;
pub type Point<T = f32, Unit = Unknown> = euclid::Point2D<T, Unit>;
pub type Rect<T = f32, Unit = Unknown> = euclid::Rect<T, Unit>;
pub type Pixels = euclid::Length<f32, Raw>;
pub type Points = euclid::Length<f32, Scaled>;
pub type Vector<T = f32, Unit = Unknown> = euclid::Vector2D<T, Unit>;
pub use euclid::{Length, Scale};
pub type ScreenScale = Scale<f32, Scaled, Raw>;

#[derive(Clone, Copy, Debug, Default)]
pub struct Raw;
#[derive(Clone, Copy, Debug, Default)]
pub struct Scaled;
#[derive(Clone, Copy, Debug, Default)]
pub struct Unknown;

pub(crate) fn max_f(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub(crate) fn min_f(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn min_max_tests() {
        assert_relative_eq!(min_f(0.0, 1.0), 0.0);
        assert_relative_eq!(min_f(1.0, 0.0), 0.0);
        assert_relative_eq!(min_f(0.0, 0.0), 0.0);

        assert_relative_eq!(max_f(0.0, 1.0), 1.0);
        assert_relative_eq!(max_f(1.0, 0.0), 1.0);
        assert_relative_eq!(max_f(0.0, 0.0), 0.0);
    }
}
