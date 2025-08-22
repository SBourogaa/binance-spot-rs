/**
 * Binance Spot-API Enum Definitions
 *
 * This module contains all the enums used by the Binance Spot API.
 * These enums are **in sync with the Binance API released on `2025-06-11`**.
 * 
 * If the API changes, bump the version in `BINANCE_ENUM_VERSION` and update the enums
 * accordingly.
 */
pub const BINANCE_ENUM_VERSION: &str = "2025-06-11";


mod symbol_status;
mod trade_group;
mod permission;
mod order_status;
mod order_list_status;
mod order_list_order_status;
mod contingency_type;
mod allocation_type;
mod order_type;
mod order_response_type;
mod working_floor;
mod order_side;
mod time_in_force;
mod rate_limit_type;
mod rate_limit_interval;
mod self_trade_prevention_mode;
mod cancel_restrictions;
mod cancel_replace_mode;
mod cancel_replace_status;
mod order_rate_limit_exceeded_mode;

pub use symbol_status::SymbolStatus;
pub use trade_group::TradeGroup;
pub use permission::Permission;
pub use order_status::OrderStatus;
pub use order_list_status::OrderListStatus;
pub use order_list_order_status::OrderListOrderStatus;
pub use contingency_type::ContingencyType;
pub use allocation_type::AllocationType;
pub use order_type::OrderType;
pub use order_response_type::OrderResponseType;
pub use working_floor::WorkingFloor;
pub use order_side::OrderSide;
pub use time_in_force::TimeInForce;
pub use rate_limit_type::RateLimitType;
pub use rate_limit_interval::RateLimitInterval;
pub use self_trade_prevention_mode::SelfTradePreventionMode;
pub use cancel_restrictions::CancelRestrictions;
pub use cancel_replace_mode::CancelReplaceMode;
pub use cancel_replace_status::CancelReplaceStatus;
pub use order_rate_limit_exceeded_mode::OrderRateLimitExceededMode;
