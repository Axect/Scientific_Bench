| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./nativedb_test/target/release/nativedb_test write` | 243.0 ± 3.5 | 239.6 | 253.6 | 1.00 |
| `./jammdb_hash/target/release/jammdb_hash write` | 368.4 ± 6.0 | 360.5 | 376.6 | 1.52 ± 0.03 |
| `./files_hash/target/release/files_hash write` | 3537.6 ± 97.1 | 3434.4 | 3674.5 | 14.56 ± 0.45 |
| `python numpy_test/test.py write` | 856.2 ± 34.0 | 802.2 | 904.4 | 3.52 ± 0.15 |
