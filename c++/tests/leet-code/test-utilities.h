#ifndef LEET_CODE_TEST_UTILITIES_H
#define LEET_CODE_TEST_UTILITIES_H

#include <leet-code/data-structures/list-node.h>
#include <memory>
#include <tuple>
#include <vector>

namespace leet_code::test_utilities {
template <class C>
std::tuple<std::vector<std::unique_ptr<data_structures::ListNode>>, data_structures::ListNode *> make_list(C &&values) {
    using data_structures::ListNode;
    using std::unique_ptr;
    using std::vector;

    auto buffer = vector<unique_ptr<ListNode>>{};

    buffer.reserve(std::size(values));

    auto head = static_cast<ListNode *>(nullptr);
    auto *tail = &head;

    for (const auto &value : values) {
        *tail = buffer.emplace_back(std::make_unique<ListNode>(value)).get();
        tail = &(*tail)->next;
    }

    return {std::move(buffer), head};
}

std::vector<int> list_to_vector(data_structures::ListNode *head);
} // namespace leet_code::test_utilities

#endif // LEET_CODE_TEST_UTILITIES_H
