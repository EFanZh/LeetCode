#include <algorithm>
#include <tuple>
#include <vector>

namespace leet_code::problem_0321_create_maximum_number::greedy {
using std::greater_equal;
using std::tuple;
using std::vector;

class Solution {
    static void single_max_number(const int *num, size_t num_length, int *result, size_t result_length) {
        if (result_length != 0) {
            const auto k = num_length - result_length;
            auto stack_length = size_t{0};
            auto i = size_t{0};

            while (i - stack_length != k) {
                const auto start = i < k ? 0 : i - k;
                const auto digit = num[i];

                const auto insertion_point = static_cast<size_t>(
                    std::lower_bound(result + start, result + stack_length, digit, greater_equal<>{}) - result);

                if (insertion_point != result_length) {
                    result[insertion_point] = digit;
                    stack_length = insertion_point + 1;
                }

                i += 1;
            }

            std::copy(num + i, num + num_length, result + stack_length);
        }
    }

    static void merge(const int *nums_1, const int *nums_1_end, const int *nums_2, const int *nums_2_end, int *result) {
        if (nums_1 == nums_1_end) {
            std::copy(nums_2, nums_2_end, result);
        } else if (nums_2 == nums_2_end) {
            std::copy(nums_1, nums_1_end, result);
        } else {
            for (;;) {
                if (std::lexicographical_compare(nums_1, nums_1_end, nums_2, nums_2_end)) {
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

        auto range = tuple<size_t, size_t>{};

        if (k_2 < nums1.size()) {
            range = {size_t{0}, k_2 + 1};
        } else if (k_2 < nums2.size()) {
            range = {size_t{0}, nums1.size() + 1};
        } else {
            range = {k_2 - nums2.size(), nums1.size() + 1};
        }

        auto result = vector<int>(k_2);
        auto max_buffer = vector<int>(k_2);
        auto merge_buffer = vector<int>(k_2);

        for (auto i = std::get<0>(range); i != std::get<1>(range); ++i) {
            const auto max_1 = max_buffer.data();
            const auto max_1_length = i;
            const auto max_2 = max_buffer.data() + i;
            const auto max_2_length = k_2 - i;

            Solution::single_max_number(nums1.data(), nums1.size(), max_1, max_1_length);
            Solution::single_max_number(nums2.data(), nums2.size(), max_2, max_2_length);
            Solution::merge(max_1, max_1 + max_1_length, max_2, max_2 + max_2_length, merge_buffer.data());

            if (merge_buffer > result) {
                result.swap(merge_buffer);
            }
        }

        return result;
    }
};
} // namespace leet_code::problem_0321_create_maximum_number::greedy
