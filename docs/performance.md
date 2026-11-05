# Performance measurements

Circuits use official Noir 1.0.0 and Barretenberg 6.0.0 with the `evm` verifier
target.
Raw reports retain tool identities, operands and source hashes. See
[benchmark setup](../benchmarks/README.md) to reproduce measurements.

## Circuit gates

Standalone consumers use private runtime inputs and public outputs. Counts include
range constraints and backend overhead. Compare within each type and adapter,
because the input/output representations differ between types.

| Type | Operation | Bigint baseline | Specialized |
| --- | --- | ---: | ---: |
| u128 | Floor | 4,112 | 3,063 |
| u128 | Quotient and remainder | 4,189 | 3,065 |
| u128 | Ceiling | 4,213 | 3,091 |
| U256 | Floor | 3,098 | 2,958 |
| U256 | Quotient and remainder | 3,104 | 2,964 |
| U256 | Ceiling | 3,180 | 3,046 |

Circuit size is input-independent for a given adapter. It is not a proof time or padded proving-domain measurement.

## Constrained execution time

Each sample executes eight precompiled calls with independent Python expected-result
assertions. Runs use two warmups and 15 alternating samples per implementation.
These retained results came from separate u128/U256 runs on an AMD EPYC 7R13.

| Type | Baseline median | Specialized median | Reduction |
| --- | ---: | ---: | ---: |
| u128 | 23.398 ms | 12.877 ms | 44.96% |
| U256 | 33.060 ms | 14.266 ms | 56.85% |

Timings include CLI startup, artifact/input loading and ACVM witness execution.
They exclude compilation, gate counting, witness serialization and proving.
They depend on machine/load and do not isolate the division helper. Batch gates
include output assertions and differ from standalone counts.

Reports preserve historical observations. Current scripts import bigint from
upstream `main`, which can advance. Match dependency versions, toolchains,
adapters and operands before comparing new results with retained reports.
