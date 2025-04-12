/// Visualization output formats
pub enum OutputFormat {
    Text,
    PNG,
    SVG,
    HTML,
}

/// Coverage calculation methods
pub enum CoverageMethod {
    PerBase,
    WindowAverage,
    SlidingWindow,
}

/// Sequence alignment methods
pub enum AlignmentMethod {
    Global,
    Local,
    SemiGlobal,
}
