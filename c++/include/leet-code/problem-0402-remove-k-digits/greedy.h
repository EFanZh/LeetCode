#ifndef LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_H
#define LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_H

#include <algorithm>
#include <string>

namespace leet_code::problem_0402_remove_k_digits::greedy {
using std::greater;
using std::string;

class Solution {
public:
    string removeKdigits(string num, int k) {
        if (k != 0) {
            const auto k2 = static_cast<size_t>(k);
            const auto n = num.length();

            if (k2 < n) {
                const auto stack_begin = num.begin();
                const auto max_stack_end = stack_begin + static_cast<ptrdiff_t>(n - k2);

                auto stack_end =
                    std::find_if(stack_begin, max_stack_end, [](const char &lhs) { return lhs > (&lhs)[1]; });

                auto it = stack_end + 1;

                while (static_cast<size_t>(it - stack_end) != k2) {
                    const auto start =
                        static_cast<size_t>(it - stack_begin) < k2 ? stack_begin : it - static_cast<ptrdiff_t>(k2);

                    const auto digit = *it;
                    const auto insertion_point = std::upper_bound(start, stack_end, digit);

                    if (insertion_point != max_stack_end) {
                        *insertion_point = digit;
                        stack_end = insertion_point + 1;
                    }

                    ++it;
                }

                num.erase(stack_end, it);

                const auto leading_zeros = num.find_first_not_of('0');

                if (leading_zeros == string::npos) {
                    num.resize(1);
                } else {
                    num.erase(0, leading_zeros);
                }
            } else {
                num = "0";
            }
        }

        return num;
    }
};
} // namespace leet_code::problem_0402_remove_k_digits::greedy

#endif // LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_H
