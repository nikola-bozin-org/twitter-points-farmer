use std::time::Duration;

pub trait RateLimiter {
    fn new(requests_amount: u8, limit: Duration) -> Self;
    fn set_requests_amount(&mut self, requests_amount: u8);
    fn set_limit(&mut self, limit: Duration);
}

#[derive(Clone)]
pub struct RateLimiterConfig {
    pub requests_amount: u8,
    pub time_frame: Duration,
}

impl RateLimiter for RateLimiterConfig {
    fn new(requests_amount: u8, limit: Duration) -> Self {
        Self {
            requests_amount,
            time_frame: limit,
        }
    }

    fn set_requests_amount(&mut self, requests_amount: u8) {
        self.requests_amount = requests_amount;
    }

    fn set_limit(&mut self, limit: Duration) {
        self.time_frame = limit;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rate_limiter_config() {
        let limiter_config = RateLimiterConfig::new(10, Duration::from_secs(60));
        assert_eq!(limiter_config.requests_amount, 10);
        assert_eq!(limiter_config.time_frame, Duration::from_secs(60));
    }

    #[test]
    fn test_set_requests_amount() {
        let mut limiter_config = RateLimiterConfig::new(10, Duration::from_secs(60));
        limiter_config.set_requests_amount(5);
        assert_eq!(limiter_config.requests_amount, 5);
    }

    #[test]
    fn test_set_limit() {
        let mut limiter_config = RateLimiterConfig::new(10, Duration::from_secs(60));
        limiter_config.set_limit(Duration::from_secs(30));
        assert_eq!(limiter_config.time_frame, Duration::from_secs(30));
    }
}
