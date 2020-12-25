#ifndef LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BFS_2_H
#define LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BFS_2_H

#include <algorithm>
#include <vector>

namespace leet_code::problem_0378_kth_smallest_element_in_a_sorted_matrix::bfs_2 {
using std::tuple;
using std::vector;

class Solution {
    static size_t saturating_sub(size_t lhs, size_t rhs) {
        return lhs <= rhs ? 0 : lhs - rhs;
    }

public:
    int kthSmallest(vector<vector<int>> &matrix, int k) {
        const auto n = matrix.size();
        const auto k_2 = static_cast<size_t>(k);
        const auto min_row = Solution::saturating_sub(n + k_2, n * n + 1);
        auto skipped = n * min_row;
        const auto max_row = std::min(k_2, n);
        auto queue = vector<tuple<size_t, size_t>>{};

        queue.reserve(max_row - min_row);

        for (auto i = min_row; i != max_row; ++i) {
            const auto j = Solution::saturating_sub(n, (n * n + 1 - k_2) / (n - i));

            skipped += j;
            queue.emplace_back(i, j);
        }

        const auto compare = [&](const auto &lhs, const auto &rhs) {
            return matrix[std::get<0>(rhs)][std::get<1>(rhs)] < matrix[std::get<0>(lhs)][std::get<1>(lhs)];
        };

        std::make_heap(queue.begin(), queue.end(), compare);

        for (auto i = size_t{0}, needed = k_2 - (skipped + 1); i != needed; ++i) {
            std::pop_heap(queue.begin(), queue.end(), compare);

            const auto node = queue.back();

            if (std::get<1>(node) == n - 1) {
                queue.pop_back();
            } else {
                queue.back() = {std::get<0>(node), std::get<1>(node) + 1};
                std::push_heap(queue.begin(), queue.end(), compare);
            }
        }

        return matrix[std::get<0>(queue.front())][std::get<1>(queue.front())];
    }
};
} // namespace leet_code::problem_0378_kth_smallest_element_in_a_sorted_matrix::bfs_2

#endif // LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BFS_2_H
