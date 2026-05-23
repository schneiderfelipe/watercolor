use std::time::Duration;

use watercolor_temporal as temporal;

use watercolor_frame as frame;

pub trait SilentFilm: temporal::Temporal {
    type Frame: frame::Frame;

    /// Called with monotonically non-decreasing `t`.
    fn frame(&mut self, t: Duration) -> &Self::Frame;
}
