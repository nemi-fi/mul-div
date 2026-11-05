use mul_div::{mul_div_ceil, mul_div_floor, mul_div_rem};

// Pro-rata asset redemption: shares * total_assets / total_shares.
fn main(shares: u128, total_assets: u128, total_shares: u128) -> pub (u128, u128, u128) {
    let (assets, remainder) = mul_div_rem(shares, total_assets, total_shares);
    assert_eq(assets, mul_div_floor(shares, total_assets, total_shares));
    // An application requiring upward rounding can account for the fractional asset.
    let rounded_assets = mul_div_ceil(shares, total_assets, total_shares);
    (assets, remainder, rounded_assets)
}
