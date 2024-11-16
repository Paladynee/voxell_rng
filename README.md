# Cheap and dirty RNGs

Use this crate if you need simple random number generators for your project
and you don't want to depend on a big library like `rand`.

There are 3 RNGs available:

-   `SplitMix64`: a 64-bit RNG with 64-bit output used for seeding other RNGs
-   `Xorshift32`: a 32-bit Xorshift RNG with 32-bit output
-   `XoRoShiRo128Plus`: a 128-bit XoRoShiRo RNG with 64-bit output

All RNGs implement `BranchRng` which is a simple trait that provides a `branch_rng` method
for creating a new divergent RNG from the current one. The resulting RNG will have a different
state and will produce different random numbers without needing to specify a new seed.

## Examples

1. Create a new RNG using a constant seed and use it:

```rust
use cheap_rng::rng::XorShift32;

// create the rng
let mut rng = XorShift32::wrap(0xcafebabe);

// generate a new number
rng.next_f32();
```

2. Seed your RNGs using the system time

```rust
use cheap_rng::time_seeded_rng::TimeSeededXorShift32;
let mut rng = TimeSeededXorShift32::generate().unwrap();
rng.next_f32();
```

3. Create new RNGs from a master RNG for divergent thread local RNGs:

```rust
use cheap_rng::branch_rng::BranchRng;
use cheap_rng::rng::XoRoShiRo128Plus;

let mut master_rng = XoRoShiRo128Plus::seed_using_splitmix(0xabad1dea);
let thread_handles = (0..16)
    .map(|_| {
        let rng = master_rng.branch_rng();
        std::thread::spawn(move || {
            let mut thread_local_rng = rng;
            for _ in 0..1000 {
                thread_local_rng.next_u64();
            }
        })
    })
    .collect::<Vec<_>>();
```
