#include "test-utilities.h"

using leet_code::data_structures::ListNode;
using std::deque;
using std::optional;
using std::vector;

namespace leet_code::test_utilities {
vector<int> list_to_vector(const ListNode *head) {
    auto result = vector<int>{};

    while (head != nullptr) {
        result.emplace_back(head->val);
        head = head->next;
    }

    return result;
}

vector<optional<int>> list_tree_levels_with_next_right_pointers(
    const data_structures::tree_node_with_next_right_pointer::Node *root) {
    using data_structures::tree_node_with_next_right_pointer::Node;

    auto result = vector<optional<int>>{};

    if (root != nullptr) {
        auto queue = deque<const Node *>{root};

        for (;;) {
            for (const auto *node = queue.front();; node = node->next) {
                result.emplace_back(node->val);

                if (node->next == nullptr) {
                    break;
                }
            }

            result.emplace_back(std::nullopt);

            for (auto i = size_t{0}, queue_size = queue.size(); i != queue_size; ++i) {
                const auto *node = queue.front();

                queue.pop_front();

                if (node->left != nullptr) {
                    queue.emplace_back(node->left);
                }

                if (node->right != nullptr) {
                    queue.emplace_back(node->right);
                }
            }

            if (queue.empty()) {
                break;
            }
        }
    }

    return result;
}
} // namespace leet_code::test_utilities
