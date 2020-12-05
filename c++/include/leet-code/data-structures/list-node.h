#ifndef LEET_CODE_DATA_STRUCTURES_LIST_NODE_H
#define LEET_CODE_DATA_STRUCTURES_LIST_NODE_H

namespace leet_code::data_structures {
struct ListNode {
    int val;
    ListNode *next;

    ListNode(int x) : val(x), next(nullptr) {
    }
};
} // namespace leet_code::data_structures

#endif // LEET_CODE_DATA_STRUCTURES_LIST_NODE_H
