#ifndef LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_TESTS_H
#define LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_TESTS_H

#include <gtest/gtest.h>

namespace leet_code::problem_0436_find_right_interval::tests {
template <class S>
void run() {
    using std::initializer_list;
    using std::tuple;
    using std::vector;

    const auto test_cases = initializer_list<tuple<vector<vector<int>>, vector<int>>>{
        {{{1, 2}}, {-1}},
        {{{3, 4}, {2, 3}, {1, 2}}, {-1, 0, 1}},
        {{{1, 4}, {2, 3}, {3, 4}}, {-1, 2, -1}},
    };

    for (const auto &[test_case, expected] : test_cases) {
        auto intervals = test_case;
        auto result = S{}.findRightInterval(intervals);

        ASSERT_EQ(expected, result);
    }
}
} // namespace leet_code::problem_0436_find_right_interval::tests

#endif // LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_TESTS_H
