use std::cmp::Ordering;
use std::fmt::{self, Debug, Formatter};
use std::ops::{Add, Div, Mul, Neg};

use ecow::{eco_format, EcoString};

use crate::diag::{At, Hint, TypstSourceResult};
use crate::foundations::{
    func, scope, ty, Fold, Resolve, TypstStyleChain, TypstStyles, TypstValueRepr,
};
use crate::layout::{TypstAbsLength, TypstEmLength};
use crate::syntax::TypstSynSpan;
use crate::util::TypstNumeric;

/// A size or distance, possibly expressed with contextual units.
///
/// Typst supports the following length units:
///
/// - Points: `{72pt}`
/// - Millimeters: `{254mm}`
/// - Centimeters: `{2.54cm}`
/// - Inches: `{1in}`
/// - Relative to font size: `{2.5em}`
///
/// You can multiply lengths with and divide them by integers and floats.
///
/// # Example
/// ```example
/// #rect(width: 20pt)
/// #rect(width: 2em)
/// #rect(width: 1in)
///
/// #(3em + 5pt).em \
/// #(20pt).em \
/// #(40em + 2pt).abs \
/// #(5em).abs
/// ```
///
/// # Fields
/// - `abs`: A length with just the absolute component of the current length
///   (that is, excluding the `em` component).
/// - `em`: The amount of `em` units in this length, as a [float]($float).
/// the actual length is equal to the sum of `abs` and `em`
#[ty(scope, cast)]
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct TypstLength {
    /// The absolute part.
    pub abs: TypstAbsLength,
    /// The font-relative part.
    pub em: TypstEmLength,
}

impl TypstLength {
    /// The zero length.
    pub const fn zero() -> Self {
        Self {
            abs: TypstAbsLength::zero(),
            em: TypstEmLength::zero(),
        }
    }

    /// Try to compute the absolute value of the length.
    pub fn try_abs(self) -> Option<Self> {
        (self.abs.is_zero() || self.em.is_zero()).then(|| Self {
            abs: self.abs.abs(),
            em: self.em.abs(),
        })
    }

    /// Try to divide two lengths.
    pub fn try_div(self, other: Self) -> Option<f64> {
        if self.abs.is_zero() && other.abs.is_zero() {
            Some(self.em / other.em)
        } else if self.em.is_zero() && other.em.is_zero() {
            Some(self.abs / other.abs)
        } else {
            None
        }
    }

    /// Convert to an absolute length at the given font size.
    pub fn at(self, font_size: TypstAbsLength) -> TypstAbsLength {
        self.abs + self.em.at(font_size)
    }

    /// Fails with an error if the length has a non-zero font-relative part.
    fn ensure_that_em_is_zero(&self, span: TypstSynSpan, unit: &str) -> TypstSourceResult<()> {
        if self.em == TypstEmLength::zero() {
            return Ok(());
        }
        Err(eco_format!(
            "cannot convert a length with non-zero em units (`{}`) to {unit}",
            self.repr()
        ))
        .hint(eco_format!(
            "use `length.abs.{unit}()` instead to ignore its em component"
        ))
        .at(span)
    }
}

#[scope]
impl TypstLength {
    /// Converts this length to points.
    ///
    /// Fails with an error if this length has non-zero `em` units (such as
    /// `5em + 2pt` instead of just `2pt`). Use the `abs` field (such as in
    /// `(5em + 2pt).abs.pt()`) to ignore the `em` component of the length (thus
    /// converting only its absolute component).
    #[func(name = "pt", title = "Points")]
    pub fn to_pt(&self, span: TypstSynSpan) -> TypstSourceResult<f64> {
        self.ensure_that_em_is_zero(span, "pt")?;
        Ok(self.abs.to_pt())
    }

    /// Converts this length to millimeters.
    ///
    /// Fails with an error if this length has non-zero `em` units. See the
    /// [`pt`]($length.pt) method for more details.
    #[func(name = "mm", title = "Millimeters")]
    pub fn to_mm(&self, span: TypstSynSpan) -> TypstSourceResult<f64> {
        self.ensure_that_em_is_zero(span, "mm")?;
        Ok(self.abs.to_mm())
    }

