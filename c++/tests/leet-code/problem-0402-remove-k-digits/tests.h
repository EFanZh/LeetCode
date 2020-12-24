#ifndef LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_TESTS_H
#define LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_TESTS_H

#include <gtest/gtest.h>

namespace leet_code::problem_0402_remove_k_digits::tests {
template <class S>
void run() {
    using std::initializer_list;
    using std::string;
    using std::string_view;
    using std::tuple;

    const auto test_cases = initializer_list<tuple<tuple<string_view, int>, string_view>>{
        {{"1432219", 3}, "1219"},
        {{"10200", 1}, "200"},
        {{"10", 2}, "0"},
        {{"112", 1}, "11"},
        {{"10", 1}, "0"},
        {{"1173", 2}, "11"},
        {{"5337", 2}, "33"},
        {{"1234", 4}, "0"},
        {{"52660469", 2}, "260469"},
        {{"1234567890", 9}, "0"},
    };

    for (const auto &[args, expected] : test_cases) {
        const auto [num, k] = args;
        const auto result = S{}.removeKdigits(string{num}, k);

        ASSERT_EQ(expected, result);
    }
}
} // namespace leet_code::problem_0402_remove_k_digits::tests

#endif // LEET_CODE_PROBLEM_0402_REMOVE_K_DIGITS_TESTS_H
