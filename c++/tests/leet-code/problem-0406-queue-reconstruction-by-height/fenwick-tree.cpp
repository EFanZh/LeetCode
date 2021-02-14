#include "tests.h"
#include <leet-code/problem-0406-queue-reconstruction-by-height/fenwick-tree.h>

namespace leet_code::problem_0406_queue_reconstruction_by_height::tests {
TEST(Problem0406QueueReconstructionByHeight, FenwickTree) {
    tests::run<fenwick_tree::Solution>();
}
} // namespace leet_code::problem_0406_queue_reconstruction_by_height::tests