    /// Converts this length to centimeters.
    ///
    /// Fails with an error if this length has non-zero `em` units. See the
    /// [`pt`]($length.pt) method for more details.
    #[func(name = "cm", title = "Centimeters")]
    pub fn to_cm(&self, span: TypstSynSpan) -> TypstSourceResult<f64> {
        self.ensure_that_em_is_zero(span, "cm")?;
        Ok(self.abs.to_cm())
    }

    /// Converts this length to inches.
    ///
    /// Fails with an error if this length has non-zero `em` units. See the
    /// [`pt`]($length.pt) method for more details.
    #[func(name = "inches")]
    pub fn to_inches(&self, span: TypstSynSpan) -> TypstSourceResult<f64> {
        self.ensure_that_em_is_zero(span, "inches")?;
        Ok(self.abs.to_inches())
    }

    /// Resolve this length to an absolute length.
    ///
    /// ```example
    /// #set text(size: 12pt)
    /// #style(styles => [
    ///   #(6pt).to-absolute(styles) \
    ///   #(6pt + 10em).to-absolute(styles) \
    ///   #(10em).to-absolute(styles)
    /// ])
    ///
    /// #set text(size: 6pt)
    /// #style(styles => [
    ///   #(6pt).to-absolute(styles) \
    ///   #(6pt + 10em).to-absolute(styles) \
    ///   #(10em).to-absolute(styles)
    /// ])
    /// ```
    #[func]
    pub fn to_absolute(
        &self,
        /// The styles to resolve the length with.
        ///
        /// Since a length can use font-relative em units, resolving it to an
        /// absolute length requires knowledge of the font size. This size is
        /// provided through these styles. You can obtain the styles using
        /// the [`style`]($style) function.
        styles: TypstStyles,
    ) -> TypstLength {
        let styles = TypstStyleChain::new(&styles);
        self.resolve(styles).into()
    }
}

impl Debug for TypstLength {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match (self.abs.is_zero(), self.em.is_zero()) {
            (false, false) => write!(f, "{:?} + {:?}", self.abs, self.em),
            (true, false) => self.em.fmt(f),
            (_, true) => self.abs.fmt(f),
        }
    }
}

impl TypstValueRepr for TypstLength {
    fn repr(&self) -> EcoString {
        match (self.abs.is_zero(), self.em.is_zero()) {
            (false, false) => eco_format!("{} + {}", self.abs.repr(), self.em.repr()),
            (true, false) => self.em.repr(),
            (_, true) => self.abs.repr(),
        }
    }
}

impl TypstNumeric for TypstLength {
    fn zero() -> Self {
        Self::zero()
    }

    fn is_finite(self) -> bool {
        self.abs.is_finite() && self.em.is_finite()
    }
}

impl PartialOrd for TypstLength {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.em.is_zero() && other.em.is_zero() {
            self.abs.partial_cmp(&other.abs)
        } else if self.abs.is_zero() && other.abs.is_zero() {
            self.em.partial_cmp(&other.em)
        } else {
            None
        }
    }
}

impl From<TypstAbsLength> for TypstLength {
    fn from(abs: TypstAbsLength) -> Self {
        Self {
            abs,
            em: TypstEmLength::zero(),
        }
    }
}

impl From<TypstEmLength> for TypstLength {
    fn from(em: TypstEmLength) -> Self {
        Self {
            abs: TypstAbsLength::zero(),
            em,
        }
    }
}

impl Neg for TypstLength {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            abs: -self.abs,
            em: -self.em,
        }
    }
}

impl Add for TypstLength {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            abs: self.abs + rhs.abs,
            em: self.em + rhs.em,
        }
    }
}

sub_impl!(TypstLength - TypstLength -> TypstLength);

impl Mul<f64> for TypstLength {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            abs: self.abs * rhs,
            em: self.em * rhs,
        }
    }
}

impl Mul<TypstLength> for f64 {
    type Output = TypstLength;

    fn mul(self, rhs: TypstLength) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for TypstLength {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            abs: self.abs / rhs,
            em: self.em / rhs,
        }
    }
}

assign_impl!(TypstLength += TypstLength);
assign_impl!(TypstLength -= TypstLength);
assign_impl!(TypstLength *= f64);
assign_impl!(TypstLength /= f64);

impl Resolve for TypstLength {
    type Output = TypstAbsLength;

    fn resolve(self, styles: TypstStyleChain) -> Self::Output {
        self.abs + self.em.resolve(styles)
    }
}

impl Fold for TypstLength {
    fn fold(self, _: Self) -> Self {
        self
    }
}