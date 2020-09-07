#include <iostream>
#include <Eigen/Dense>

using namespace Eigen;
using namespace std;

const int ROW = 100;
const int COL = 100;

int main() {
    auto c = MatrixXd::Constant(ROW, COL, 1.);
    auto m = (MatrixXd::Random(ROW, COL) + c) / 2;
    auto n = (MatrixXd::Random(ROW, COL) + c) / 2;

    auto result = m * n;
    cout << result(ROW/2, COL/2) << endl;
}
