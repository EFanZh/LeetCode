#ifndef LEET_CODE_PROBLEM_0386_LEXICOGRAPHICAL_NUMBERS_ITERATIVE_DFS_H
#define LEET_CODE_PROBLEM_0386_LEXICOGRAPHICAL_NUMBERS_ITERATIVE_DFS_H

#include <vector>

namespace leet_code::problem_0386_lexicographical_numbers::iterative_dfs {
using std::size_t;
using std::vector;

// ------------------------------------------------------ snip ------------------------------------------------------ //

class Solution {
public:
    vector<int> lexicalOrder(int n) {
        auto result = vector<int>{};
        auto value = 1;

        result.reserve(static_cast<size_t>(n));

        for (;;) {
            result.emplace_back(value);

            const auto next = value * 10;

            if (next <= n) {
                value = next;
            } else {
                if (value == n) {
                    value /= 10;
                }

                while (value % 10 == 9) {
                    value /= 10;
                }

                if (value == 0) {
                    break;
                }

                ++value;
            }
        }

        return result;
    }
};

// ------------------------------------------------------ snip ------------------------------------------------------ //

} // namespace leet_code::problem_0386_lexicographical_numbers::iterative_dfs

#endif // LEET_CODE_PROBLEM_0386_LEXICOGRAPHICAL_NUMBERS_ITERATIVE_DFS_H
