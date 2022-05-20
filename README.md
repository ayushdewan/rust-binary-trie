# rust-binary-trie
implements a generic integer set

Operations/Features:
* Supports types: `u8, u16, u32, u64, u128, usize`
* Supports following operations:
    * New: Creates new set in O(1)
    * Len: Check length in O(1)
    * Add: Adds a new element `n` in O(log n)
    * Delete: Removes an element `n` or does nothing if not present in O(log n)
* Supports following queries:
    * Exists: Checks if an element `n` exists in O(log n)
    * Min/Max: Outputs the min/max element `m` in O(log m)
    * Median (bonus): Outputs the median element `m` in O(log m)