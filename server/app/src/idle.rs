//! Optional idle shutdown (`IDLE_TIMEOUT_SECS`). The global rule for local
//! servers is to exit after 600 s without requests. Every request touches
//! [`Activity`], and [`wait_idle`] resolves once nothing happened for the
//! timeout.

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::Instant;

#[derive(Debug)]
pub struct Activity {
    start: Instant,
    last_ms: AtomicU64,
}

impl Activity {
    pub fn new() -> Arc<Self> {
        Arc::new(Activity {
            start: Instant::now(),
            last_ms: AtomicU64::new(0),
        })
    }

    fn now_ms(&self) -> u64 {
        u64::try_from(self.start.elapsed().as_millis()).unwrap_or(u64::MAX)
    }

    pub fn touch(&self) {
        self.last_ms.store(self.now_ms(), Ordering::Relaxed);
    }

    pub fn idle_for(&self) -> Duration {
        Duration::from_millis(
            self.now_ms()
                .saturating_sub(self.last_ms.load(Ordering::Relaxed)),
        )
    }
}

/// Resolves when `activity` has been idle for `timeout`.
pub async fn wait_idle(activity: Arc<Activity>, timeout: Duration) {
    loop {
        let idle = activity.idle_for();
        if idle >= timeout {
            return;
        }
        tokio::time::sleep(timeout.saturating_sub(idle).min(Duration::from_secs(5))).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn idle_time_and_wait() {
        let a = Activity::new();
        a.touch();
        tokio::time::advance(Duration::from_millis(500)).await;
        assert_eq!(a.idle_for(), Duration::from_millis(500));
        a.touch();
        assert_eq!(a.idle_for(), Duration::ZERO);
        let start = Instant::now();
        wait_idle(a.clone(), Duration::from_secs(3)).await;
        assert!(start.elapsed() >= Duration::from_secs(3));
    }
}
