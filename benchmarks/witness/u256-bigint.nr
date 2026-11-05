// Preserved U256 computation from 9248d54; benchmark-only.
use bigint::{BigInt, U256, U512};

pub fn mul_div_rem(a: U256, rhs: U256, denominator: U256) -> (U256, U256) {
    a.validate_in_range();
    rhs.validate_in_range();
    denominator.validate_in_range();
    assert(denominator != U256::zero(), "mul_div: zero denominator");
    let product = U512::from(a) * U512::from(rhs);
    let (q, r) = product.div_rem(&U512::from(denominator));
    let maximum = U256::from_le_bytes([255; 32]);
    assert(q <= U512::from(maximum), "mul_div: quotient overflow");
    // q was checked above; r < denominator guarantees that r also fits U256.
    let quotient = q.get_limbs();
    let remainder = r.get_limbs();
    (
        U256::from_limbs([quotient[0], quotient[1], quotient[2]]),
        U256::from_limbs([remainder[0], remainder[1], remainder[2]]),
    )
}
