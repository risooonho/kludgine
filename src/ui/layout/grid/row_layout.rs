use super::chain_layout::{ChainElementContents, ChainElementDimensionTranslator, ChainLayout};
use crate::math::{Dimension, Points, Rect, Scaled, SizeExt, Surround};
use std::ops::Deref;

#[derive(Debug, Default)]
pub struct RowLayout {
    chain: ChainLayout,
}

impl ChainElementDimensionTranslator for RowLayout {
    fn convert_to_margin(min: Points, max: Points) -> Surround<f32, Scaled> {
        Surround {
            left: Points::default(),
            top: min,
            right: Points::default(),
            bottom: max,
        }
    }

    fn length_from_bounds(bounds: &Rect<f32, Scaled>) -> Points {
        bounds.size.height()
    }
}

impl RowLayout {
    pub fn row<I: Into<ChainElementContents>>(mut self, child: I, height: Dimension) -> Self {
        self.chain = self.chain.element(child, height);
        self
    }
}

impl Deref for RowLayout {
    type Target = ChainLayout;

    fn deref(&self) -> &Self::Target {
        &self.chain
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        math::{Point, Rect, Size},
        ui::layout::grid::chain_layout::ChainElementDynamicContents,
    };
    use generational_arena::Index;

    #[test]
    fn two_auto_rows_one_fixed_smaller() {
        let layouts = dbg!(RowLayout::default()
            .row(Index::from_raw_parts(0, 0), Dimension::Auto)
            .row(Index::from_raw_parts(0, 1), Dimension::from_f32(30.))
            .row(Index::from_raw_parts(0, 2), Dimension::Auto)
            .layouts_within_bounds(&Rect::new(Point::new(5., 5.), Size::new(150., 100.))));

        assert_eq!(layouts.len(), 3);
        assert_eq!(
            layouts[&Index::from_raw_parts(0, 0)]
                .inner_bounds()
                .to_u32(),
            Rect::new(Point::new(5, 5), Size::new(150, 35))
        );
        assert_eq!(
            layouts[&Index::from_raw_parts(0, 1)]
                .inner_bounds()
                .to_u32(),
            Rect::new(Point::new(5, 40), Size::new(150, 30))
        );
        assert_eq!(
            layouts[&Index::from_raw_parts(0, 2)]
                .inner_bounds()
                .to_u32(),
            Rect::new(Point::new(5, 70), Size::new(150, 35))
        );
    }
}
