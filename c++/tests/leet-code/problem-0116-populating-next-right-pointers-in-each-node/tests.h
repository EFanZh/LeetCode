#ifndef LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_TESTS_H
#define LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_TESTS_H

#include "../test-utilities.h"
#include <algorithm>
#include <gtest/gtest.h>
#include <leet-code/data-structures/tree-node-with-next-right-pointer.h>
#include <optional>

namespace leet_code::problem_0116_populating_next_right_pointers_in_each_node::tests {
template <class S>
void run() {
    using data_structures::tree_node_with_next_right_pointer::Node;
    using std::initializer_list;
    using std::optional;
    using std::tuple;
    using std::vector;

    const auto test_cases = initializer_list<tuple<vector<optional<int>>, vector<optional<int>>>>{
        {{1, 2, 3, 4, 5, 6, 7}, {1, std::nullopt, 2, 3, std::nullopt, 4, 5, 6, 7, std::nullopt}},
    };

    for (const auto &[tree, expected] : test_cases) {
        auto [buffer, root] = test_utilities::make_tree<Node>(tree);

        ASSERT_EQ(expected, test_utilities::list_tree_levels_with_next_right_pointers(S{}.connect(root)));
    }
}
} // namespace leet_code::problem_0116_populating_next_right_pointers_in_each_node::tests

#endif // LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_TESTS_H
