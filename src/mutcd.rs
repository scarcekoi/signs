//! Types for representing signs in MUTCD format.

use mutcd_colors::models::MutcdColor;

use crate::models;

/// FHWA Standard Alphabet series.
pub enum FhwaSeries {
    B,
    C,
    D,
    E(FhwaSeriesE),
    F,
}

/// Variants of the FHWA Standard Alphabet Series E.
pub enum FhwaSeriesE {
    Unmodified,
    Modified { alternate: bool },
}

/// [Available sign shapes](https://mutcd.fhwa.dot.gov/pdfs/11th_Edition/mutcd11theditionhl.pdf#%5B%7B%22num%22%3A276%2C%22gen%22%3A0%7D%2C%7B%22name%22%3A%22FitR%22%7D%2C-154%2C194%2C766%2C792%5D) defined by the [MUTCD 11th Edition](https://mutcd.fhwa.dot.gov/pdfs/11th_Edition/mutcd11theditionhl.pdf).
pub enum MutcdSignShape {
    Octagon,
    /// An equilateral triangle that points downward.
    Triangle,
    Circle,
    /// An isosceles triangle with a longer axis horizontally, pointed right.
    Pennant,
    /// A pentagon pointed upwards.
    Pentagon,
    /// Two rectangles in a perpendicular “X” configuration.
    Crossbuck,
    Diamond,
    Rectangle,
    Trapezoid,
}

/// The colors on the sign.
pub struct MutcdSignColors {
    pub border: Option<MutcdColor>,
}

/// The features of the sign.
pub struct MutcdSignFeatures {
    pub arrow: Option<models::Arrow>,
    pub colors: Option<MutcdSignColors>,
}

/// A sign with MUTCD features.
pub struct MutcdSign {
    /// Text to display on the sign.
    pub text: String,
    /// Which MUTCD series to use for the text on the sign.
    pub font: FhwaSeries,
    /// The shape of the sign.
    pub shape: MutcdSignShape,
    /// The features of the sign.
    pub features: MutcdSignFeatures,
}
