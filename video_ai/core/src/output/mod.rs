//! Video output handling using FFmpeg.

use anyhow::Result;

#[derive(Debug)]
pub struct VideoOutput {
    path: String,
}

impl VideoOutput {
    /// Create a new `VideoOutput` for the given path.
    pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        Ok(Self {
            path: path.as_ref().to_string(),
        })
    }

    /// Write a placeholder frame count (stub).
    pub fn write_placeholder(&self, _frames: i64) -> Result<()> {
        // TODO: implement actual writing via ffmpeg-next.
        Ok(())
    }
}
