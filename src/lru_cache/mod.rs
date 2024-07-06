use std::collections::HashMap;

/// # [146. LRU Cache](https://leetcode.com/problems/lru-cache/description/?envType=study-plan-v2&envId=top-interview-150)
///
/// Design a data structure that follows the constraints of a **<a href="https://en.wikipedia.org/wiki/Cache_replacement_policies#LRU" target="_blank">Least Recently Used (LRU) cache</a>** .
///
/// Implement the `LRUCache` class:
///
/// - `LRUCache(int capacity)` Initialize the LRU cache with **positive**  size `capacity`.
/// - `int get(int key)` Return the value of the `key` if the key exists, otherwise return `-1`.
/// - `void put(int key, int value)` Update the value of the `key` if the `key` exists. Otherwise, add the `key-value` pair to the cache. If the number of keys exceeds the `capacity` from this operation, **evict**  the least recently used key.
///
/// The functions `get` and `put` must each run in `O(1)` average time complexity.
///
/// **Example 1:**
///
/// ```txt
/// Input
///
/// ["LRUCache", "put", "put", "get", "put", "get", "put", "get", "get", "get"]
/// [[2], [1, 1], [2, 2], [1], [3, 3], [2], [4, 4], [1], [3], [4]]
/// Output
///
/// [null, null, null, 1, null, -1, null, -1, 3, 4]
///
/// Explanation
///
/// LRUCache lRUCache = new LRUCache(2);
/// lRUCache.put(1, 1); // cache is {1=1}
/// lRUCache.put(2, 2); // cache is {1=1, 2=2}
/// lRUCache.get(1);    // return 1
/// lRUCache.put(3, 3); // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
/// lRUCache.get(2);    // returns -1 (not found)
/// lRUCache.put(4, 4); // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
/// lRUCache.get(1);    // return -1 (not found)
/// lRUCache.get(3);    // return 3
/// lRUCache.get(4);    // return 4
/// ```
///
/// **Constraints:**
///
/// - `1 <= capacity <= 3000`
/// - `0 <= key <= 10^4`
/// - `0 <= value <= 10^5`
/// - At most `2 * 10^5` calls will be made to `get` and `put`.
pub struct LRUCache {
    // slow (116ms Beats 16.94%)
    implementation: Box<dyn LRUCacheImpl>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            implementation: Box::new(LRUCacheImpl2 {
                capacity,
                vertices: HashMap::new(),
                front: std::ptr::null_mut(),
            }),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        self.implementation.get(key)
    }

    pub fn put(&mut self, key: i32, value: i32) {
        self.implementation.put(key, value);
    }
}

pub trait LRUCacheImpl {
    fn get(&mut self, key: i32) -> i32;
    fn put(&mut self, key: i32, value: i32);
}

// slow implementation (116ms Beats 16.94%)
#[allow(unused)]
struct LRUCacheImpl1 {
    capacity: i32,
    vertices: HashMap<i32, i32>,         // (vid, value)
    adjacency: HashMap<i32, (i32, i32)>, // (vid, (previous vid, next vid))
    front: Option<i32>,
}

