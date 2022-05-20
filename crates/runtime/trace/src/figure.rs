mod control;
mod graphics2d;

pub use control::*;
pub use graphics2d::*;

use crate::*;
use map_collect::MapCollect;
use visual_runtime::RuntimeVisualizer;
use visual_syntax::VisualProps;
use vm::{CopyableValue, MutationData};
use word::Identifier;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum FigureProps {
    Primitive {
        value: CopyableValue,
    },
    Plot2d {
        plot_kind: Plot2dKind,
        point_groups: Vec<Point2dGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Graphics2d {
        image: Option<ImageProps>,
        shape_groups: Vec<Shape2dGroupProps>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Mutations {
        mutations: Vec<MutationFigureProps>,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct MutationFigureProps {
    pub name: String,
    pub before: Option<FigureProps>,
    pub after: FigureProps,
    pub idx: usize,
}

impl<'eval> MutationFigureProps {
    pub fn new(
        text: &Text,
        visualizer: &RuntimeVisualizer,
        mutation_data: &MutationData<'eval>,
        idx: usize,
    ) -> Self {
        MutationFigureProps {
            name: match mutation_data.kind {
                vm::MutationDataKind::Exec { range } => text.ranged(range),
                vm::MutationDataKind::Block { varname, .. } => varname.as_str().to_string(),
            },
            before: mutation_data
                .before
                .as_ref()
                .map(|before| FigureProps::new_specific(visualizer.visualize(before.any_ref()))),
            after: FigureProps::new_specific(visualizer.visualize(mutation_data.after.any_ref())),
            idx,
        }
    }
}

impl FigureProps {
    pub fn new_specific(visual_props: VisualProps) -> Self {
        match visual_props {
            VisualProps::BinaryImage28 { padded_rows } => FigureProps::Graphics2d {
                image: Some(ImageProps::binary_image_28(&padded_rows)),
                shape_groups: Vec::new(),
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualProps::Primitive { value } => FigureProps::Primitive { value },
            VisualProps::BinaryGrid28 { ref padded_rows } => FigureProps::Graphics2d {
                image: None,
                shape_groups: vec![Shape2dGroupProps::laser_grid28(padded_rows)],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
        }
    }

    pub fn void() -> Self {
        Self::Primitive {
            value: CopyableValue::Void,
        }
    }
}

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Plot2dKind {
    Scatter,
}

#[derive(Debug, Serialize, Clone)]
pub struct Point2dGroup {
    pub points: Vec<Point2d>,
    pub color: Color,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

#[derive(Debug, Serialize, Clone)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

impl Point2d {
    pub fn from_ij28(i: usize, j: usize) -> Self {
        Point2d {
            x: j as f32 + 1.0,
            y: 29.0 - i as f32,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Vector2d {
    x: f32,
    y: f32,
}
