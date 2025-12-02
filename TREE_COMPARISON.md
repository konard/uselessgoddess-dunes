# Tree Structure Comparison for Graph Database Integer Key Indexing

This document compares various balanced tree structures for storing 64-bit integer keys in a graph database context where relations are indexed as u8/u16/u32/u64 keys.

## 1. Red-Black Tree (RB Tree)

**Search Performance**: O(log n) worst-case, excellent cache locality with path copying

**Insertion Performance**: O(log n) worst-case, 1-2 rotations typically needed for rebalancing

**Memory Footprint**: 3 pointers + 1 bit (color flag) per node. For u64 keys: ~25 bytes per node (with padding)

**Ease of Persistence**: Good - incremental changes only affect path to root, supports copy-on-write naturally

**Suitability for Raw Integers**: Excellent - color bit can be packed into pointer or stored separately

**Range Query Support**: Good - ordered traversal is straightforward

**In-Memory vs Mmap**: Works well for both. Fixed-size nodes benefit from memory mapping

**Implementation Complexity**: Moderate - rotation logic and color rules are well-documented

**Worst-Case Consistency**: Guaranteed O(log n) operations, no amortization needed

**Randomization Effects**: None - deterministic structure

**Database Tradeoffs**: Better than B-trees for in-memory but worse for disk due to pointer chasing

## 2. AVL Tree

**Search Performance**: O(log n) worst-case, slightly faster than RB trees due to stricter balancing

**Insertion Performance**: O(log n) worst-case, may require multiple rotations (up to log n)

**Memory Footprint**: 2 pointers + 2-bit height field per node. For u64 keys: ~24 bytes per node

**Ease of Persistence**: Good - similar to RB trees, path copying for persistence

**Suitability for Raw Integers**: Excellent - height field can be packed efficiently

**Range Query Support**: Excellent - ordered structure supports efficient range scans

**In-Memory vs Mmap**: Good for both, slightly more balanced than RB trees

**Implementation Complexity**: Moderate - 4 rotation cases to handle

**Worst-Case Consistency**: Guaranteed O(log n), stricter balance than RB trees

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Better search than RB trees but slower insertions/deletions

## 3. WAVL Tree (Weak AVL)

**Search Performance**: O(log n) worst-case, similar to AVL trees

**Insertion Performance**: O(log n) worst-case, fewer rotations than AVL on average

**Memory Footprint**: 2 pointers + rank field per node. Similar to AVL: ~24 bytes

**Ease of Persistence**: Good - incremental updates like RB/AVL trees

**Suitability for Raw Integers**: Excellent - rank field is small integer

**Range Query Support**: Excellent - ordered traversal

**In-Memory vs Mmap**: Good for both scenarios

**Implementation Complexity**: High - complex rank rules and rebalancing

**Worst-Case Consistency**: Guaranteed O(log n), combines benefits of AVL and RB trees

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Theoretically optimal but complex implementation

## 4. Treap (Cartesian Tree)

**Search Performance**: O(log n) expected, O(n) worst-case (extremely unlikely)

**Insertion Performance**: O(log n) expected, simple rotation-based rebalancing

**Memory Footprint**: 2 pointers + priority field per node: ~24-32 bytes

**Ease of Persistence**: Excellent - very natural for persistent data structures

**Suitability for Raw Integers**: Good - needs extra priority field (can use hash of key)

**Range Query Support**: Good - maintains BST property

**In-Memory vs Mmap**: Better for in-memory due to randomization requirements

**Implementation Complexity**: Low - simple rotation rules based on heap property

**Worst-Case Consistency**: Probabilistic - expected O(log n), no worst-case guarantee

**Randomization Effects**: Critical - requires good random priorities for balance

**Database Tradeoffs**: Simpler than deterministic trees but unpredictable performance

## 5. Skip List

**Search Performance**: O(log n) expected, excellent cache locality on lower levels

**Insertion Performance**: O(log n) expected, no rotations needed

**Memory Footprint**: Variable - average 2 pointers per node, ~16-32 bytes depending on level

