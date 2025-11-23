use std::time::{SystemTime, UNIX_EPOCH};

use rand::Rng;

use crate::types::{OrderSide, TickSize};

/// Get current Unix timestamp in seconds
pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Calculate maker and taker amounts for an order
pub fn calculate_order_amounts(
    price: f64,
    size: f64,
    side: OrderSide,
    tick_size: TickSize,
) -> (String, String) {
    const SIZE_DECIMALS: u32 = 2; // shares are in 2 decimals

    let tick_decimals = tick_size.decimals();

    // Round price to tick size
    let price_rounded = round_to_decimals(price, tick_decimals);

    // Round size to 2 decimals
    let size_rounded = round_to_decimals(size, SIZE_DECIMALS);

    // Calculate cost
    let cost = price_rounded * size_rounded;
    let cost_rounded = round_to_decimals(cost, tick_decimals);

    // Convert to raw amounts (no decimals)
    let share_amount = to_raw_amount(size_rounded, SIZE_DECIMALS);
    let cost_amount = to_raw_amount(cost_rounded, SIZE_DECIMALS);

    match side {
        OrderSide::Buy => {
            // BUY: maker pays USDC, receives shares
            (cost_amount, share_amount)
        }
        OrderSide::Sell => {
            // SELL: maker pays shares, receives USDC
            (share_amount, cost_amount)
        }
    }
}

/// Round a float to specified decimal places
fn round_to_decimals(value: f64, decimals: u32) -> f64 {
    let multiplier = 10_f64.powi(decimals as i32);
    (value * multiplier).round() / multiplier
}

/// Convert float to raw integer amount
fn to_raw_amount(value: f64, decimals: u32) -> String {
    let multiplier = 10_f64.powi(decimals as i32);
    let raw = (value * multiplier).floor() as u128;
    raw.to_string()
}

/// Generate random salt for orders
pub fn generate_salt() -> String {
    rand::rng().random::<u128>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_order_amounts_buy() {
        let (maker, taker) =
            calculate_order_amounts(0.52, 100.0, OrderSide::Buy, TickSize::Hundredth);

        // BUY: maker = cost (5200), taker = shares (10000)
        assert_eq!(maker, "5200");
        assert_eq!(taker, "10000");
    }

    #[test]
    fn test_calculate_order_amounts_sell() {
        let (maker, taker) =
            calculate_order_amounts(0.52, 100.0, OrderSide::Sell, TickSize::Hundredth);

        // SELL: maker = shares (10000), taker = cost (5200)
        assert_eq!(maker, "10000");
        assert_eq!(taker, "5200");
    }
}
