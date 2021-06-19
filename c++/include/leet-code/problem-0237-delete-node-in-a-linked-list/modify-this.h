#ifndef LEET_CODE_PROBLEM_0237_DELETE_NODE_IN_A_LINKED_LIST_MODIFY_THIS_H
#define LEET_CODE_PROBLEM_0237_DELETE_NODE_IN_A_LINKED_LIST_MODIFY_THIS_H

#include <leet-code/data-structures/list-node.h>

namespace leet_code::problem_0237_delete_node_in_a_linked_list::modify_this {
using data_structures::list_node::ListNode;

// ------------------------------------------------------ snip ------------------------------------------------------ //

class Solution {
public:
    void deleteNode(ListNode *node) {
        *node = *node->next;
    }
};

// ------------------------------------------------------ snip ------------------------------------------------------ //

} // namespace leet_code::problem_0237_delete_node_in_a_linked_list::modify_this

#endif // LEET_CODE_PROBLEM_0237_DELETE_NODE_IN_A_LINKED_LIST_MODIFY_THIS_H
