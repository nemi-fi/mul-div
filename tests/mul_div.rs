use hegel::{TestCase, generators as gs};
use num_bigint::BigUint;
use std::process::Command;
mod common;

fn check(a: u128, b: u128, d: u128) {
    let (_, executor) = common::tools();
    let expected = common::division(a, b, d);
    if let Some((q, r)) = expected {
        assert_eq!(
            BigUint::from(a) * BigUint::from(b),
            BigUint::from(q) * BigUint::from(d) + BigUint::from(r)
        );
        assert!(r < d);
    }
    let dir = tempfile::tempdir().unwrap();
    let prover = dir.path().join("Prover.toml");
    // Adapter protocol: 0 = remainder, 1 = floor, 2 = ceiling.
    for operation in 0..3 {
        let result = if operation == 2 {
            common::ceiling(a, b, d).map(|q| (q, 0))
        } else {
            expected
        };
        let (q, r) = result.unwrap_or((0, 0));
        std::fs::write(&prover, format!("a = \"{a}\"\nb = \"{b}\"\nd = \"{d}\"\noperation = {operation}\nexpected_q = \"{q}\"\nexpected_r = \"{}\"\n", if operation == 1 { 0 } else { r })).unwrap();
        for mode in ["constrained", "unconstrained"] {
            let output = Command::new(executor)
                .args([
                    "--artifact-path",
                    &format!("target/division_{mode}.json"),
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
                result.is_some(),
                "{mode}, operation={operation}, ({a}, {b}, {d}): {diagnostic}"
            );
            if result.is_none() {
                let reason = if d == 0 {
                    "mul_div: zero denominator"
                } else if operation == 2 && expected.is_some() {
                    "mul_div: ceiling overflow"
                } else {
                    "mul_div: quotient overflow"
                };
                assert!(
                    diagnostic.contains(reason),
                    "Unexpected execution failure: {diagnostic}"
                );
            }
        }
    }
}

#[test]
fn boundaries_and_regressions() {
    for (a, b, d) in common::division_boundaries() {
        check(a, b, d);
    }
    for line in include_str!("regressions/mul_div.txt")
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let values: Vec<u128> = line.split(',').map(|v| v.parse().unwrap()).collect();
        assert_eq!(values.len(), 3);
        check(values[0], values[1], values[2]);
    }
}

#[hegel::test]
fn generated_division(tc: TestCase) {
    let a = tc.draw(gs::integers::<u128>());
    let b = tc.draw(gs::integers::<u128>());
    let d = tc.draw(gs::integers::<u128>());
    check(a, b, d);
    // Every generated example exercises deliberate domains, even after shrinking.
    check(a, b, a.max(b).max(1)); // fitting quotient, usually a wide product
    check(a, b, 0);
    check(0, b, d.max(1));
    check(a, b.max(1), b.max(1)); // exact division
    check(a | (1 << 127), 2, 1); // guaranteed quotient overflow
    let small = tc.draw(gs::integers::<u64>()) as u128;
    check(a, small, small + 1); // small divisor with a potentially wide product
    check(small, 2, 3); // tiny divisor, exact or inexact
    check(small * 2 + 1, 1, 2); // guaranteed nonzero remainder
    let k = tc.draw(gs::integers::<u32>());
    let (a, b, d) = common::ceiling_overflow(k);
    assert_eq!(common::division(a, b, d).unwrap().0, u128::MAX);
    assert!(common::ceiling(a, b, d).is_none());
    check(a, b, d);
}
