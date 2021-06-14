#ifndef LEET_CODE_PROBLEM_0117_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_II_TESTS_H
#define LEET_CODE_PROBLEM_0117_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_II_TESTS_H

#include "../test-utilities.h"
#include <algorithm>
#include <gtest/gtest.h>
#include <leet-code/data-structures/tree-node-with-next-right-pointer.h>
#include <optional>

namespace leet_code::problem_0117_populating_next_right_pointers_in_each_node_ii::tests {
template <class S>
void run() {
    using data_structures::tree_node_with_next_right_pointer::Node;
    using std::deque;
    using std::initializer_list;
    using std::optional;
    using std::tuple;
    using std::unordered_map;
    using std::vector;

    const auto test_cases = initializer_list<tuple<vector<optional<int>>, vector<optional<int>>>>{
        {{1, 2, 3, 4, 5, std::nullopt, 7}, {1, std::nullopt, 2, 3, std::nullopt, 4, 5, 7, std::nullopt}},
        {{1, std::nullopt, 2}, {1, std::nullopt, 2, std::nullopt}},
        {{1, 2}, {1, std::nullopt, 2, std::nullopt}},
        {{0, 2, 4, 1, std::nullopt, 3, -1, 5, 1, std::nullopt, 6, std::nullopt, 8},
         {0, std::nullopt, 2, 4, std::nullopt, 1, 3, -1, std::nullopt, 5, 1, 6, 8, std::nullopt}},
    };

    for (const auto &[tree, expected] : test_cases) {
        auto [buffer, root] = test_utilities::make_tree<Node>(tree);

        ASSERT_EQ(expected, test_utilities::list_tree_levels_with_next_right_pointers(S{}.connect(root)));
    }
}
} // namespace leet_code::problem_0117_populating_next_right_pointers_in_each_node_ii::tests

#endif // LEET_CODE_PROBLEM_0117_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_II_TESTS_H
