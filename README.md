# Cheap and dirty RNGs

```rust
use voxell_rng::rng::XorShift32;
// seeds using runtime entropy (no overhead / no syscalls)
let mut rng = XorShift32::default();
rng.next_u32();
```

### Welcome to the land of unreproducible builds

Use this crate if you need simple random number generators for your project
and you don't want to depend on a big library like `rand`.

You can seed your RNGs using the system time [`voxell_rng::time_seeded`] or runtime entropy [`voxell_rng::runtime_seeded`].

There are 4 RNGs available:

-   [`rng::SplitMix64`]: a 64-bit RNG with 64-bit output used for seeding other RNGs
-   [`rng::XorShift32`]: a 32-bit Xorshift RNG with 32-bit output
-   [`rng::XoRoShiRo128Plus`]: a 128-bit XoRoShiRo RNG with 64-bit output
-   [`rng::Pcg8`] through [`rng::Pcg128`]: the PCG family of RNGs

All RNGs implement `BranchRng` which is a simple trait that provides a `branch_rng` method
for creating a new divergent RNG from the current one. The resulting RNG will have a different
state and will produce different random numbers without needing to specify a new seed.

## Examples

1. Create a new RNG using a constant seed and use it:

```rust
use voxell_rng::rng::XorShift32;

// create the rng
let mut rng = XorShift32::new(0xcafebabe as u64);

// generate a new number
rng.next_f32();
```

2. Seed your RNGs using the system time

```rust
use voxell_rng::time_seeded::TimeSeededXorShift32;
let mut rng = TimeSeededXorShift32::generate().unwrap();
rng.next_f32();
```

3. Seed your RNGs using runtime entropy

```rust
use voxell_rng::runtime_seeded::MagicallySeededXorShift32;
let mut rng = MagicallySeededXorShift32::new_magic();
rng.next_f32();

let mut rng2 = MagicallySeededXorShift32::new_with_reference(&());
rng2.next_f32();
```

4. Create new RNGs from a master RNG for divergent thread local RNGs:

```rust
use voxell_rng::branch_rng::BranchRng;
use voxell_rng::rng::XoRoShiRo128Plus;

let mut master_rng = XoRoShiRo128Plus::new(0xabad1dea as u64);
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
