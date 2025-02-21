#pragma once

#include <memory>
#include <utility>
#include <vector>
#include <string>
#include <iostream>
using namespace std;

/* 键值对 */
struct Pair {
    int key;
    string val;
    Pair(int key, string val) : key(key), val(val) {}
    ~Pair() = default;
};

/* 基于数组实现的哈希表 */
class ArrayHashMap {
  private:
    vector<unique_ptr<Pair>> buckets;

  public:
    ArrayHashMap() {
        // 初始化数组，包含 100 个桶
        buckets = vector<unique_ptr<Pair>>(100);
    }

    ~ArrayHashMap() = default;

    /* 哈希函数 */
    int hashFunc(int key) {
        return key % 100;
    }

    /* 查询操作 */
    string get(int key) {
        int index = hashFunc(key);
        if (buckets[index] == nullptr) {
            return "";
        }
        return buckets[index]->val;
    }

    /* 添加操作 */
    void put(int key, string val) {
        auto pair = make_unique<Pair>(key, val);
        int index = hashFunc(key);
        buckets[index] = std::move(pair);
    }

    /* 删除操作 */
    void remove(int key) {
        int index = hashFunc(key);
        buckets[index].reset();
    }

    /* 获取所有键值对 */
    vector<Pair*> pairSet() {
        vector<Pair*> pairSet;
        for (auto &pair : buckets) {
            if (pair != nullptr) {
                pairSet.push_back(pair.get());
            }
        }
        return pairSet;
    }

    /* 获取所有键 */
    vector<int> keySet() {
        vector<int> keySet;
        for (auto &pair : buckets) {
            if (pair != nullptr) {
                keySet.push_back(pair->key);
            }
        }
        return keySet;
    }

    /* 获取所有值 */
    vector<string> valueSet() {
        vector<string> valueSet;
        for (auto &pair : buckets) {
            if (pair != nullptr) {
                valueSet.push_back(pair->val);
            }
        }
        return valueSet;
    }

    /* 打印哈希表 */
    void print() {
        for (Pair *kv : pairSet()) {
            cout << kv->key << " -> " << kv->val << endl;
        }
    }
};