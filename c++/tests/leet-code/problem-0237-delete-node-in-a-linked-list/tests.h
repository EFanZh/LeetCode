#ifndef LEET_CODE_PROBLEM_0237_DELETE_NODE_IN_A_LINKED_LIST_TESTS_H
#define LEET_CODE_PROBLEM_0237_DELETE_NODE_IN_A_LINKED_LIST_TESTS_H

#include "../test-utilities.h"
#include <gtest/gtest.h>

namespace leet_code::problem_0237_delete_node_in_a_linked_list::tests {
template <class S>
void run_tests() {
    using std::tuple;
    using std::vector;

    const tuple<tuple<vector<int>, int>, vector<int>> test_cases[] = {
        {{{4, 5, 1, 9}, 5}, {4, 1, 9}},
        {{{4, 5, 1, 9}, 1}, {4, 5, 9}},
        {{{1, 2, 3, 4}, 3}, {1, 2, 4}},
        {{{0, 1}, 0}, {1}},
        {{{-3, 5, -99}, -3}, {5, -99}},
    };

    for (const auto &[args, expected] : test_cases) {
        const auto [list_values, node_value] = args;
        const auto [buffer, head] = test_utilities::make_list(list_values);
        auto *node = head;

        while (node->val != node_value) {
            node = node->next;
        }

        S{}.deleteNode(node);

        ASSERT_EQ(expected, test_utilities::list_to_vector(head));
    }
}
} // namespace leet_code::problem_0237_delete_node_in_a_linked_list::tests

#endif // LEET_CODE_PROBLEM_0237_DELETE_NODE_IN_A_LINKED_LIST_TESTS_H
