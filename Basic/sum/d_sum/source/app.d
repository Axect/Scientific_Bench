import std.stdio : writeln;
import std.range : iota;
import std.json;
import std.algorithm : sum, map;
import std.array : array;
import std.file : write;
import core.simd;

import simplebench;

float sum_loop(immutable float[] arr) {
    float res = 0;
    foreach(el; arr)
        res += el;
    return res;
}

float sum_func(immutable float[] arr) {
    return sum(arr);
}

float sum_simd(immutable float[] arr) {
    size_t n = arr.length;
    float result = 0.0;

    float8 vec;
    // Process the array in simds of 8 elements using SIMD
    for (size_t i = 0; i + 8 <= n; i += 8) {
        // Load 8 floats into a SIMD register
        auto tmp = ([arr[i], arr[i + 1], arr[i + 2], arr[i + 3],
                            arr[i + 4], arr[i + 5], arr[i + 6], arr[i + 7]]);
        vec[] += tmp[];
    }

    // Sum the elements of the SIMD register (horizontal sum)
    result += vec[0] + vec[1] + vec[2] + vec[3] +
              vec[4] + vec[5] + vec[6] + vec[7];

    // Handle any remaining elements that don't fit into a full SIMD vector
    for (size_t i = n - (n % 8); i < n; i++) {
        result += arr[i];
    }

    return result;
}

void test_sum_loop(float N)(ref Bencher bencher){
  immutable float[] arr = N.iota.array;
  bencher.iter((){
      return sum_loop(arr);
  });
}

void test_sum_func(float N)(ref Bencher bencher){
  immutable float[] arr = N.iota.array;
  bencher.iter((){
      return sum_func(arr);
  });
}

void test_sum_simd(float N)(ref Bencher bencher){
  immutable float[] arr = N.iota.array;
  bencher.iter((){
      return sum_simd(arr);
  });
}

void main()
{
  JSONValue js_s;
  js_s.array = [];
  static foreach(float N; iota(0,100_001,10_000)) {
    js_s.array ~= BenchMain!(test_sum_loop!N, test_sum_func!N, test_sum_simd!N).toJSON;
  }
  write("data_small.json", js_s.toPrettyString(JSONOptions.specialFloatLiterals));

  JSONValue js_l;
  js_l.array = [];
  static foreach(float N; iota(0,100_000_001,10_000_000)) {
    js_l.array ~= BenchMain!(test_sum_loop!N, test_sum_func!N, test_sum_simd!N).toJSON;
  }
  write("data_large.json", js_s.toPrettyString(JSONOptions.specialFloatLiterals));
}
