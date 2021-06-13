#ifndef LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_SORT_AND_MERGE_H
#define LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_SORT_AND_MERGE_H

#include <algorithm>
#include <vector>

namespace leet_code::problem_0436_find_right_interval::sort_and_merge {
using std::greater;
using std::tuple;
using std::vector;

// ------------------------------------------------------ snip ------------------------------------------------------ //

class Solution {
public:
    vector<int> findRightInterval(vector<vector<int>> &intervals) {
        const auto n = intervals.size();
        auto lefts = vector<tuple<int, int>>(n);
        auto rights = vector<tuple<int, int>>(n);

        for (auto i = size_t{0}; i != n; ++i) {
            const auto &interval = intervals[i];

            lefts[i] = {interval[0], static_cast<int>(i)};
            rights[i] = {interval[1], static_cast<int>(i)};
        }

        std::sort(lefts.begin(), lefts.end(), greater{});
        std::sort(rights.begin(), rights.end(), greater{});

        auto result = vector<int>(n);
        auto prev = -1;
        auto left_iter = lefts.cbegin();
        const auto left_end = lefts.cend();
        auto right_iter = rights.cbegin();
        const auto right_end = rights.cend();

        if (left_iter != left_end) {
            for (;;) {
                if (std::get<0>(*left_iter) < std::get<0>(*right_iter)) {
                    result[static_cast<size_t>(std::get<1>(*right_iter))] = prev;

                    ++right_iter;

                    if (right_iter == right_end) {
                        break;
                    }
                } else {
                    prev = std::get<1>(*left_iter);

                    ++left_iter;

                    if (left_iter == left_end) {
                        break;
                    }
                }
            }
        }

        return result;
    }
};

// ------------------------------------------------------ snip ------------------------------------------------------ //

} // namespace leet_code::problem_0436_find_right_interval::sort_and_merge

#endif // LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_SORT_AND_MERGE_H
