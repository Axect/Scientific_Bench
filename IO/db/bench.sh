# Argument: write or update or read
if [ "$1" = "write" ]; then
  hyperfine -w 3 \
      --export-markdown bench_write.md \
      --export-csv bench_write.csv \
      "./nativedb_test/target/release/nativedb_test write" \
      "./jammdb_test/target/release/jammdb_test write" \
      "./files/target/release/files write" \
      "python numpy_test/test.py write"
elif [ "$1" = "update" ]; then
  hyperfine -w 3 \
      --export-markdown bench_update.md \
      --export-csv bench_update.csv \
      "./nativedb_test/target/release/nativedb_test update" \
      "./jammdb_test/target/release/jammdb_test update" \
      "./files/target/release/files update" \
      "python numpy_test/test.py update"
elif [ "$1" = "read" ]; then
  hyperfine -w 3 \
      --export-markdown bench_read.md \
      --export-csv bench_read.csv \
      "./nativedb_test/target/release/nativedb_test read" \
      "./jammdb_test/target/release/jammdb_test read" \
      "./files/target/release/files read" \
      "python numpy_test/test.py read"
fi
