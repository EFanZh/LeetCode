#ifndef LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_FENWICK_TREE_H
#define LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_FENWICK_TREE_H

#include <algorithm>
#include <tuple>
#include <vector>

namespace leet_code::problem_0406_queue_reconstruction_by_height::fenwick_tree {
using std::vector;

class Solution {
    static size_t prefix_sum(const vector<size_t> &tree, size_t length) {
        auto result = size_t{0};

        while (length - 1 < tree.size()) {
            result += tree[length - 1];
            length &= length - 1;
        }

        return result;
    }

    static void update(vector<size_t> &tree, size_t index) {
        do {
            tree[index] += 1;
            index |= index + 1;
        } while (index < tree.size());
    }

    static size_t find_nth_free_slot(const vector<size_t> &tree, size_t n) {
        auto left = size_t{0};
        auto right = tree.size() - 1;

        while (left != right) {
            const auto middle = left + (right - left) / 2;

            if (middle < n + Solution::prefix_sum(tree, middle + 1)) {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        return left;
    }

public:
    vector<vector<int>> reconstructQueue(vector<vector<int>> &people) {
        std::sort(people.begin(), people.end(), [](const auto &lhs, const auto &rhs) {
            return lhs[0] < rhs[0] || (lhs[0] == rhs[0] && lhs[1] > rhs[1]);
        });

        auto result = vector<vector<int>>{people.size()};
        auto fenwick_tree = vector<size_t>(people.size());

        for (auto &p : people) {
            const auto index = Solution::find_nth_free_slot(fenwick_tree, static_cast<size_t>(p[1]));

            result[index] = std::move(p);
            Solution::update(fenwick_tree, index);
        }

        return result;
    }
};
} // namespace leet_code::problem_0406_queue_reconstruction_by_height::fenwick_tree

#endif // LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_FENWICK_TREE_H
