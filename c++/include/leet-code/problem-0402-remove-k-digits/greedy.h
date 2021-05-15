#ifndef LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_H
#define LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_H

#include <algorithm>
#include <string>

namespace leet_code::problem_0402_remove_k_digits::greedy {
using std::ptrdiff_t;
using std::string;

class Solution {
public:
    string removeKdigits(string num, int k) {
        if (k != 0) {
            const auto n = num.length();
            const auto k_2 = static_cast<size_t>(k);

            if (k_2 == n) {
                num = "0";
            } else {
                const auto stack_base = num.begin();
                const auto stack_top_end = stack_base + static_cast<ptrdiff_t>(n - k_2);

                auto stack_top =
                    std::find_if(stack_base, stack_top_end, [](const char &lhs) { return lhs > (&lhs)[1]; });

                auto it = stack_top + 1;

                while (static_cast<size_t>(it - stack_top) != k_2) {
                    const auto start =
                        it < stack_base + static_cast<ptrdiff_t>(k_2) ? stack_base : it - static_cast<ptrdiff_t>(k_2);

                    const auto digit = *it;
                    const auto insertion_point = std::upper_bound(start, stack_top, digit);

                    if (insertion_point != stack_top_end) {
                        *insertion_point = digit;
                        stack_top = insertion_point + 1;
                    }

                    ++it;
                }

                num.erase(stack_top, it);

                const auto it_2 = std::find_if(num.begin(), num.end(), [](char d) { return d != '0'; });

                if (it_2 == num.end()) {
                    num.resize(1);
                } else {
                    num.erase(num.begin(), it_2);
                }
            }
        }

        return num;
    }
};
} // namespace leet_code::problem_0402_remove_k_digits::greedy

#endif // LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_H
