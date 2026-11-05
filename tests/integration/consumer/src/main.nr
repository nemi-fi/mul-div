use bigint::{BigInt, U256};
use mul_div::{mul_div_ceil, mul_div_floor, mul_div_rem, MulDiv};

fn main(a: u128, b: u128, d: u128) -> pub (u128, u128, u128) {
    let (q, r) = mul_div_rem(a, b, d);
    assert_eq(mul_div_floor(a, b, d), q);
    let (wide_q, wide_r) = U256::from(a).mul_div_rem(U256::from(b), U256::from(d));
    assert_eq((wide_q, wide_r), (U256::from(q), U256::from(r)));
    (q, r, mul_div_ceil(a, b, d))
}

#[test]
fn downstream_wide_ratio() {
    assert_eq(
        main(
            0x10000000000000001,
            0x10000000000000001,
            0x10000000000000000,
        ),
        (0x10000000000000002, 1, 0x10000000000000003),
    );
}

#[test]
fn downstream_u128_trait() {
    assert_eq((7 as u128).mul_div_rem(9, 4), (15, 3));
    assert_eq((7 as u128).mul_div_floor(9, 4), 15);
    assert_eq((7 as u128).mul_div_ceil(9, 4), 16);
}

#[test]
fn downstream_u256_trait() {
    let m = U256::from_le_bytes([255; 32]);
    assert_eq(m.mul_div_rem(m, m), (m, U256::zero()));
    assert_eq(m.mul_div_floor(m, m), m);
    assert_eq(m.mul_div_ceil(m, m), m);
    let a = m - U256::one();
    let d = m - U256::from(2 as u128);
    assert_eq(a.mul_div_rem(a, d), (m, U256::one()));
}
