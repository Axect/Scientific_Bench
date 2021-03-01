use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use packed_simd_2::f64x8;

fn for_sum(v: &[f64]) -> f64 {
    let mut s = 0f64;
    for t in v {
        s += *t;
    }
    s
}

fn simd_sum(x: &[f64]) -> f64 {
    let mut sum = f64x8::splat(0.);
    for i in (0 .. x.len()).step_by(8) {
        sum += f64x8::from_slice_unaligned(&x[i..]);
    }
    sum.sum()
}

fn chunk_sum(v: &[f64]) -> f64 {
    v.chunks(8).map(|t| t.iter().sum::<f64>()).sum()
}

fn bench_sums(c: &mut Criterion) {
    let mut group = c.benchmark_group("sum");
    let small_range = (1 .. 11).map(|t| 10000 * t);
    let large_range = (1 .. 11).map(|t| 1000_0000 * t);
    for i in small_range.chain(large_range) {
        let x: Vec<f64> = (0 .. i).map(|t| t as f64).collect();
        group.bench_with_input(BenchmarkId::new("for_sum", i), black_box(&x), 
                               |b, x| b.iter(|| for_sum(&x)));
        group.bench_with_input(BenchmarkId::new("simd_sum", i), black_box(&x), 
                               |b, x| b.iter(|| simd_sum(&x)));

        group.bench_with_input(BenchmarkId::new("chunk_sum", i), black_box(&x), 
                               |b, x| b.iter(|| chunk_sum(&x)));
    }
    group.finish();
}

criterion_group!(benches, bench_sums);
criterion_main!(benches);
