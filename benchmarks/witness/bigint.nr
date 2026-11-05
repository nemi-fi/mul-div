// Preserved constrained implementation from bacd1c0; benchmark-only.
use bigint::{BigInt, U256};

pub fn mul_div_rem(a: u128, b: u128, d: u128) -> (u128, u128) {
    assert(d != 0, "mul_div: zero denominator");
    let (q, r) = (U256::from(a) * U256::from(b)).div_rem(&U256::from(d));
    assert(
        q <= U256::from(0xffffffffffffffffffffffffffffffff as u128),
        "mul_div: quotient overflow",
    );
    // q is checked above; r < d <= u128::MAX is enforced by bigint division.
    (low_u128(q), low_u128(r))
}

fn low_u128(value: U256) -> u128 {
    let bytes = value.to_le_bytes();
    let mut result: u128 = 0;
    for i in 0..16 {
        result = result | ((bytes[i] as u128) << ((8 * i) as u128));
    }
    result
}


pub fn mul_div_floor(a: u128, b: u128, d: u128) -> u128 {
    mul_div_rem(a, b, d).0
}

pub fn mul_div_ceil(a: u128, b: u128, d: u128) -> u128 {
    let (q, r) = mul_div_rem(a, b, d);
    if r == 0 {
        q
    } else {
        assert(q < 0xffffffffffffffffffffffffffffffff, "mul_div: ceiling overflow");
        q + 1
    }
}
