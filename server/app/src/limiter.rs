//! Login rate limit (KNOWN_ISSUES K5, approved as plan Q9). Laravel has
//! none. A fixed one-minute window per client IP, in memory (one process,
//! one editor). `LOGIN_RATE_LIMIT=0` turns it off, which `run.sh rust`
//! does because the contract suite logs in about 60 times.
//!
//! Over the limit: 429 `{"message": "Too Many Attempts."}` with
//! `Retry-After`, the body Laravel's `throttle` middleware uses.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Mutex;
use std::time::Duration;
use tokio::time::Instant;

const WINDOW: Duration = Duration::from_secs(60);
/// Drop expired entries once the map grows past this.
const PRUNE_AT: usize = 1024;

#[derive(Debug)]
pub struct LoginLimiter {
    max: u32,
    hits: Mutex<HashMap<Option<IpAddr>, (Instant, u32)>>,
}

impl LoginLimiter {
    pub fn new(max_per_minute: u32) -> Self {
        LoginLimiter {
            max: max_per_minute,
            hits: Mutex::new(HashMap::new()),
        }
    }

    /// Count one attempt. `Err(seconds)` when over the limit. `None` is a
    /// client without a known address (in-process tests); it shares a bucket.
    pub fn hit(&self, ip: Option<IpAddr>) -> Result<(), u64> {
        if self.max == 0 {
            return Ok(());
        }
        let now = Instant::now();
        let Ok(mut hits) = self.hits.lock() else {
            // A poisoned lock means a panic elsewhere; fail open rather than
            // locking the editor out.
            return Ok(());
        };
        if hits.len() > PRUNE_AT {
            hits.retain(|_, (start, _)| now.duration_since(*start) < WINDOW);
        }
        let entry = hits.entry(ip).or_insert((now, 0));
        if now.duration_since(entry.0) >= WINDOW {
            *entry = (now, 0);
        }
        if entry.1 >= self.max {
            let left = WINDOW.saturating_sub(now.duration_since(entry.0));
            return Err(left.as_secs().max(1));
        }
        entry.1 += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn window_per_ip() {
        let l = LoginLimiter::new(2);
        let a: Option<IpAddr> = "127.0.0.1".parse().ok();
        let b: Option<IpAddr> = "127.0.0.2".parse().ok();
        assert_eq!(l.hit(a), Ok(()));
        assert_eq!(l.hit(a), Ok(()));
        assert_eq!(l.hit(a), Err(60));
        assert_eq!(l.hit(b), Ok(()), "other IPs are separate");
        tokio::time::advance(Duration::from_secs(45)).await;
        assert_eq!(l.hit(a), Err(15));
        tokio::time::advance(Duration::from_secs(15)).await;
        assert_eq!(l.hit(a), Ok(()), "new window");
    }

    #[test]
    fn zero_disables() {
        let l = LoginLimiter::new(0);
        for _ in 0..100 {
            assert_eq!(l.hit(None), Ok(()));
        }
    }
}
