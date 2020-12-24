#ifndef LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_TESTS_H
#define LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_TESTS_H

#include <gtest/gtest.h>

namespace leet_code::problem_0373_find_k_pairs_with_smallest_sums::tests {
template <class S>
void run() {
    using std::initializer_list;
    using std::tuple;
    using std::vector;

    const auto test_cases = initializer_list<tuple<tuple<vector<int>, vector<int>, int>, vector<vector<int>>>>{
        {{{1, 7, 11}, {2, 4, 6}, 3}, {{1, 2}, {1, 4}, {1, 6}}},
        {{{1, 1, 2}, {1, 2, 3}, 2}, {{1, 1}, {1, 1}}},
        {{{1, 2}, {3}, 3}, {{1, 3}, {2, 3}}},
        {{{1, 7, 11}, {2, 4, 6}, 9}, {{1, 2}, {1, 4}, {1, 6}, {7, 2}, {7, 4}, {7, 6}, {11, 2}, {11, 4}, {11, 6}}},
    };

    for (const auto &[args, expected] : test_cases) {
        auto [nums1, nums2, k] = args;
        auto result = S{}.kSmallestPairs(nums1, nums2, k);

        std::sort(result.begin(), result.end());

        ASSERT_EQ(expected, result);
    }
}
} // namespace leet_code::problem_0373_find_k_pairs_with_smallest_sums::tests

#endif // LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_TESTS_H
