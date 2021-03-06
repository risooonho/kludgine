use crate::{
    math::{Point, Raw, Scale, Scaled, Surround, Vector},
    style::{ColorPair, Style, StyleComponent, UnscaledStyleComponent},
    window::event::MouseButton,
};
use euclid::Length;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub enum ControlEvent {
    Clicked {
        button: MouseButton,
        window_position: Point<f32, Scaled>,
    },
}

#[derive(Debug, Clone, Default)]
pub struct ComponentPadding<Unit>(pub Surround<f32, Unit>);

impl StyleComponent<Scaled> for ComponentPadding<Scaled> {
    fn scale(&self, scale: Scale<f32, Scaled, Raw>, destination: &mut Style<Raw>) {
        destination.push(ComponentPadding(self.0 * scale))
    }

    fn should_be_inherited(&self) -> bool {
        false
    }
}

impl StyleComponent<Raw> for ComponentPadding<Raw> {
    fn scale(&self, _scale: Scale<f32, Raw, Raw>, map: &mut Style<Raw>) {
        map.push(ComponentPadding(self.0));
    }

    fn should_be_inherited(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct Border {
    pub width: Length<f32, Scaled>,
    pub color: ColorPair,
}

impl Border {
    pub fn new(width: f32, color: ColorPair) -> Self {
        Self {
            width: Length::new(width),
            color,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ComponentBorder {
    pub left: Option<Border>,
    pub top: Option<Border>,
    pub right: Option<Border>,
    pub bottom: Option<Border>,
}

impl ComponentBorder {
    pub fn uniform(border: Border) -> Self {
        Self {
            left: Some(border.clone()),
            top: Some(border.clone()),
            right: Some(border.clone()),
            bottom: Some(border),
        }
    }

    pub fn with_left(mut self, left: Border) -> Self {
        self.left = Some(left);
        self
    }

    pub fn with_right(mut self, right: Border) -> Self {
        self.right = Some(right);
        self
    }

    pub fn with_bottom(mut self, bottom: Border) -> Self {
        self.bottom = Some(bottom);
        self
    }

    pub fn with_top(mut self, top: Border) -> Self {
        self.top = Some(top);
        self
    }
}

impl UnscaledStyleComponent<Scaled> for ComponentBorder {
    fn unscaled_should_be_inherited(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Default)]
pub struct ContentOffset(pub Vector<f32, Scaled>);
