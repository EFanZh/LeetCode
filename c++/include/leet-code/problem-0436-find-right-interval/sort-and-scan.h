#ifndef LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_SORT_AND_SCAN_H
#define LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_SORT_AND_SCAN_H

#include <algorithm>
#include <vector>

namespace leet_code::problem_0436_find_right_interval::sort_and_scan {
using std::tuple;
using std::vector;

// ------------------------------------------------------ snip ------------------------------------------------------ //

class Solution {
public:
    vector<int> findRightInterval(vector<vector<int>> &intervals) {
        const auto n = intervals.size();
        auto endpoints = vector<tuple<tuple<int, bool>, int>>(n * 2);

        for (auto i = size_t{0}; i != n; ++i) {
            const auto &interval = intervals[i];

            endpoints[i * 2] = {{interval[1], false}, static_cast<int>(i)};
            endpoints[i * 2 + 1] = {{interval[0], true}, static_cast<int>(i)};
        }

        std::sort(endpoints.begin(), endpoints.end(), [](const auto &lhs, const auto &rhs) {
            return std::get<0>(rhs) < std::get<0>(lhs);
        });

        auto result = vector<int>(n);
        auto right = -1;

        for (const auto &[endpoint, index] : endpoints) {
            if (std::get<1>(endpoint)) {
                right = index;
            } else {
                result[static_cast<size_t>(index)] = right;
            }
        }

        return result;
    }
};

// ------------------------------------------------------ snip ------------------------------------------------------ //

} // namespace leet_code::problem_0436_find_right_interval::sort_and_scan

#endif // LEET_CODE_PROBLEM_0436_FIND_RIGHT_INTERVAL_SORT_AND_SCAN_H
