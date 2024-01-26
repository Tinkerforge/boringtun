use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Instant {
    inner: f64,
}

impl Instant {
    pub fn now() -> Self {
        Self {
            inner: js_sys::Date::now(),
        }
    }

    pub fn duration_since(&self, earlier: Instant) -> Duration {
        Duration::from_millis(self.inner as u64 - earlier.inner as u64)
    }

    pub fn elapsed(&self) -> Duration {
        Self::now().duration_since(*self)
    }
}
