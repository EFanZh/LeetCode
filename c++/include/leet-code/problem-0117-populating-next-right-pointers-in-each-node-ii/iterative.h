#ifndef LEET_CODE_PROBLEM_0117_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_II_ITERATIVE_H
#define LEET_CODE_PROBLEM_0117_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_II_ITERATIVE_H

#include <leet-code/data-structures/tree-node-with-next-right-pointer.h>

namespace leet_code::problem_0117_populating_next_right_pointers_in_each_node_ii::iterative {
using data_structures::tree_node_with_next_right_pointer::Node;

// ------------------------------------------------------ snip ------------------------------------------------------ //

class Solution {
public:
    Node *connect(Node *root) {
        if (root != nullptr) {
            auto *row = root;

            for (;;) {
                auto *node = row;
                auto *prev = static_cast<Node *>(nullptr);

                // Find first non-null node in the next row.

                for (;; node = node->next) {
                    if (node->left == nullptr) {
                        if (node->right != nullptr) {
                            row = node->right;
                            prev = node->right;

                            break;
                        }
                    } else {
                        row = node->left;

                        if (node->right == nullptr) {
                            prev = node->left;
                        } else {
                            node->left->next = node->right;
                            prev = node->right;
                        }

                        break;
                    }

                    if (node->next == nullptr) {
                        break;
                    }
                }

                if (prev == nullptr) {
                    break;
                }

                // Link nodes in the next row.

                for (node = node->next; node != nullptr; node = node->next) {
                    if (node->left != nullptr) {
                        prev->next = node->left;
                        prev = node->left;
                    }

                    if (node->right != nullptr) {
                        prev->next = node->right;
                        prev = node->right;
                    }
                }
            }
        }

        return root;
    }
};

// ------------------------------------------------------ snip ------------------------------------------------------ //

} // namespace leet_code::problem_0117_populating_next_right_pointers_in_each_node_ii::iterative

#endif // LEET_CODE_PROBLEM_0117_POPULATING_NEXT_RIGHT_POINTERS_IN_EACH_NODE_II_ITERATIVE_H
