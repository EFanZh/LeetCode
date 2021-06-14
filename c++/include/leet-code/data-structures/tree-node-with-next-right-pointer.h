#ifndef LEET_CODE_DATA_STRUCTURES_TREE_NODE_WITH_NEXT_POINTER_H
#define LEET_CODE_DATA_STRUCTURES_TREE_NODE_WITH_NEXT_POINTER_H

namespace leet_code::data_structures::tree_node_with_next_right_pointer {
struct Node {
    int val;     // NOLINT(misc-non-private-member-variables-in-classes)
    Node *left;  // NOLINT(misc-non-private-member-variables-in-classes)
    Node *right; // NOLINT(misc-non-private-member-variables-in-classes)
    Node *next;  // NOLINT(misc-non-private-member-variables-in-classes)

    explicit Node(int val) : val{val}, left{nullptr}, right{nullptr}, next{nullptr} {
    }
};
} // namespace leet_code::data_structures::tree_node_with_next_right_pointer

#endif // LEET_CODE_DATA_STRUCTURES_TREE_NODE_WITH_NEXT_POINTER_H
