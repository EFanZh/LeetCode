#ifndef LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_TESTS_H
#define LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_TESTS_H

#include <gtest/gtest.h>

namespace leet_code::problem_0406_queue_reconstruction_by_height::tests {
template <class S>
void run() {
    using std::initializer_list;
    using std::tuple;
    using std::vector;

    const auto test_cases = initializer_list<tuple<vector<vector<int>>, vector<vector<int>>>>{
        {
            {{7, 0}, {4, 4}, {7, 1}, {5, 0}, {6, 1}, {5, 2}},
            {{5, 0}, {7, 0}, {5, 2}, {6, 1}, {4, 4}, {7, 1}},
        },
        {
            {{6, 0}, {5, 0}, {4, 0}, {3, 2}, {2, 2}, {1, 4}},
            {{4, 0}, {5, 0}, {2, 2}, {3, 2}, {1, 4}, {6, 0}},
        },
        {{}, {}},
        {{{2, 0}}, {{2, 0}}},
    };

    for (const auto &[arg, expected] : test_cases) {
        auto people = arg;
        auto result = S{}.reconstructQueue(people);

        ASSERT_EQ(expected, result);
    }
}
} // namespace leet_code::problem_0406_queue_reconstruction_by_height::tests

#endif // LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_TESTS_H
