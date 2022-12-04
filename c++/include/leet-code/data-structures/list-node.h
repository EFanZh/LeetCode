#ifndef LEET_CODE_DATA_STRUCTURES_LIST_NODE_H
#define LEET_CODE_DATA_STRUCTURES_LIST_NODE_H

namespace leet_code::data_structures::list_node {
struct ListNode {
    int val;
    ListNode *next = nullptr;

    explicit ListNode(int x) : val{x} {
    }
};
} // namespace leet_code::data_structures::list_node

#endif // LEET_CODE_DATA_STRUCTURES_LIST_NODE_H
