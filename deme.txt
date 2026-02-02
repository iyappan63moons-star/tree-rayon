## Tree Based Algorithms:

### 1. Load balance

Shared / Sharding - uneven
Tree / Reduction - excellent

Load balance is about how evenly work is distributed across CPU cores.

Shared/Sharding:
    Each thread owns a fixed shard.

    Problem: shards may not have equal work.
    Example: Thread 0 has 10,000 accounts, Thread 1 has 20,000 accounts.
    Result: some threads finish early, others still busy → CPU cores idle → bad load balance.

Tree/Reduction:
    Recursive splitting of tasks (map → reduce).
    Work-stealing scheduler (Rayon) lets idle threads steal work from busy threads.
    Result: all cores stay busy → excellent load balance, even if account sizes vary.

### 2. Tail latency

Shared / Sharding - bad
Tree / Reduction - good

Tail latency is the time taken by the slowest thread to finish its work.

Shared/Sharding:
    If shards are uneven, the slowest shard dictates when the whole computation finishes → long tail latency.

Tree/Reduction:
    Tasks are dynamically split and stolen.
    Even if some accounts are “heavy”, work-stealing balances them across cores → lower tail latency.

HFT impact:
    Lower tail latency → faster risk computation → quicker liquidation decisions.

### 3. CPU saturation

Shared / Sharding - depends
Tree / Reduction - near-optimal

How fully your CPU cores are utilized.

Shared/Sharding:
    Depends on shard sizes: small shard finishes quickly → core sits idle until merge → CPU not fully used.

Tree/Reduction:
    Work-stealing ensures all cores are busy until the last task → near-optimal CPU utilization.

HFT impact:
    Maximizes throughput per CPU → essential for high-speed risk calculations.

### 4. State sharing 

Shared / Sharding - complex
Tree / Reduction - trivial

How difficult it is to safely share data between threads.

Shared/Sharding
    Mutable state might need locks or atomic operations if cross-shard updates happen → complexity increases.

Tree/Reduction:
    Computation is stateless per leaf → only the merge combines results.
    No locks needed → trivial state sharing.

HFT impact:
    Fewer synchronization issues → lower latency, easier debugging.

### 5.Determinism 

Shared / Sharding - tricky
Tree / Reduction - clean

Determinism is whether repeated runs produce the same results.

Shared/Sharding:
    Thread scheduling or shard merge order can vary → FP summations may differ → tricky to get reproducible results.

Tree/Reduction:
    Merges follow a fixed binary-tree pattern → deterministic order → results are reproducible if FP rounding is consistent.

HFT impact:
    Deterministic risk calculations are crucial for auditing, regulatory compliance, and reproducibility.

### 6.Rayon-friendly:

Shared / Sharding - meh
Tree / Reduction - perfect

How well the parallel model fits Rayon’s scheduling model.

Shared/Sharding:
    Threads own shards → fixed work → Rayon cannot steal or dynamically balance → underutilizes Rayon’s scheduler.

Tree/Reduction:
    Recursive tasks + merges → natural fit for Rayon → threads steal tasks dynamically → perfect integration.

HFT impact:
    Using Rayon as intended → minimal scheduler overhead, better core usage, simpler code.