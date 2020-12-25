#ifndef LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_TESTS_H
#define LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_TESTS_H

#include <gtest/gtest.h>

namespace leet_code::problem_0378_kth_smallest_element_in_a_sorted_matrix::tests {
template <class S>
void run() {
    using std::initializer_list;
    using std::tuple;
    using std::vector;

    const auto test_cases = initializer_list<tuple<tuple<vector<vector<int>>, int>, int>>{
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 1}, 1},
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 2}, 5},
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 3}, 9},
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 4}, 10},
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 5}, 11},
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 6}, 12},
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 7}, 13},
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 8}, 13},
        {{{{1, 5, 9}, {10, 11, 13}, {12, 13, 15}}, 9}, 15},
    };

    for (const auto &[args, expected] : test_cases) {
        auto [matrix, k] = args;
        const auto result = S{}.kthSmallest(matrix, k);

        ASSERT_EQ(expected, result);
    }
}
} // namespace leet_code::problem_0378_kth_smallest_element_in_a_sorted_matrix::tests

#endif // LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_TESTS_H
