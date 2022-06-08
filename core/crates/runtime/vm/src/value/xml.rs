use crate::*;
use check_utils::should_eq;
use husky_debugger_gui::protocol::{Point2dProps, VisualProps};
use print_utils::msg_once;
use serde::Serialize;
use serde_json::value::Value;
use word::{CustomIdentifier, IdentPairDict};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlValue {
    pub tag_kind: XmlTagKind,
    pub props: IdentPairDict<Value>,
}

impl Serialize for XmlValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTagKind {
    Point2d,
    Contour,
    Arrow2d,
    LineSegment,
}

impl XmlTagKind {
    pub fn as_str(self) -> &'static str {
        match self {
            XmlTagKind::Point2d => "Point2d",
            XmlTagKind::Arrow2d => "Arrow2d",
            XmlTagKind::Contour => "Contour",
            XmlTagKind::LineSegment => "LineSegment",
        }
    }

    pub fn from_ident(ident: CustomIdentifier) -> Self {
        match ident.as_str() {
            "Point2d" => XmlTagKind::Point2d,
            "Contour" => XmlTagKind::Contour,
            "Arrow2d" => XmlTagKind::Arrow2d,
            "LineSegment" => XmlTagKind::LineSegment,
            _ => todo!(),
        }
    }
}

impl Into<VisualProps> for XmlValue {
    fn into(self) -> VisualProps {
        let mut data = self.props.take_data();
        msg_once!("ad hoc");
        match self.tag_kind.as_str() {
            "Contour" => {
                should_eq!(data.len(), 1);
                let (ident, value) = data.pop().unwrap();
                let points: Vec<Point2dProps> = serde_json::from_value(value).unwrap();
                VisualProps::Contour { points }
            }
            "LineSegment" => {
                should_eq!(data.len(), 2);
                // end
                let (ident, value) = data.pop().unwrap();
                should_eq!(ident.as_str(), "end");
                let end: Point2dProps = serde_json::from_value(value).unwrap();
                // start
                let (ident, value) = data.pop().unwrap();
                should_eq!(ident.as_str(), "start");
                let start: Point2dProps = serde_json::from_value(value).unwrap();
                VisualProps::LineSegment { start, end }
            }
            _ => todo!(),
        }
    }
}
