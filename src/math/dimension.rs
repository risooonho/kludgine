use crate::math::{Length, Scaled};

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension<Unit = Scaled> {
    Auto,
    /// Scale-corrected to the users preference of DPI
    Length(Length<f32, Unit>),
}

impl<Unit> Dimension<Unit> {
    pub fn from_f32(value: f32) -> Self {
        Self::Length(Length::new(value))
    }

    pub fn from_length<V: Into<Length<f32, Unit>>>(value: V) -> Self {
        Self::Length(value.into())
    }

    pub fn is_auto(&self) -> bool {
        match self {
            Dimension::Auto => true,
            Dimension::Length(_) => false,
        }
    }
    pub fn is_length(&self) -> bool {
        !self.is_auto()
    }

    pub fn length(&self) -> Option<Length<f32, Unit>> {
        if let Dimension::Length(points) = &self {
            Some(*points)
        } else {
            None
        }
    }
}

impl<Unit> Default for Dimension<Unit> {
    fn default() -> Self {
        Dimension::Auto
    }
}

impl<Unit> From<Length<f32, Unit>> for Dimension<Unit> {
    fn from(value: Length<f32, Unit>) -> Self {
        Dimension::from_length(value)
    }
}
