#include "test-utilities.h"

using leet_code::data_structures::ListNode;
using std::vector;

namespace leet_code::test_utilities {
vector<int> list_to_vector(ListNode *head) {
    auto result = std::vector<int>{};

    while (head != nullptr) {
        result.emplace_back(head->val);
        head = head->next;
    }

    return result;
}
} // namespace leet_code::test_utilities
