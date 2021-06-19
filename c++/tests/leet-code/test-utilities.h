#ifndef LEET_CODE_TEST_UTILITIES_H
#define LEET_CODE_TEST_UTILITIES_H

#include <deque>
#include <leet-code/data-structures/list-node.h>
#include <leet-code/data-structures/tree-node-with-next-right-pointer.h>
#include <memory>
#include <optional>
#include <tuple>
#include <vector>

namespace leet_code::test_utilities {
template <class C>
std::tuple<std::vector<std::unique_ptr<data_structures::list_node::ListNode>>, data_structures::list_node::ListNode *>
make_list(C &&values) {
    using data_structures::list_node::ListNode;
    using std::unique_ptr;
    using std::vector;

    auto buffer = vector<unique_ptr<ListNode>>{};

    buffer.reserve(std::size(values));

    auto *head = static_cast<ListNode *>(nullptr);
    auto *tail = &head;

    for (const auto &value : values) {
        *tail = buffer.emplace_back(std::make_unique<ListNode>(value)).get();
        tail = &(*tail)->next;
    }

    return {std::move(buffer), head};
}

template <class T, class C>
std::tuple<std::vector<std::unique_ptr<T>>, T *> make_tree(C &&values) {
    using std::deque;
    using std::unique_ptr;
    using std::vector;

    auto buffer = vector<unique_ptr<T>>{};
    auto *root = static_cast<T *>(nullptr);
    auto it = std::cbegin(values);
    const auto it_end = std::cend(values);

    if (it != it_end && *it) {
        root = buffer.emplace_back(std::make_unique<T>(**it)).get();

        ++it;

        auto queue = deque<T *>{};
        auto *target = root;

        for (; it != it_end; ++it) {
            if (*it) {
                target->left = buffer.emplace_back(std::make_unique<T>(**it)).get();

                queue.emplace_back(target->left);
            }

            ++it;

            if (it == it_end) {
                break;
            }

            if (*it) {
                target->right = buffer.emplace_back(std::make_unique<T>(**it)).get();

                queue.emplace_back(target->right);
            }

            target = queue.front();
            queue.pop_front();
        }
    }

    return {std::move(buffer), root};
}

std::vector<int> list_to_vector(const data_structures::list_node::ListNode *head);

std::vector<std::optional<int>> list_tree_levels_with_next_right_pointers(
    const data_structures::tree_node_with_next_right_pointer::Node *root);
} // namespace leet_code::test_utilities

#endif // LEET_CODE_TEST_UTILITIES_H
