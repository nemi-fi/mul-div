#![allow(dead_code)]
use num_bigint::BigUint;

use std::{path::PathBuf, process::Command, sync::OnceLock};

pub fn tools() -> &'static (PathBuf, PathBuf) {
    static TOOLS: OnceLock<(PathBuf, PathBuf)> = OnceLock::new();
    TOOLS.get_or_init(|| {
        let nargo = PathBuf::from(std::env::var("NARGO").unwrap_or_else(|_| "nargo".into()));
        let executor =
            PathBuf::from(std::env::var("NOIR_EXECUTE").unwrap_or_else(|_| "noir-execute".into()));
        let version = Command::new(&nargo).arg("--version").output().unwrap();
        assert!(String::from_utf8_lossy(&version.stdout).contains("nargo version = 1.0.0\n"));
        assert!(
            Command::new(&nargo)
                .args(["compile", "--workspace", "--deny-warnings"])
                .status()
                .unwrap()
                .success()
        );
        (nargo, executor)
    })
}

/// None denotes the API's rejected domain, computed without fixed-width arithmetic.
pub fn division(a: u128, b: u128, d: u128) -> Option<(u128, u128)> {
    if d == 0 {
        return None;
    }
    let product = BigUint::from(a) * BigUint::from(b);
    let divisor = BigUint::from(d);
    let q = &product / &divisor;
    if q > BigUint::from(u128::MAX) {
        return None;
    }
    Some((
        u128::try_from(q).unwrap(),
        u128::try_from(product % divisor).unwrap(),
    ))
}

pub fn division_boundaries() -> Vec<(u128, u128, u128)> {
    let m = u128::MAX;
    let w = 1u128 << 64;
    vec![
        (0, 0, 0),
        (0, m, 0),
        (m, 0, 0),
        (1, 1, 0),
        (0, m, 1),
        (m, 0, m),
        (0, 0, m),
        (1, 1, 2),
        (7, 9, 4),
        (m, 1, 1),
        (m, m, m),
        (w, w, 1),
        (w, w, 2),
        (w, w, w),
        (w + 1, w + 1, w),
        (m, m, m - 1),
        (m, 2, 1),
        (m, 2, 2),
        (m, 2, 3),
        (w, w, w - 1),
        (w, w, w + 1),
        (m - 1, m - 1, m - 2),
        // Retained normalization/correction fixtures from the independent model.
        (0, 0, 18446744073709551616),
        (0, 0, 170141183460469231731687303715884105728),
        (1, 18446744073709551616, 18446744073709551617),
        (
            1,
            170141183460469231731687303715884105728,
            170141183460469231731687303715884105729,
        ),
        (
            2,
            170141183460469231731687303715884105728,
            18446744073709551617,
        ),
        (
            18446744073709551616,
            170141183460469231731687303715884105728,
            170141183460469231731687303715884105729,
        ),
        (
            18446744073709551617,
            170141183460469231731687303715884105727,
            18446744073709551617,
        ),
        (
            170141183460469231731687303715884105727,
            340282366920938463463374607431768211454,
            340282366920938463463374607431768211454,
        ),
        (
            177688935058149085577775968461259923061,
            107543167803409442633630436820249481746,
            56724143819944981891978735205918094561,
        ),
        (
            151657525668555594596929511848685794740,
            227425096319174033932326311921144600730,
            188484088415838494862360551510607733977,
        ),
        (
            269896647375605271303154059427744105189,
            96530978226248310919428960225006206672,
            89367370553326359708425354988609983937,
        ),
        (
            27717914300549216374447279040427350009,
            69776569747443122320284741838032158718,
            224508990309906061808299784799804794123,
        ),
        (
            188951178823576345429165047763863935604,
            329593685291808425650442688440374590942,
            208308503050791093311199142760431812076,
        ),
        (
            209237539720136749106924430178954244415,
            101690174718494383333567690419877994383,
            117755531770556146676219234839975185588,
        ),
        (
            327860485764577485418718968713688651041,
            157110879210500047019170153482038269456,
            173720247647676566644251301680011549585,
        ),
    ]
}

/// Independent ceiling reference: use unbounded numerator adjustment before division.
pub fn ceiling(a: u128, b: u128, d: u128) -> Option<u128> {
    if d == 0 {
        return None;
    }
    let divisor = BigUint::from(d);
    let q = (BigUint::from(a) * BigUint::from(b) + &divisor - BigUint::from(1u8)) / divisor;
    u128::try_from(q).ok()
}

/// For 1 <= k <= 2^32, a*b = MAX*d + k^2 and 0 < k^2 < d.
pub fn ceiling_overflow(k: u32) -> (u128, u128, u128) {
    let k = u128::from(k) + 1;
    (u128::MAX - k, u128::MAX - k, u128::MAX - 2 * k)
}

/// Full-width U256 oracle; errors classify the public contract independently.
pub fn u256_division(
    a: &BigUint,
    b: &BigUint,
    d: &BigUint,
    ceil: bool,
) -> Result<(BigUint, BigUint), &'static str> {
    if *d == BigUint::from(0u8) {
        return Err("mul_div: zero denominator");
    }
    let product = a * b;
    let mut q = &product / d;
    let r = product % d;
    let limit = BigUint::from(1u8) << 256;
    if q >= limit {
        return Err("mul_div: quotient overflow");
    }
    if ceil && r != BigUint::from(0u8) {
        q += BigUint::from(1u8);
        if q >= limit {
            return Err("mul_div: ceiling overflow");
        }
    }
    Ok((q, r))
}

pub fn u256_bytes(value: &BigUint) -> Vec<u8> {
    let mut bytes = value.to_bytes_le();
    assert!(bytes.len() <= 32);
    bytes.resize(32, 0);
    bytes
}

pub fn u256_boundaries() -> Vec<(BigUint, BigUint, BigUint)> {
    let z = BigUint::from(0u8);
    let one = BigUint::from(1u8);
    let two = BigUint::from(2u8);
    let w: BigUint = &one << 128;
    let m: BigUint = (&one << 256) - &one;
    let mut cases = vec![
        (z.clone(), z.clone(), z.clone()),
        (z.clone(), m.clone(), z.clone()),
        (m.clone(), z.clone(), z.clone()),
        (one.clone(), one.clone(), z.clone()),
        (z.clone(), m.clone(), one.clone()),
        (m.clone(), z.clone(), m.clone()),
        (7u8.into(), 9u8.into(), 4u8.into()),
        (m.clone(), one.clone(), one.clone()),
        (m.clone(), m.clone(), m.clone()),
        (w.clone(), w.clone(), two.clone()),
        (&w + &one, &w + &one, w.clone()),
        (m.clone(), m.clone(), &m - &one),
        (m.clone(), two.clone(), one.clone()),
        (&m - &one, &m - &one, &m - &two),
    ];
    for line in include_str!("../regressions/u256_mul_div.txt")
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        let values: Vec<BigUint> = line
            .split(',')
            .map(|value| value.parse().unwrap())
            .collect();
        assert_eq!(values.len(), 3);
        cases.push((values[0].clone(), values[1].clone(), values[2].clone()));
    }
    cases
}
