#ifndef LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_BINARY_SEARCH_TREE_H
#define LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_BINARY_SEARCH_TREE_H

#include <algorithm>
#include <tuple>
#include <vector>

namespace leet_code::problem_0406_queue_reconstruction_by_height::binary_search_tree {
using std::vector;

struct Node {
    size_t length;
    vector<int> value;
    Node *left;
    Node *right;
};

class Solution {
    static void insert_node(Node &node, size_t index, Node &value) {
        node.length += 1;

        if (node.left == nullptr) {
            if (index == 0) {
                node.left = &value;
            } else {
                Solution::insert_tree(node.right, index - 1, value);
            }
        } else if (index < node.left->length + 1) {
            Solution::insert_node(*node.left, index, value);
        } else {
            Solution::insert_tree(node.right, index - (node.left->length + 1), value);
        }
    }

    static void insert_tree(Node *&tree, size_t index, Node &value) {
        if (tree == nullptr) {
            tree = &value;
        } else {
            Solution::insert_node(*tree, index, value);
        }
    }

    static void dfs(Node *tree, vector<vector<int>> &result) {
        if (tree != nullptr) {
            Solution::dfs(tree->left, result);
            result.push_back(std::move(tree->value));
            Solution::dfs(tree->right, result);
        }
    }

public:
    vector<vector<int>> reconstructQueue(vector<vector<int>> &people) {
        std::sort(people.begin(), people.end(), [](const auto &lhs, const auto &rhs) {
            return lhs[0] < rhs[0] || (lhs[0] == rhs[0] && lhs[1] > rhs[1]);
        });

        auto tree = static_cast<Node *>(nullptr);
        auto nodes = vector<Node>{};

        nodes.reserve(people.size());

        while (!people.empty()) {
            auto p = std::move(people.back());

            people.pop_back();

            const auto index = static_cast<size_t>(p[1]);

            nodes.push_back({1, std::move(p), nullptr, nullptr});
            Solution::insert_tree(tree, index, nodes.back());
        }

        Solution::dfs(tree, people);

        return people;
    }
};
} // namespace leet_code::problem_0406_queue_reconstruction_by_height::binary_search_tree

#endif // LEET_CODE_PROBLEM_0406_QUEUE_RECONSTRUCTION_BY_HEIGHT_BINARY_SEARCH_TREE_H
