use mul_div::{mul_div_ceil, mul_div_floor, mul_div_rem};
// Adapter protocol: 0 = remainder, 1 = floor, 2 = ceiling.
fn compute(a: u128, b: u128, d: u128, operation: u8) -> (u128, u128) {
    if operation == 2 {
        (mul_div_ceil(a, b, d), 0)
    } else if operation == 1 {
        (mul_div_floor(a, b, d), 0)
    } else {
        mul_div_rem(a, b, d)
    }
}
fn main(a: u128, b: u128, d: u128, operation: u8, expected_q: u128, expected_r: u128) {
    // The host adapter checks computed answers against independent reference values.
    let (q, r) = compute(a, b, d, operation);
    assert_eq(q, expected_q);
    assert_eq(r, expected_r);
}
