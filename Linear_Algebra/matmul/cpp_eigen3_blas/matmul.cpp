#define EIGEN_USE_BLAS

#include <cstdlib>
#include <iostream>
#include <Eigen/Dense>

using namespace Eigen;
using namespace std;

int main(int argc, const char* argv[]) {
    const int ROW = atoi(argv[1]);
    const int COL = atoi(argv[2]);
    auto c = MatrixXd::Constant(ROW, COL, 1.);
    auto m = (MatrixXd::Random(ROW, COL) + c) / 2;
    auto n = (MatrixXd::Random(ROW, COL) + c) / 2;

    auto result = m * n;
    cout << result(ROW/2, COL/2) << endl;
}
