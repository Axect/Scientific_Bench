# Argument: write or update or read
if [ "$1" = "write" ]; then
  hyperfine -w 3 \
      --export-markdown bench_db_write.md \
      --export-csv bench_db_write.csv \
      "./nativedb_test/target/release/nativedb_test write" \
      "./jammdb_test/target/release/jammdb_test write" \
      "./rkyv_test/target/release/rkyv_test write"
elif [ "$1" = "update" ]; then
  hyperfine -w 3 \
      --export-markdown bench_db_update.md \
      --export-csv bench_db_update.csv \
      "./nativedb_test/target/release/nativedb_test update" \
      "./jammdb_test/target/release/jammdb_test update" \
      "./rkyv_test/target/release/rkyv_test update"
elif [ "$1" = "read" ]; then
  hyperfine -w 3 \
      --export-markdown bench_db_read.md \
      --export-csv bench_db_read.csv \
      "./nativedb_test/target/release/nativedb_test read" \
      "./jammdb_test/target/release/jammdb_test read" \
      "./rkyv_test/target/release/rkyv_test read"
fi
