#include <gtest/gtest.h>

namespace leet_code::problem_0321_create_maximum_number::tests {
template <class S>
void run_tests() {
    using std::tuple;
    using std::vector;

    const tuple<tuple<vector<int>, vector<int>, int>, vector<int>> test_cases[] = {
        {
            {{3, 4, 6, 5}, {9, 1, 2, 5, 8, 3}, 5},
            {9, 8, 6, 5, 3},
        },
        {{{6, 7}, {6, 0, 4}, 5}, {6, 7, 6, 0, 4}},
        {{{3, 9}, {8, 9}, 3}, {9, 8, 9}},
        {
            {{2, 5, 6, 4, 4, 0}, {7, 3, 8, 0, 6, 5, 7, 6, 2}, 15},
            {7, 3, 8, 2, 5, 6, 4, 4, 0, 6, 5, 7, 6, 2, 0},
        },
    };

    for (const auto &[args, expected] : test_cases) {
        auto [nums1, nums2, k] = args;
        const auto result = S{}.maxNumber(nums1, nums2, k);

        ASSERT_EQ(expected, result);
    }
}
} // namespace leet_code::problem_0321_create_maximum_number::tests
