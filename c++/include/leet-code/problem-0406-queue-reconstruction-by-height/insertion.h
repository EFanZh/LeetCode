#ifndef LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_INSERTION_H
#define LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_INSERTION_H

#include <algorithm>
#include <tuple>
#include <vector>

namespace leet_code::problem_0406_queue_reconstruction_by_height::insertion {
using std::ptrdiff_t;
using std::vector;

class Solution {
public:
    vector<vector<int>> reconstructQueue(vector<vector<int>> &people) {
        std::sort(people.begin(), people.end(), [](const auto &lhs, const auto &rhs) {
            return lhs[0] > rhs[0] || (lhs[0] == rhs[0] && lhs[1] < rhs[1]);
        });

        for (auto it = people.begin(); it != people.end(); ++it) {
            std::rotate(people.begin() + static_cast<ptrdiff_t>((*it)[1]), it, it + 1);
        }

        return std::move(people);
    }
};
} // namespace leet_code::problem_0406_queue_reconstruction_by_height::insertion

#endif // LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_INSERTION_H
