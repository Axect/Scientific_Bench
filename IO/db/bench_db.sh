# Argument: write or update or read
if [ "$1" = "write" ]; then
  hyperfine -w 3 \
      --export-markdown bench_db_write.md \
      --export-csv bench_db_write.csv \
      "./nativedb_test/target/release/nativedb_test write" \
      "./jammdb_test/target/release/jammdb_test write" \
      "./jammdb_postcard/target/release/jammdb_postcard write"
elif [ "$1" = "update" ]; then
  hyperfine -w 3 \
      --export-markdown bench_db_update.md \
      --export-csv bench_db_update.csv \
      "./nativedb_test/target/release/nativedb_test update" \
      "./jammdb_test/target/release/jammdb_test update" \
      "./jammdb_postcard/target/release/jammdb_postcard update"
elif [ "$1" = "read" ]; then
  hyperfine -w 3 \
      --export-markdown bench_db_read.md \
      --export-csv bench_db_read.csv \
      "./nativedb_test/target/release/nativedb_test read" \
      "./jammdb_test/target/release/jammdb_test read" \
      "./jammdb_postcard/target/release/jammdb_postcard read"
fi
