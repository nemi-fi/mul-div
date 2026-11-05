use bigint::{BigInt, U256};
use mul_div::MulDiv;

unconstrained fn main(
    a: [u8; 32],
    b: [u8; 32],
    d: [u8; 32],
    operation: u8,
    expected_q: [u8; 32],
    expected_r: [u8; 32],
) {
    let a = U256::from_le_bytes(a);
    let b = U256::from_le_bytes(b);
    let d = U256::from_le_bytes(d);
    let (q, r) = if operation == 2 {
        (a.mul_div_ceil(b, d), U256::zero())
    } else if operation == 1 {
        (a.mul_div_floor(b, d), U256::zero())
    } else {
        a.mul_div_rem(b, d)
    };
    assert_eq(q.to_le_bytes(), expected_q);
    assert_eq(r.to_le_bytes(), expected_r);
}
