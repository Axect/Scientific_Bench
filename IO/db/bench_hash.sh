# Argument: write or update or read
if [ "$1" = "write" ]; then
  hyperfine -w 3 \
      --export-markdown bench_write.md \
      --export-csv bench_write.csv \
      "./nativedb_test/target/release/nativedb_test write" \
      "./jammdb_hash/target/release/jammdb_hash write" \
      "./files_hash/target/release/files_hash write" \
      "python numpy_test/test.py write"
elif [ "$1" = "update" ]; then
  hyperfine -w 3 \
      --export-markdown bench_hash_update.md \
      --export-csv bench_hash_update.csv \
      "./nativedb_test/target/release/nativedb_test update" \
      "./jammdb_hash/target/release/jammdb_hash update" \
      "./files_hash/target/release/files_hash update" \
      "python numpy_test/test.py update"
elif [ "$1" = "read" ]; then
  hyperfine -w 3 \
      --export-markdown bench_hash_read.md \
      --export-csv bench_hash_read.csv \
      "./nativedb_test/target/release/nativedb_test read" \
      "./jammdb_hash/target/release/jammdb_hash read" \
      "./files_hash/target/release/files_hash read" \
      "python numpy_test/test.py read"
fi
