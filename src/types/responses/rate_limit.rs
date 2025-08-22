use serde::{Deserialize, Serialize};

use crate::enums::{RateLimitType, RateLimitInterval};

/**
 * Rate limit information from exchange.
 * 
 * # Fields
 * - `rate_limit_type`: The type of rate limit (REQUEST_WEIGHT, ORDERS, etc.).
 * - `interval`: The time interval for the rate limit.
 * - `interval_num`: The multiplier for the interval (e.g., 1 minute).
 * - `limit`: The maximum number of requests allowed per interval.
 * - `count`: Optional current count of requests in this interval.
 */
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct RateLimit {
    pub rate_limit_type: RateLimitType,
    pub interval: RateLimitInterval,
    pub interval_num: u32,
    pub limit: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
}