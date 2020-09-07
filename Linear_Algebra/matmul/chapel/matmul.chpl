use LinearAlgebra;
use Random;

config const r = 100;
config const c = 100;

var A: [1..r, 1..c] real;
var B: [1..r, 1..c] real;
fillRandom(A);
fillRandom(B);

var C = A.dot(B);
writeln(C[r/2+1,c/2+1]);
