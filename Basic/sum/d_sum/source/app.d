import std.stdio : writeln;
import std.range : iota;
import std.json;
import std.algorithm : sum, map;
import std.array : array;
import std.file : write;
import ldc.simd;
version(LDC) {
    pragma(LDC_no_moduleinfo);
    import ldc.attributes;
} else {
    import core.attribute;
}

import simplebench;

enum win_size = 32;

T sum_autov(T)(T[] a) {
    T[64/T.sizeof] sa = 0;
    const partial = a.length % sa.length;
    sa[0 .. partial] = a[0 .. partial];
    for (a = a[partial .. $] ; a.length >= sa.length; a = a[sa.length .. $])
        sa[] += a[0 .. sa.length];
    T result = 0;
    foreach(x; sa[])
        result += x;
    return result;
}

T sum_loop(T)(T[] arr) {
    T res = 0;
    foreach(ref el; arr)
        res += el;
    return res;
}

T sum_func(T)(T[] arr) {
    return sum(arr);
}

T sum_simd(T)(T[] arr) {
    size_t n = arr.length;
    T result = 0;

    __vector(T[win_size]) vec = 0;
    // Process the array in simds of 32 elements using SIMD
    for (size_t i = 0; i + win_size <= n; i += win_size) {
        // Load 8 doubles into a SIMD register
        __vector(T[win_size]) tmp = 0;
        static foreach(k; 0..win_size)
            tmp[k] = arr[i + k];
        vec[] += tmp[];
    }

    // Sum the elements of the SIMD register (horizontal sum)
    static foreach(l; 0..win_size)
        result += vec[l];

    // Handle any remaining elements that don't fit into a full SIMD vector
    for (size_t i = n - (n % win_size); i < n; i++) {
        result += arr[i];
    }

    return result;
}

void test_sum__loop(double N)(ref Bencher bencher){
  double[] arr = N.iota.array;
  bencher.iter((){
      return sum_loop(arr);
  });
}

void test_sum__func(double N)(ref Bencher bencher){
  double[] arr = N.iota.array;
  bencher.iter((){
      return sum_func(arr);
  });
}

void test_sum__simd(double N)(ref Bencher bencher){
  double[] arr = N.iota.array;
  bencher.iter((){
      return sum_simd(arr);
  });
}

void test_sum_autov(double N)(ref Bencher bencher){
  double[] arr = N.iota.array;
  bencher.iter((){
      return sum_autov(arr);
  });
}

void main()
{
  JSONValue js_s;
  js_s.array = [];
  static foreach(double N; iota(10_000,100_001,10_000)) {
    js_s.array ~= BenchMain!(test_sum__loop!N, test_sum__func!N, test_sum_autov!N, test_sum__simd!N).toJSON;
  }
  write("data_small.json", js_s.toPrettyString(JSONOptions.specialFloatLiterals));

  JSONValue js_l;
  js_l.array = [];
  static foreach(double N; iota(10_000_000,100_000_001,10_000_000)) {
    js_l.array ~= BenchMain!(test_sum__loop!N, test_sum__func!N, test_sum_autov!N, test_sum__simd!N).toJSON;
  }
  write("data_large.json", js_l.toPrettyString(JSONOptions.specialFloatLiterals));
}
