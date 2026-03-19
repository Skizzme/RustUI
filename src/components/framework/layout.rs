use std::alloc::LayoutError;
use crate::components::render::color::Color;
use crate::components::render::font::format::Alignment;
use crate::components::spatial::vec2::Vec2;
use crate::components::spatial::vec4::Vec4;

#[derive(Debug, PartialEq)]
pub enum LayoutEvent {
    FitWidth,
    GrowWidth(f32),
    OptimizeSize(Vec2<f32>), // ex. text wrapping
    FitHeight,
    GrowHeight(f32),
    Position(Vec2<f32>),
}

impl LayoutEvent {
    pub fn grow_direction(dir: &LayoutDirection, v: f32) -> Self {
        match dir {
            LayoutDirection::Horizontal => LayoutEvent::GrowWidth(v),
            LayoutDirection::Vertical => LayoutEvent::GrowHeight(v),
        }
    }
}

#[derive(Default, Clone)]
pub enum LayoutDirectionV {
    #[default]
    TopToBottom,
    BottomToTop,
}

#[derive(Default, Clone)]
pub enum LayoutDirectionH {
    #[default]
    LeftToRight,
    RightToLeft,
}

#[derive(Default, Clone, PartialEq, Debug)]
pub enum LayoutDirection {
    #[default]
    Horizontal,
    Vertical,
}

impl LayoutDirection {
    pub fn is_horizontal(&self) -> bool {
        match self {
            LayoutDirection::Horizontal => true,
            LayoutDirection::Vertical => false
        }
    }
    pub fn is_vertical(&self) -> bool {
        match self {
            LayoutDirection::Horizontal => false,
            LayoutDirection::Vertical => true
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum Sizing {
    Grow,
    #[default]
    Shrink,
    Fixed(f32),
}

#[derive(Clone, Default)]
pub struct LayoutContext {
    pub min_size: Vec2<f32>,
    pub max_size: Option<Vec2<f32>>,
    pub pref_size: Vec2<f32>,
    pub debug_color: Color,

    pub spacing: Vec2<f32>,

    pub size_behavior: (Sizing, Sizing),

    pub margin: Vec4,
    pub padding: Vec4,

    pub alignment_h: Alignment,
    pub alignment_v: Alignment,

    pub direction: LayoutDirection,
    pub direction_h: LayoutDirectionH,
    pub direction_v: LayoutDirectionV,
}

impl LayoutContext {
    pub fn new() -> Self {
        LayoutContext {
            min_size: Vec2::zero(),
            max_size: None,
            pref_size: Vec2::zero(),
            debug_color: Default::default(),
            spacing: Default::default(),
            size_behavior: (Sizing::Shrink, Sizing::Shrink),
            margin: Vec4::zero().clone(),
            padding: Vec4::zero().clone(),
            alignment_h: Alignment::Center,
            alignment_v: Alignment::Center,
            direction: Default::default(),
            direction_h: LayoutDirectionH::LeftToRight,
            direction_v: LayoutDirectionV::TopToBottom,
        }
    }

    pub fn min_size_margined(&self) -> Vec2<f32> {
        self.min_size + (self.margin.left() + self.margin.right(), self.margin.top() + self.margin.bottom())
    }

    pub fn max_size_margined(&self) -> Option<Vec2<f32>> {
        if let Some(max) = self.max_size {
            Some(max + (self.margin.left() + self.margin.right(), self.margin.top() + self.margin.bottom()))
        } else {
            None
        }
    }
}