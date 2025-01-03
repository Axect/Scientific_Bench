import std.stdio : writeln;
import std.conv : to;
import kaleidic.lubeck : mtimes;

import mir.random.variable;
import mir.random.algorithm;

void main(string[] args)
{
	int row = args[1].to!int;
	int col = args[2].to!int;
	auto n = randomSlice(uniformVar(-1.0, 1.0), row, col);
	auto m = randomSlice(uniformVar(-1.0, 1.0), col, row);

	auto result = mtimes(n, m);

	writeln(result[row / 2, col / 2]);
}
