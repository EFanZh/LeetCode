#ifndef LEET_CODE_DATA_STRUCTURES_LIST_NODE_H
#define LEET_CODE_DATA_STRUCTURES_LIST_NODE_H

namespace leet_code::data_structures {
struct ListNode {
    int val;        // NOLINT(misc-non-private-member-variables-in-classes)
    ListNode *next; // NOLINT(misc-non-private-member-variables-in-classes)

    explicit ListNode(int x) : val(x), next(nullptr) {
    }
};
} // namespace leet_code::data_structures

#endif // LEET_CODE_DATA_STRUCTURES_LIST_NODE_H
