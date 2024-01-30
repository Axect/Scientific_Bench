| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `./nativedb_test/target/release/nativedb_test read` | 9.4 ± 0.7 | 8.8 | 16.1 | 1.00 |
| `./jammdb_test/target/release/jammdb_test read` | 35.6 ± 1.8 | 31.2 | 40.9 | 3.79 ± 0.34 |
| `./files/target/release/files read` | 14033.4 ± 608.5 | 13191.4 | 14751.8 | 1492.71 ± 127.70 |
| `python numpy_test/test.py read` | 3101.8 ± 159.0 | 2887.6 | 3276.5 | 329.93 ± 29.63 |
