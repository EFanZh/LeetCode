#ifndef LEET_CODE_PROBLEM_0321_CREATE_MAXIMUM_NUMBER_DYNAMIC_PROGRAMMING_H
#define LEET_CODE_PROBLEM_0321_CREATE_MAXIMUM_NUMBER_DYNAMIC_PROGRAMMING_H

#include <algorithm>
#include <tuple>
#include <vector>

namespace leet_code::problem_0321_create_maximum_number::dynamic_programming {
using std::greater_equal;
using std::tuple;
using std::vector;

class Solution {
    static vector<vector<int>> single_max_numbers(int *nums, size_t nums_length, size_t min_length, size_t max_length) {
        auto *const nums_end = nums + nums_length;
        const auto expected_items = max_length - min_length + 1;
        auto result = vector<vector<int>>{};

        result.reserve(expected_items);

        if (max_length == 0) {
            result.emplace_back();
        } else {
            const auto min_remove = nums_length - max_length;
            auto *const stack_base_end = nums + min_remove;
            auto *stack_base = std::find_if(nums, stack_base_end, [](const int &v) { return v >= (&v)[1]; });
            int *stack_top = nullptr;
            int *p = nullptr;

            if (stack_base == stack_base_end) {
                result.emplace_back(stack_base, nums_end);

                stack_top = stack_base + 1;
                p = stack_base + 1;
            } else {
                const auto remaining_to_remove = min_remove - static_cast<size_t>(stack_base - nums);
                auto *const stack_top_end = stack_base + max_length;

                stack_top = std::find_if(stack_base + 1, stack_top_end, [](const int &v) { return v < (&v)[1]; });
                p = stack_top + 1;

                for (;;) {
                    const auto num = *p;
                    const auto start = std::max(stack_base, p - remaining_to_remove);
                    const auto insertion_point = std::lower_bound(start, stack_top, num, greater_equal{});

                    if (static_cast<size_t>(p - insertion_point) == remaining_to_remove) {
                        stack_top = insertion_point;

                        auto &item = result.emplace_back();

                        item.reserve(max_length);
                        item.insert(item.end(), stack_base, stack_top);
                        item.insert(item.end(), p, nums_end);

                        break;
                    } else {
                        if (insertion_point != stack_base + max_length) {
                            *insertion_point = num;
                            stack_top = insertion_point + 1;
                        }

                        ++p;

                        if (p == nums_end) {
                            result.emplace_back(stack_base, stack_top);

                            break;
                        }
                    }
                }
            }

            if (expected_items != 1) {
                for (;;) {
                    if (p == nums_end) {
                        for (auto *top = stack_top - 1, *top_end = stack_base + min_length - 1; top != top_end; --top) {
                            result.emplace_back(stack_base, top);
                        }

                        break;
                    } else {
                        const auto num = *p;

                        if (num > stack_top[-1]) {
                            if (stack_top - stack_base == 1) {
                                result.emplace_back(p, nums_end);

                                if (result.size() == expected_items) {
                                    break;
                                } else {
                                    stack_base = p;
                                    stack_top = p + 1;
                                    ++p;
                                }
                            } else {
                                --stack_top;

                                const auto reserve_size = max_length - result.size();
                                auto &item = result.emplace_back();

                                item.reserve(reserve_size);
                                item.insert(item.end(), stack_base, stack_top);
                                item.insert(item.end(), p, nums_end);

                                if (result.size() == expected_items) {
                                    break;
                                }
                            }
                        } else {
                            *stack_top = num;
                            ++stack_top;
                            ++p;
                        }
                    }
                }
            }
        }

        return result;
    }

    static void merge(const int *nums_1, const int *nums_1_end, const int *nums_2, const int *nums_2_end, int *result) {
        if (nums_1 == nums_1_end) {
            std::copy(nums_2, nums_2_end, result);
        } else if (nums_2 == nums_2_end) {
            std::copy(nums_1, nums_1_end, result);
        } else {
            for (;;) {
                if (*nums_1 < *nums_2 || (*nums_1 == *nums_2 && std::lexicographical_compare(
                                                                    nums_1 + 1, nums_1_end, nums_2 + 1, nums_2_end))) {
                    *result = *nums_2;
                    ++result;
                    ++nums_2;

                    if (nums_2 == nums_2_end) {
                        std::copy(nums_1, nums_1_end, result);

                        break;
                    }
                } else {
                    *result = *nums_1;
                    ++result;
                    ++nums_1;

                    if (nums_1 == nums_1_end) {
                        std::copy(nums_2, nums_2_end, result);

                        break;
                    }
                }
            }
        }
    }

public:
    vector<int> maxNumber(vector<int> &nums1, vector<int> &nums2, int k) {
        const auto k_2 = static_cast<size_t>(k);

        if (nums2.size() < nums1.size()) {
            nums1.swap(nums2);
        }

        const auto [range_0, range_1] = k_2 < nums1.size()
                                            ? tuple{size_t{0}, k_2}
                                            : tuple{k_2 < nums2.size() ? size_t{0} : k_2 - nums2.size(), nums1.size()};

        auto result = vector<int>(k_2);
        const auto max_nums_1 = Solution::single_max_numbers(nums1.data(), nums1.size(), range_0, range_1);
        const auto max_nums_2 = Solution::single_max_numbers(nums2.data(), nums2.size(), k_2 - range_1, k_2 - range_0);
        auto merge_buffer = vector<int>(k_2);
        auto max_nums_2_iter = max_nums_2.rbegin();

        for (const auto &max_num_1 : max_nums_1) {
            const auto &max_num_2 = *max_nums_2_iter;

            Solution::merge(max_num_1.data(),
                            max_num_1.data() + max_num_1.size(),
                            max_num_2.data(),
                            max_num_2.data() + max_num_2.size(),
                            merge_buffer.data());

            if (merge_buffer > result) {
                result.swap(merge_buffer);
            }

            ++max_nums_2_iter;
        }

        return result;
    }
};
} // namespace leet_code::problem_0321_create_maximum_number::dynamic_programming

#endif // LEET_CODE_PROBLEM_0321_CREATE_MAXIMUM_NUMBER_DYNAMIC_PROGRAMMING_H
