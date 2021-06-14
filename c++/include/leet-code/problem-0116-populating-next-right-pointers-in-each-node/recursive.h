#ifndef LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_RECURSIVE_H
#define LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_RECURSIVE_H

#include <leet-code/data-structures/tree-node-with-next-right-pointer.h>

namespace leet_code::problem_0116_populating_next_right_pointers_in_each_node::recursive {
using data_structures::tree_node_with_next_right_pointer::Node;

// ------------------------------------------------------ snip ------------------------------------------------------ //

class Solution {
    static void connect_between(Node *left, Node *right) {
        left->next = right;

        if (left->left != nullptr) {
            Solution::connect_between(left->right, right->left);
        }
    }

    static void connect_self(Node *node) {
        if (node->left != nullptr) {
            Solution::connect_between(node->left, node->right);
            Solution::connect_self(node->left);
            Solution::connect_self(node->right);
        }
    }

public:
    Node *connect(Node *root) {
        if (root != nullptr) {
            Solution::connect_self(root);
        }

        return root;
    }
};

// ------------------------------------------------------ snip ------------------------------------------------------ //

} // namespace leet_code::problem_0116_populating_next_right_pointers_in_each_node::recursive

#endif // LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_RECURSIVE_H