**Ease of Persistence**: Moderate - multiple pointers per node complicate persistence

**Suitability for Raw Integers**: Excellent - just keys and forward pointers

**Range Query Support**: Excellent - bottom level provides sorted sequence

**In-Memory vs Mmap**: Good for both, sequential nature helps with caching

**Implementation Complexity**: Low - very simple to implement correctly

**Worst-Case Consistency**: Probabilistic - O(n) worst-case, O(log n) expected

**Randomization Effects**: Uses randomness for level assignment

**Database Tradeoffs**: Simpler than B-trees, good concurrent access potential

## 6. Splay Tree

**Search Performance**: O(log n) amortized, O(n) worst-case per operation

**Insertion Performance**: O(log n) amortized, self-adjusting brings accessed items to root

**Memory Footprint**: Minimal - only 2 pointers per node: ~16 bytes

**Ease of Persistence**: Poor - constant restructuring makes persistence difficult

**Suitability for Raw Integers**: Excellent - minimal metadata

**Range Query Support**: Poor - structure changes with every access

**In-Memory vs Mmap**: Better for in-memory due to frequent restructuring

**Implementation Complexity**: Moderate - splaying operation is subtle

**Worst-Case Consistency**: Amortized only - individual operations can be O(n)

**Randomization Effects**: None - deterministic but access-pattern dependent

**Database Tradeoffs**: Good for temporal locality, poor for uniform access

## 7. Scapegoat Tree

**Search Performance**: O(log n) worst-case, no extra overhead

**Insertion Performance**: O(log n) amortized, occasional full subtree rebuild

**Memory Footprint**: Minimal - 2 pointers per node: ~16 bytes

**Ease of Persistence**: Moderate - rebuilds complicate persistence

**Suitability for Raw Integers**: Excellent - no extra metadata

**Range Query Support**: Good - maintains BST property

**In-Memory vs Mmap**: Better for in-memory due to rebuild operations

**Implementation Complexity**: Low-moderate - simple balance criterion

**Worst-Case Consistency**: Amortized - rebuilds take O(n) time

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Simple implementation but unpredictable rebuild pauses

## 8. Size-Balanced Tree (SBT)

**Search Performance**: O(log n) worst-case, very good cache behavior

**Insertion Performance**: O(log n) worst-case, efficient rebalancing with size field

**Memory Footprint**: 2 pointers + size field per node: ~24 bytes

**Ease of Persistence**: Good - size field makes incremental updates efficient

**Suitability for Raw Integers**: Excellent - size enables order statistics

**Range Query Support**: Excellent - supports efficient rank/select operations

**In-Memory vs Mmap**: Excellent for both - size field benefits mmap scenarios

**Implementation Complexity**: Moderate - straightforward rotation rules based on size

**Worst-Case Consistency**: Guaranteed O(log n) operations

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Excellent balance of simplicity and performance

## 9. Weight-Balanced Tree (WBT)

**Search Performance**: O(log n) worst-case

**Insertion Performance**: O(log n) worst-case, weight-based rebalancing

**Memory Footprint**: 2 pointers + weight field: ~24 bytes

**Ease of Persistence**: Good - similar to SBT

**Suitability for Raw Integers**: Excellent - supports functional implementations

**Range Query Support**: Excellent - maintains sorted order

**In-Memory vs Mmap**: Good for both scenarios

**Implementation Complexity**: Moderate - weight ratio maintenance

**Worst-Case Consistency**: Guaranteed O(log n)

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Similar to SBT, good for functional/persistent structures

## 10. B-Tree

**Search Performance**: O(log n) worst-case, excellent for disk-based storage

**Insertion Performance**: O(log n) worst-case, bulk operations efficient

**Memory Footprint**: High per node (multiple keys) but fewer nodes: order m needs ~m*8 bytes per key

**Ease of Persistence**: Excellent - designed for persistence, incremental updates

**Suitability for Raw Integers**: Excellent - keys stored contiguously in nodes

**Range Query Support**: Excellent - very cache-friendly sequential scans

**In-Memory vs Mmap**: Optimal for mmap/disk-backed storage

