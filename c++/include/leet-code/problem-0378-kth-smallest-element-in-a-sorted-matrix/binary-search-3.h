#ifndef LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BINARY_SEARCH_3_H
#define LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BINARY_SEARCH_3_H

#include <algorithm>
#include <vector>

namespace leet_code::problem_0378_kth_smallest_element_in_a_sorted_matrix::binary_search_3 {
using std::vector;

class Solution {
public:
    int kthSmallest(vector<vector<int>> &matrix, int k) {
        const auto n = matrix.size();
        auto start = matrix.front().front();
        auto end = matrix.back().back();

        while (start != end) {
            const auto middle = start + (end - start) / 2;
            auto rank = 0;
            auto prev = static_cast<ptrdiff_t>(n);

            for (const auto &row : matrix) {
                prev = row.rend() - std::find_if(row.rend() - prev, row.rend(), [=](int v) { return v <= middle; });

                if (prev == 0) {
                    break;
                }

                rank += static_cast<int>(prev);
            }

            if (rank < k) {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        return start;
    }
};
} // namespace leet_code::problem_0378_kth_smallest_element_in_a_sorted_matrix::binary_search_3

#endif // LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BINARY_SEARCH_3_H
