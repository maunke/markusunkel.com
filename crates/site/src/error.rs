/// Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    /// IO error
    #[error("chrono parse error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),
    /// Invalid Parsing Event
    #[error("Invalid markdown: {0}")]
    InvalidMarkdown(&'static str),
    /// `{name}` fence naming a component that is not registered
    #[error("unknown markdown component: {0}")]
    UnknownComponent(String),
    /// Param the component does not read, usually a typo
    #[error("markdown component {component}: unknown param {param}")]
    UnknownParam { component: String, param: String },
    #[cfg(feature = "ssr")]
    /// Image Buffer Error
    #[error("image buffer error: {0}")]
    ImageBuffer(#[from] fast_image_resize::ImageBufferError),
    #[cfg(feature = "ssr")]
    /// Image Resize Error
    #[error("image resize error: {0}")]
    ImageResize(#[from] fast_image_resize::ResizeError),
    #[cfg(feature = "ssr")]
    /// Image Decode Error
    #[error("image decode error: {0}")]
    ImageDecode(#[from] image::ImageError),
    #[cfg(feature = "ssr")]
    /// Asset could not be read
    #[error("asset {}: {source}", path.display())]
    Asset {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
    #[cfg(feature = "ssr")]
    /// SVG asset could not be optimized or measured
    #[error("svg {}: {reason}", path.display())]
    Svg {
        path: std::path::PathBuf,
        reason: String,
    },
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