impl LRUCacheImpl for LRUCacheImpl1 {
    fn get(&mut self, key: i32) -> i32 {
        match self.vertices.get(&key) {
            None => -1,
            Some(&value) => {
                self.put(key, value);
                value
            }
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        match self.front {
            None => {
                self.vertices.insert(key, value);
                self.adjacency.insert(key, (key, key));
                self.front = Some(key);
            }
            Some(a) => {
                self.vertices
                    .entry(key)
                    .and_modify(|v| *v = value)
                    .or_insert(value);

                // assume that the structure of the list: <- a <-> b <-> c <-> d ->
                let (d, b) = self.adjacency.get(&a).copied().unwrap();

                match self.adjacency.get(&key).copied() {
                    None => {
                        // push back
                        self.adjacency.insert(key, (d, a));
                        self.adjacency.get_mut(&d).unwrap().1 = key;
                        self.adjacency.get_mut(&a).unwrap().0 = key;

                        if self.vertices.len() > self.capacity as usize {
                            // pop front
                            let (d, b) = self.adjacency.get(&a).copied().unwrap();
                            self.adjacency.get_mut(&d).unwrap().1 = b;
                            self.adjacency.get_mut(&b).unwrap().0 = d;
                            self.adjacency.remove(&a);
                            self.vertices.remove(&a);
                            self.front = Some(b);
                        }
                    }
                    Some((prev, next)) if a != key => {
                        // remove key from the adjacency list
                        self.adjacency.get_mut(&prev).unwrap().1 = next;
                        self.adjacency.get_mut(&next).unwrap().0 = prev;

                        // push key to the back of the adjacency list
                        let (d, _) = self.adjacency.get(&a).copied().unwrap();
                        self.adjacency.get_mut(&d).unwrap().1 = key;
                        self.adjacency.get_mut(&a).unwrap().0 = key;
                        self.adjacency.insert(key, (d, a));
                    }
                    _ => {
                        self.front = Some(b);
                    }
                }
            }
        }
    }
}

struct LRUCacheImpl2 {
    capacity: i32,
    // NOTE: If we use MemoryBlock instead of Box<MemoryBlock>, we might get pointer to an invalid memory location when we add a new element to the vertices.
    vertices: HashMap<i32, Box<MemoryBlock>>,
    front: *mut MemoryBlock,
}

#[derive(Debug)]
struct MemoryBlock {
    pub key: i32,
    pub value: i32,
    pub prev: *mut MemoryBlock,
    pub next: *mut MemoryBlock,
}

impl LRUCacheImpl for LRUCacheImpl2 {
    fn get(&mut self, key: i32) -> i32 {
        if let Some(value) = self.vertices.get(&key).map(|node| node.value) {
            self.put(key, value);
            value
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        unsafe {
            if self.vertices.is_empty() {
                let mut node = Box::new(MemoryBlock {
                    key,
                    value,
                    prev: std::ptr::null_mut(),
                    next: std::ptr::null_mut(),
                });
                self.front = node.as_mut();
                node.prev = node.as_mut();
                node.next = node.as_mut();
                self.vertices.insert(key, node);
                return;
            } else if (*self.front).key == key {
                (*self.front).value = value;
                self.front = (*self.front).next;
                return;
            }
        }

        self.vertices
            .entry(key)
            .and_modify(|node| unsafe {
                (*node.prev).next = node.next;
                (*node.next).prev = node.prev;
                node.prev = (*self.front).prev;
                node.next = self.front;
                (*(*self.front).prev).next = node.as_mut();
                (*self.front).prev = node.as_mut();
                node.value = value;
            })
            .or_insert_with(|| unsafe {
                let mut node = Box::new(MemoryBlock {
                    key,
                    value,
                    prev: (*self.front).prev,
                    next: self.front,
                });
                (*(*self.front).prev).next = node.as_mut();
                (*self.front).prev = node.as_mut();
                node
            });

        if self.vertices.len() > self.capacity as usize {
            unsafe {
                (*(*self.front).prev).next = (*self.front).next;
                (*(*self.front).next).prev = (*self.front).prev;
                let k = (*self.front).key;
                self.front = (*self.front).next;
                self.vertices.remove(&k);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(1), 1);
        lru_cache.put(3, 3);
        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }

    #[test]
    fn example2() {
        let mut lru_cache = LRUCache::new(1);
        lru_cache.put(2, 1);
        assert_eq!(lru_cache.get(2), 1);
        lru_cache.put(3, 2);
        assert_eq!(lru_cache.get(2), -1);
        assert_eq!(lru_cache.get(3), 2);
    }

    #[test]
    fn example3() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(2, 1);
        lru_cache.put(2, 2);
        assert_eq!(lru_cache.get(2), 2);
        lru_cache.put(1, 1);
        lru_cache.put(4, 1);
        assert_eq!(lru_cache.get(2), -1);
    }

    #[test]
    fn example4() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(2, 1);
        lru_cache.put(1, 1);
        lru_cache.put(2, 3);
        lru_cache.put(4, 1);
        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(2), 3);
    }

    #[test]
    fn example5() {
        let mut lru_cache = LRUCache::new(2);
        lru_cache.put(2, 1);
        lru_cache.put(3, 2);
        assert_eq!(lru_cache.get(3), 2);
        assert_eq!(lru_cache.get(2), 1);
        lru_cache.put(4, 3);
        assert_eq!(lru_cache.get(2), 1);
        assert_eq!(lru_cache.get(3), -1);
        assert_eq!(lru_cache.get(4), 3);
    }

    #[test]
    fn example6() {
        let mut lru_cache = LRUCache::new(2);
        assert_eq!(lru_cache.get(2), -1);
        lru_cache.put(2, 6);
        assert_eq!(lru_cache.get(1), -1);
        lru_cache.put(1, 5);
        lru_cache.put(1, 2);
        assert_eq!(lru_cache.get(1), 2);
        assert_eq!(lru_cache.get(2), 6);
    }

    #[test]
    fn example7() {
        let mut lru_cache = LRUCache::new(3);
        lru_cache.put(1, 1);
        lru_cache.put(2, 2);
        lru_cache.put(3, 3);
        lru_cache.put(4, 4);
        assert_eq!(lru_cache.get(4), 4);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(2), 2);
        assert_eq!(lru_cache.get(1), -1);
    }
}
