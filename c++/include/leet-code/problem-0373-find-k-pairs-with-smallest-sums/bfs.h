#ifndef LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_BFS_H
#define LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_BFS_H

#include <queue>
#include <tuple>
#include <vector>

namespace leet_code::problem_0373_find_k_pairs_with_smallest_sums::bfs {
using std::initializer_list;
using std::priority_queue;
using std::size_t;
using std::tuple;
using std::vector;

class Solution {
public:
    vector<vector<int>> kSmallestPairs(vector<int> &nums1, vector<int> &nums2, int k) {
        const auto k_2 = static_cast<size_t>(k);
        auto result = vector<vector<int>>{};

        result.reserve(k_2);

        if (k_2 != 0 && !nums1.empty() && !nums2.empty()) {
            result.emplace_back(initializer_list<int>{nums1.front(), nums2.front()});

            auto node = tuple{size_t{0}, size_t{0}};

            auto queue = priority_queue{[&](const tuple<size_t, size_t> &lhs, const tuple<size_t, size_t> &rhs) {
                                            return nums1[std::get<0>(rhs)] + nums2[std::get<1>(rhs)] <
                                                   nums1[std::get<0>(lhs)] + nums2[std::get<1>(lhs)];
                                        },
                                        vector<tuple<size_t, size_t>>{}};

            while (result.size() != k_2) {
                const auto [i, j] = node;

                if (j != nums2.size() - 1) {
                    queue.emplace(i, j + 1);
                }

                if (j == 0 && i != nums1.size() - 1) {
                    queue.emplace(i + 1, j);
                }

                if (queue.empty()) {
                    break;
                }

                node = queue.top();
                queue.pop();
                result.emplace_back(initializer_list<int>{nums1[std::get<0>(node)], nums2[std::get<1>(node)]});
            }
        }

        return result;
    }
};
} // namespace leet_code::problem_0373_find_k_pairs_with_smallest_sums::bfs

#endif // LEET_CODE_PROBLEM_0373_FIND_K_PAIRS_WITH_SMALLEST_SUMS_BFS_H
