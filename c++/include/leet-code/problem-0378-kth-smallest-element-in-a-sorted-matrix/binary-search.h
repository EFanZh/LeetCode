#ifndef LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BINARY_SEARCH_H
#define LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BINARY_SEARCH_H

#include <algorithm>
#include <numeric>
#include <vector>

namespace leet_code::problem_0378_kth_smallest_element_in_a_sorted_matrix::binary_search {
using std::less_equal;
using std::plus;
using std::vector;

class Solution {
public:
    int kthSmallest(vector<vector<int>> &matrix, int k) {
        auto start = matrix.front().front();
        auto end = matrix.back().back();

        while (start != end) {
            const auto middle = start + (end - start) / 2;

            const auto rank = std::transform_reduce(matrix.cbegin(), matrix.cend(), 0, plus{}, [=](const auto &row) {
                return static_cast<int>(std::lower_bound(row.begin(), row.end(), middle, less_equal{}) - row.begin());
            });

            if (rank < k) {
                start = middle + 1;
            } else {
                end = middle;
            }
        }

        return start;
    }
};
} // namespace leet_code::problem_0378_kth_smallest_element_in_a_sorted_matrix::binary_search

#endif // LEET_CODE_PROBLEM_0378_KTH_SMALLEST_ELEMENT_IN_A_SORTED_MATRIX_BINARY_SEARCH_H