**Implementation Complexity**: Moderate-high - node splitting/merging logic

**Worst-Case Consistency**: Guaranteed O(log n)

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Industry standard for database indexes

## 11. B+ Tree

**Search Performance**: O(log n) worst-case, excellent cache behavior

**Insertion Performance**: O(log n) worst-case, efficient bulk loading

**Memory Footprint**: Similar to B-tree but all data in leaves

**Ease of Persistence**: Excellent - optimal for database storage

**Suitability for Raw Integers**: Excellent - compact leaf storage

**Range Query Support**: Best - leaves linked for sequential scans

**In-Memory vs Mmap**: Optimal for mmap-backed storage, good for in-memory

**Implementation Complexity**: High - separate internal/leaf node handling

**Worst-Case Consistency**: Guaranteed O(log n)

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Standard choice for database indexes, especially on disk

## 12. Finger Tree

**Search Performance**: O(log n) amortized

**Insertion Performance**: O(log n) amortized at ends, O(log n) general

**Memory Footprint**: High - nested structure with cached measurements

**Ease of Persistence**: Excellent - designed for persistent structures

**Suitability for Raw Integers**: Moderate - high overhead for simple keys

**Range Query Support**: Good - supports split/concatenate efficiently

**In-Memory vs Mmap**: Better for in-memory, complex for mmap

**Implementation Complexity**: Very high - monoid-based generic structure

**Worst-Case Consistency**: Amortized bounds only

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Overkill for simple key-value storage

## 13. Jump List / Indexed Skip List

**Search Performance**: O(log n) expected with position tracking

**Insertion Performance**: O(log n) expected

**Memory Footprint**: Similar to skip lists + width counts: ~24-40 bytes

**Ease of Persistence**: Moderate - width counts complicate updates

**Suitability for Raw Integers**: Good - adds order statistics to skip lists

**Range Query Support**: Excellent - supports rank operations

**In-Memory vs Mmap**: Better for in-memory

**Implementation Complexity**: Moderate - skip list + width maintenance

**Worst-Case Consistency**: Probabilistic - expected O(log n)

**Randomization Effects**: Uses randomization for levels

**Database Tradeoffs**: Good for concurrent access with order statistics

## 14. Van Emde Boas Tree (vEB)

**Search Performance**: O(log log n) worst-case for fixed universe size

**Insertion Performance**: O(log log n) worst-case

**Memory Footprint**: O(sqrt(u)) where u is universe size - impractical for u64

**Ease of Persistence**: Poor - complex recursive structure

**Suitability for Raw Integers**: Moderate - excellent for small universes (u16), impractical for u64

**Range Query Support**: Excellent - successor/predecessor in O(log log n)

**In-Memory vs Mmap**: In-memory only - memory usage prohibitive for u64

**Implementation Complexity**: Very high - complex recursive structure

**Worst-Case Consistency**: Guaranteed O(log log n) but with huge constants

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Theoretical interest only for u64, practical for u16

## 15. Fusion Tree

**Search Performance**: O(log_w n) where w is word size

**Insertion Performance**: O(log_w n)

**Memory Footprint**: High - uses bit-parallel operations

**Ease of Persistence**: Poor - complex in-node structures

**Suitability for Raw Integers**: Moderate - designed for integers but complex

**Range Query Support**: Possible but complicated

**In-Memory vs Mmap**: In-memory only

**Implementation Complexity**: Extremely high - bit-level tricks

**Worst-Case Consistency**: Guaranteed but with large constants

**Randomization Effects**: None - deterministic

**Database Tradeoffs**: Theoretical interest only, impractical to implement

## 16. Order Statistic Tree (Augmented BST)

**Search Performance**: O(log n) worst-case depending on base tree

**Insertion Performance**: O(log n) worst-case

**Memory Footprint**: Base tree + size/count field per node

**Ease of Persistence**: Same as base tree

**Suitability for Raw Integers**: Excellent - adds rank/select to any BST

**Range Query Support**: Excellent - supports rank queries efficiently

**In-Memory vs Mmap**: Inherits properties from base tree

