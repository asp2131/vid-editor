//! Video input handling using FFmpeg.

use anyhow::Result;

/// Reads video frames from a file or stream.
/// Currently a stub; real implementation will wrap ffmpeg-next APIs.
#[derive(Debug)]
pub struct VideoInput {
    path: String,
}

impl VideoInput {
    /// Create a new `VideoInput` from a given path.
    pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        Ok(Self {
            path: path.as_ref().to_string(),
        })
    }

    /// Return number of frames (placeholder).
    pub fn frame_count(&self) -> Result<i64> {
        // TODO: Query using ffmpeg-next once hooked up.
        Ok(0)
    }
}
