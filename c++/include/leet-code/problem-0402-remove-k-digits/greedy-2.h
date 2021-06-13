#ifndef LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_2_H
#define LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_2_H

#include <algorithm>
#include <string>

namespace leet_code::problem_0402_remove_k_digits::greedy_2 {
using std::ptrdiff_t;
using std::string;

// ------------------------------------------------------ snip ------------------------------------------------------ //

class Solution {
public:
    string removeKdigits(string num, int k) {
        if (k != 0) {
            const auto n = num.length();
            const auto k_2 = static_cast<size_t>(k);

            if (k_2 == n) {
                num = "0";
            } else {
                const auto stack_base_end = num.begin() + static_cast<ptrdiff_t>(k_2);

                const auto stack_base =
                    std::find_if(num.begin(), stack_base_end, [](const char &lhs) { return lhs <= (&lhs)[1]; });

                if (stack_base == stack_base_end) {
                    const auto it = std::find_if(stack_base_end, num.end(), [](char d) { return d != '0'; });

                    if (it == num.end()) {
                        num = "0";
                    } else {
                        num.erase(num.begin(), it);
                    }
                } else {
                    const auto stack_top_end = stack_base + static_cast<ptrdiff_t>(n - k_2);

                    auto stack_top =
                        std::find_if(stack_base + 1, stack_top_end, [](const char &lhs) { return lhs > (&lhs)[1]; });

                    auto it = stack_top + 1;
                    const auto to_remove = k_2 - static_cast<size_t>(stack_base - num.begin());

                    while (it != num.end()) {
                        const auto digit = *it;

                        const auto start = it < stack_base + static_cast<ptrdiff_t>(to_remove)
                                               ? stack_base
                                               : it - static_cast<ptrdiff_t>(to_remove);

                        const auto insertion_point = std::upper_bound(start, stack_top, digit);

                        if (static_cast<size_t>(it - insertion_point) == to_remove) {
                            stack_top = insertion_point;

                            break;
                        }

                        if (insertion_point != stack_top_end) {
                            *insertion_point = digit;
                            stack_top = insertion_point + 1;
                        }

                        ++it;
                    }

                    std::copy(stack_base, stack_top, num.begin());
                    num.erase(it - static_cast<ptrdiff_t>(k_2), it);

                    const auto it_2 = std::find_if(num.begin(), num.end(), [](char d) { return d != '0'; });

                    if (it_2 == num.end()) {
                        num.resize(1);
                    } else {
                        num.erase(num.begin(), it_2);
                    }
                }
            }
        }

        return num;
    }
};

// ------------------------------------------------------ snip ------------------------------------------------------ //

} // namespace leet_code::problem_0402_remove_k_digits::greedy_2

#endif // LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_GREEDY_2_H
