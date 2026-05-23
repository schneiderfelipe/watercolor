use std::time::Duration;

use watercolor_temporal as temporal;

pub trait Sound: temporal::Temporal {
    /// Called with monotonically non-decreasing `t`.
    fn amplitude(&mut self, t: Duration) -> [f32; 2];
}