**Implementation Complexity**: Moderate - augments existing tree implementation

**Worst-Case Consistency**: Depends on base tree (RB/AVL/SBT)

**Randomization Effects**: Depends on base tree

**Database Tradeoffs**: Augmentation technique applicable to any base tree

## 17. Lock-Free Skip List

**Search Performance**: O(log n) expected, excellent for concurrent reads

**Insertion Performance**: O(log n) expected, wait-free reads

**Memory Footprint**: Skip list + atomic pointers: ~24-48 bytes per node

**Ease of Persistence**: Poor - concurrent structure complicates persistence

**Suitability for Raw Integers**: Good - simple concurrent structure

**Range Query Support**: Excellent - bottom level provides sorted sequence

**In-Memory vs Mmap**: In-memory only - requires atomic operations

**Implementation Complexity**: High - correct lock-free implementation is subtle

**Worst-Case Consistency**: Probabilistic expected bounds

**Randomization Effects**: Uses randomization for levels

**Database Tradeoffs**: Excellent for highly concurrent in-memory workloads

## 18. Concurrent Search Trees (Ctrie, B-link)

**Search Performance**: O(log n) with concurrent access

**Insertion Performance**: O(log n) with fine-grained locking

**Memory Footprint**: Higher due to synchronization metadata

**Ease of Persistence**: Moderate - designed for concurrent access not persistence

**Suitability for Raw Integers**: Good - designed for concurrent databases

**Range Query Support**: Good but complicated by concurrency

**In-Memory vs Mmap**: Primarily in-memory, complex for mmap

**Implementation Complexity**: Very high - correct concurrent tree implementation is difficult

**Worst-Case Consistency**: Depends on specific structure

**Randomization Effects**: Depends on structure (Ctrie uses hashing)

**Database Tradeoffs**: Essential for high-concurrency scenarios

## Recommendations

### For In-Memory Indexing

**Winner: Size-Balanced Tree (SBT)**

Rationale:
- Guaranteed O(log n) operations without amortization
- Minimal memory overhead (size field naturally supports order statistics)
- Simple, clean implementation suitable for Rust
- Excellent cache locality
- No randomization needed
- Well-suited for u8/u16/u32/u64 keys

**Runner-up: Red-Black Tree**

Rationale:
- Proven in production (used in Linux kernel, C++ std::map)
- Slightly fewer rotations than AVL
- Well-documented algorithms

### For Persistent Mmap-Backed Indexing

**Winner: B+ Tree**

Rationale:
- Designed specifically for disk-based storage
- Excellent sequential scan performance (linked leaves)
- Bulk loading support
- Industry standard for database indexes
- Minimizes disk I/O through high fanout

**Runner-up: B-Tree**

Rationale:
- Similar benefits to B+ tree
- Simpler implementation
- Good when point queries dominate

### For Hybrid Design (Cache + On-Disk)

**Winner: Two-tier approach**

1. In-memory hot cache: SBT or RB Tree for frequently accessed keys
2. Disk-backed cold storage: B+ Tree for bulk data

Rationale:
- SBT provides fast in-memory operations
- B+ tree optimized for sequential disk access
- Clear migration path as data moves between tiers
- Each structure optimized for its storage medium

**Alternative: Adaptive Radix Tree (ART)** - Not listed above but worth considering

- Excellent for integer keys
- Cache-friendly through node path compression
- Space-efficient for sparse key spaces
- Growing adoption in modern databases (PostgreSQL, HyPer)

## Implementation Priority for Dunes Project

Given the project context (graph database with growing memory):

1. Start with Size-Balanced Tree implementation
   - Clean Rust implementation using the &mut [Node] slice approach
   - Matches the trees-rs inspiration
   - Simple to test and benchmark

2. Add proptest coverage
   - Property tests for insert/delete/search invariants
   - Shrinking support for debugging

3. Optimize for cache hits
   - Benchmark with criterion
   - Profile with perf/cachegrind
   - Consider node layout optimizations

4. Future: Add B+ tree for persistent storage if needed
   - When database needs to exceed memory
   - Reuse same Node abstraction if possible
