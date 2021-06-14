#ifndef LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_TESTS_H
#define LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_TESTS_H

#include <algorithm>
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
        {{{2, 7, 11, 19, 23}, {3, 5, 13, 17, 29}, 25},
         {{2, 3},  {2, 5},   {7, 3},   {7, 5},   {11, 3},  {2, 13},  {11, 5}, {2, 17}, {7, 13},
          {19, 3}, {7, 17},  {11, 13}, {19, 5},  {23, 3},  {11, 17}, {23, 5}, {2, 29}, {19, 13},
          {7, 29}, {19, 17}, {23, 13}, {11, 29}, {23, 17}, {19, 29}, {23, 29}}},
    };

    for (const auto &[args, expected] : test_cases) {
        auto [nums1, nums2, k] = args;
        auto result = S{}.kSmallestPairs(nums1, nums2, k);

        std::sort(result.begin(), result.end(), [](const auto &lhs, const auto &rhs) {
            return tuple{lhs[0] + lhs[1], lhs[0], lhs[1]} < tuple{rhs[0] + rhs[1], rhs[0], rhs[1]};
        });

        ASSERT_EQ(expected, result);
    }
}
} // namespace leet_code::problem_0373_find_k_pairs_with_smallest_sums::tests

#endif // LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_TESTS_H
