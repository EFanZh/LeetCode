#ifndef LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_ITERATIVE_H
#define LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_ITERATIVE_H

#include <leet-code/data-structures/tree-node-with-next-right-pointer.h>

namespace leet_code::problem_0116_populating_next_right_pointers_in_each_node::iterative {
using data_structures::tree_node_with_next_right_pointer::Node;

// ------------------------------------------------------ snip ------------------------------------------------------ //

class Solution {
public:
    Node *connect(Node *root) {
        if (root != nullptr) {
            auto *row = root;

            while (row->left != nullptr) {
                for (auto *node = row;; node = node->next) {
                    node->left->next = node->right;

                    if (node->next == nullptr) {
                        break;
                    }

                    node->right->next = node->next->left;
                }

                row = row->left;
            }
        }

        return root;
    }
};

// ------------------------------------------------------ snip ------------------------------------------------------ //

} // namespace leet_code::problem_0116_populating_next_right_pointers_in_each_node::iterative

#endif // LEET_CODE_PROBLEM_0116_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_ITERATIVE_H
