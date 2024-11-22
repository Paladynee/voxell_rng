# Cheap and dirty RNGs

```rust
use voxell_rng::prelude::*;
use voxell_rng::time_seeded::TimeSeededXorShift32;
// seeds using os entropy
let mut rng = TimeSeededXorShift32::generate().unwrap();
rng.next_u32();
```

### Welcome to the land of unreproducible builds

Use this crate if you need simple random number generators for your project
and you don't want to depend on a big library like `rand`.

You can seed your RNGs using the system time [`voxell_rng::time_seeded`] or runtime entropy [`voxell_rng::runtime_seeded`].

There are 5 RNGs available:

-   [`SplitMix64`]: a 64-bit RNG with 64-bit output used for seeding other RNGs
-   [`XorShift32`]: a 32-bit Xorshift RNG with 32-bit output
-   [`XorShift128`]: a 128-bit Xorshift RNG with 64-bit output
-   [`XoRoShiRo128`]: a 128-bit XoRoShiRo RNG with 64-bit output
-   [`Pcg8`] through [`Pcg128`]: the PCG family of RNGs

[`SplitMix64`]: crate::rng::SplitMix64
[`XorShift32`]: crate::rng::XorShift32
[`XorShift128`]: crate::rng::XorShift128
[`XoRoShiRo128`]: crate::rng::XoRoShiRo128
[`Pcg8`]: crate::rng::Pcg8
[`Pcg128`]: crate::rng::Pcg128

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
let mut rng = MagicallySeededXorShift32::new_magic().unwrap();
rng.next_f32();

let mut rng2 = MagicallySeededXorShift32::new_with_reference(&());
rng2.next_f32();
```

4. Create new RNGs from a master RNG for divergent thread local RNGs:

```rust
use voxell_rng::prelude::*;
use voxell_rng::rng::XoRoShiRo128;

let mut master_rng = XoRoShiRo128::new(0xabad1dea as u64);
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
