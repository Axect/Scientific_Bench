
## Test command

```sh
pytest --benchmark-only matmul.py --benchmark-save=bench

py.test-benchmark compare .benchmarks/Linux-CPython-3.9-64bit/0001_bench.json --csv=../bench_python.csv
```
