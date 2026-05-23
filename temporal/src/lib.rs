use std::time::Duration;

pub trait Temporal {
    fn duration(&self) -> Duration;
}
