#ifndef LEET_CODE_DATA_STRUCTURES_TREE_NODE_WITH_NEXT_POINTER_H
#define LEET_CODE_DATA_STRUCTURES_TREE_NODE_WITH_NEXT_POINTER_H

namespace leet_code::data_structures::tree_node_with_next_right_pointer {
struct Node {
    int val;
    Node *left = nullptr;
    Node *right = nullptr;
    Node *next = nullptr;

    explicit Node(int val) : val{val} {
    }
};
} // namespace leet_code::data_structures::tree_node_with_next_right_pointer

#endif // LEET_CODE_DATA_STRUCTURES_TREE_NODE_WITH_NEXT_POINTER_H
