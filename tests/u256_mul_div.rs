use hegel::{TestCase, generators as gs};
use num_bigint::BigUint;
use std::process::Command;
mod common;

fn check(a: &BigUint, b: &BigUint, d: &BigUint) {
    let (_, executor) = common::tools();
    let directory = tempfile::tempdir().unwrap();
    let prover = directory.path().join("Prover.toml");
    for operation in 0..3 {
        let expected = common::u256_division(a, b, d, operation == 2);
        let (q, r) = expected.clone().unwrap_or_default();
        let r = if operation == 0 { r } else { 0u8.into() };
        std::fs::write(
            &prover,
            format!(
                "a = {:?}\nb = {:?}\nd = {:?}\noperation = {operation}\nexpected_q = {:?}\nexpected_r = {:?}\n",
                common::u256_bytes(a), common::u256_bytes(b), common::u256_bytes(d),
                common::u256_bytes(&q), common::u256_bytes(&r)
            ),
        ).unwrap();
        for mode in ["constrained", "unconstrained"] {
            let output = Command::new(executor)
                .args([
                    "--artifact-path",
                    &format!("target/u256_{mode}.json"),
                    "--prover-file",
                ])
                .arg(&prover)
                .output()
                .unwrap();
            let diagnostic = format!(
                "{}\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(
                output.status.success(),
                expected.is_ok(),
                "{mode}, op {operation}, ({a}, {b}, {d}): {diagnostic}"
            );
            if let Err(reason) = expected {
                assert!(
                    diagnostic.contains(reason),
                    "Unexpected failure: {diagnostic}"
                );
            }
        }
    }
}

#[test]
fn full_width_boundaries() {
    for (a, b, d) in common::u256_boundaries() {
        check(&a, &b, &d);
    }
}

#[hegel::test]
fn generated_u256(tc: TestCase) {
    let a = BigUint::from(tc.draw(gs::integers::<u128>()))
        + (BigUint::from(tc.draw(gs::integers::<u128>())) << 128);
    let b = BigUint::from(tc.draw(gs::integers::<u128>()))
        + (BigUint::from(tc.draw(gs::integers::<u128>())) << 128);
    let d = BigUint::from(tc.draw(gs::integers::<u128>()))
        + (BigUint::from(tc.draw(gs::integers::<u128>())) << 128);
    check(&a, &b, &d);
    for bits in [64usize, 128, 192] {
        let mask = (BigUint::from(1u8) << bits) - BigUint::from(1u8);
        let divisor = (&d & mask).max(1u8.into());
        check(&a, &(&b % &divisor), &divisor);
    }
    check(&a, &b, &a.clone().max(b.clone()).max(1u8.into()));
    check(&a, &b, &b.clone().max(1u8.into()));
    // (M-k)^2 = M*(M-2k) + k^2: floor fits, ceiling overflows.
    let m = (BigUint::from(1u8) << 256) - BigUint::from(1u8);
    let k = BigUint::from(tc.draw(gs::integers::<u64>())) + BigUint::from(1u8);
    check(&(&m - &k), &(&m - &k), &(&m - &k * BigUint::from(2u8)));
}
