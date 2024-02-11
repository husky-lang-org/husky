use crate::diag::{bail, SourceResult};
use crate::engine::Engine;
use crate::foundations::{
    cast, elem, scope, Array, Fold, Func, Packed, Smart, StyleChain, TexContent, TexValue,
};
use crate::layout::{
    Axes, BlockElem, Cell, CellGrid, Fragment, GridLayouter, HAlignment, LayoutMultiple, Length,
    LengthInEm, Regions, Spacing, TexSizing, VAlignment,
};
use crate::model::ParagraphTexElem;
use crate::text::TextElem;

/// A bullet list.
///
/// Displays a sequence of items vertically, with each item introduced by a
/// marker.
///
/// # Example
/// ```example
/// Normal list.
/// - Text
/// - Math
/// - Layout
/// - ...
///
/// Multiple lines.
/// - This list item spans multiple
///   lines because it is indented.
///
/// Function call.
/// #list(
///   [Foundations],
///   [Calculate],
///   [Construct],
///   [Data Loading],
/// )
/// ```
///
/// # Syntax
/// This functions also has dedicated syntax: Start a line with a hyphen,
/// followed by a space to create a list item. A list item can contain multiple
/// paragraphs and other block-level content. All content that is indented
/// more than an item's marker becomes part of that item.
#[elem(scope, title = "Bullet List", LayoutMultiple)]
pub struct ListElem {
    /// If this is `{false}`, the items are spaced apart with
    /// [list spacing]($list.spacing). If it is `{true}`, they use normal
    /// [leading]($par.leading) instead. This makes the list more compact, which
    /// can look better if the items are short.
    ///
    /// In markup mode, the value of this parameter is determined based on
    /// whether items are separated with a blank line. If items directly follow
    /// each other, this is set to `{true}`; if items are separated by a blank
    /// line, this is set to `{false}`.
    ///
    /// ```example
    /// - If a list has a lot of text, and
    ///   maybe other inline content, it
    ///   should not be tight anymore.
    ///
    /// - To make a list wide, simply insert
    ///   a blank line between the items.
    /// ```
    #[default(true)]
    pub tight: bool,

    /// The marker which introduces each item.
    ///
    /// Instead of plain content, you can also pass an array with multiple
    /// markers that should be used for nested lists. If the list nesting depth
    /// exceeds the number of markers, the markers are cycled. For total
    /// control, you may pass a function that maps the list's nesting depth
    /// (starting from `{0}`) to a desired marker.
    ///
    /// ```example
    /// #set list(marker: [--])
    /// - A more classic list
    /// - With en-dashes
    ///
    /// #set list(marker: ([•], [--]))
    /// - Top-level
    ///   - Nested
    ///   - Items
    /// - Items
    /// ```
    #[borrowed]
    #[default(ListMarker::Content(vec![
        // These are all available in the default font, vertically centered, and
        // roughly of the same size (with the last one having slightly lower
        // weight because it is not filled).
        TextElem::packed('\u{2022}'), // Bullet
        TextElem::packed('\u{2023}'), // Triangular Bullet
        TextElem::packed('\u{2013}'), // En-dash
    ]))]
    pub marker: ListMarker,

    /// The indent of each item.
    #[resolve]
    pub indent: Length,

    /// The spacing between the marker and the body of each item.
    #[resolve]
    #[default(LengthInEm::new(0.5).into())]
    pub body_indent: Length,

    /// The spacing between the items of a wide (non-tight) list.
    ///
    /// If set to `{auto}`, uses the spacing [below blocks]($block.below).
    pub spacing: Smart<Spacing>,

    /// The bullet list's children.
    ///
    /// When using the list syntax, adjacent items are automatically collected
    /// into lists, even through constructs like for loops.
    ///
    /// ```example
    /// #for letter in "ABC" [
    ///   - Letter #letter
    /// ]
    /// ```
    #[variadic]
    pub children: Vec<Packed<ListItem>>,

    /// The nesting depth.
    #[internal]
    #[fold]
    #[ghost]
    depth: Depth,
}

#[scope]
impl ListElem {
    #[elem]
    type ListItem;
}

impl LayoutMultiple for Packed<ListElem> {
    #[husky_tex_macros::time(name = "list", span = self.span())]
    fn layout(
        &self,
        engine: &mut Engine,
        styles: StyleChain,
        regions: Regions,
    ) -> SourceResult<Fragment> {
        let indent = self.indent(styles);
        let body_indent = self.body_indent(styles);
        let gutter = if self.tight(styles) {
            ParagraphTexElem::leading_in(styles).into()
        } else {
            self.spacing(styles)
                .unwrap_or_else(|| *BlockElem::below_in(styles).amount())
        };

        let Depth(depth) = ListElem::depth_in(styles);
        let marker = self
            .marker(styles)
            .resolve(engine, depth)?
            // avoid '#set align' interference with the list
            .aligned(HAlignment::Start + VAlignment::Top);

        let mut cells = vec![];
        for item in self.children() {
            cells.push(Cell::from(TexContent::empty()));
            cells.push(Cell::from(marker.clone()));
            cells.push(Cell::from(TexContent::empty()));
            cells.push(Cell::from(
                item.body().clone().styled(ListElem::set_depth(Depth(1))),
            ));
        }

        let stroke = None;
        let grid = CellGrid::new(
            Axes::with_x(&[
                TexSizing::Rel(indent.into()),
                TexSizing::Auto,
                TexSizing::Rel(body_indent.into()),
                TexSizing::Auto,
            ]),
            Axes::with_y(&[gutter.into()]),
            cells,
        );
        let layouter = GridLayouter::new(&grid, &stroke, regions, styles, self.span());

        layouter.layout(engine)
    }
}

/// A bullet list item.
#[elem(name = "item", title = "Bullet List Item")]
pub struct ListItem {
    /// The item's body.
    #[required]
    pub body: TexContent,
}

cast! {
    ListItem,
    v: TexContent => v.unpack::<Self>().unwrap_or_else(Self::new)
}

/// A list's marker.
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum ListMarker {
    Content(Vec<TexContent>),
    Func(Func),
}

impl ListMarker {
    /// Resolve the marker for the given depth.
    fn resolve(&self, engine: &mut Engine, depth: usize) -> SourceResult<TexContent> {
        Ok(match self {
            Self::Content(list) => list.get(depth % list.len()).cloned().unwrap_or_default(),
            Self::Func(func) => func.call(engine, [depth])?.display(),
        })
    }
}

cast! {
    ListMarker,
    self => match self {
        Self::Content(vec) => if vec.len() == 1 {
            vec.into_iter().next().unwrap().into_value()
        } else {
            vec.into_value()
        },
        Self::Func(func) => func.into_value(),
    },
    v: TexContent => Self::Content(vec![v]),
    array: Array => {
        if array.is_empty() {
            bail!("array must contain at least one marker");
        }
        Self::Content(array.into_iter().map(TexValue::display).collect())
    },
    v: Func => Self::Func(v),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
struct Depth(usize);

impl Fold for Depth {
    fn fold(self, outer: Self) -> Self {
        Self(outer.0 + self.0)
    }
}