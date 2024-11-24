#![allow(unused)]
#![allow(
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::unreadable_literal,
    missing_docs
)]

use core::ptr;

use rand_core::RngCore;

use crate::branch_rng::BranchRng;

#[derive(Clone, PartialEq, Eq)]
pub struct Pcg128 {
    state: PcgInnerState128,
}

impl BranchRng<Self> for Pcg128 {
    #[inline]
    fn branch_rng(&mut self) -> Self {
        let mut oldstate = self.state.clone();
        PcgInnerState128::oneseq_advance(&mut oldstate, 1);
        Self { state: oldstate }
    }
}

impl Default for Pcg128 {
    #[inline]
    fn default() -> Self {
        Self {
            state: PcgInnerState128::oneseq_seeded(PCG128_ONESEQ_INIT),
        }
    }
}

impl Pcg128 {
    #[inline]
    #[must_use]
    pub fn new(seed: u128) -> Self {
        let state = PcgInnerState128::unique_seeded(seed);
        Self { state }
    }

    #[inline]
    pub fn next_u128(&mut self) -> u128 {
        self.state.oneseq_rxs_m_xs()
    }
}

impl RngCore for Pcg128 {
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut chunksmut = dest.chunks_exact_mut(16);
        for chunk in chunksmut.by_ref() {
            let next = self.next_u128();
            let bytes = next.to_le_bytes();
            chunk.copy_from_slice(&bytes);
        }
        let a = chunksmut.into_remainder();
        if !a.is_empty() {
            let next = self.next_u128();
            let bytes = next.to_le_bytes();
            a.copy_from_slice(&bytes[0..a.len()]);
        }
    }

    #[inline]
    #[expect(clippy::cast_possible_truncation)]
    fn next_u32(&mut self) -> u32 {
        // TODO: use output functions that dont use the entire inner state
        self.next_u128() as u32
    }
    #[inline]
    fn next_u64(&mut self) -> u64 {
        // TODO: use output functions that dont use the entire inner state
        self.next_u128() as u64
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerState128 {
    state: u128,
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerStateSetseq128 {
    state: u128,
    inc: u128,
}

const PCG128_DEFAULT_MUL: u128 = pcg128_const(2549297995355413924, 4865540595714422341);
const PCG128_DEFAULT_INC: u128 = pcg128_const(6364136223846793005, 1442695040888963407);
const PCG128_ONESEQ_INIT: u128 = pcg128_const(0xb8dc10e158a92392, 0x98046df007ec0a53);
const PCG128_UNIQUE_INIT: u128 = PCG128_ONESEQ_INIT;
const PCG128_MCG_INIT: u128 = pcg128_const(0x0000000000000000, 0xcafef00dd15ea5e5);
const PCG128_SETSEQ_INIT: [u128; 2] = [
    pcg128_const(0x979c9a98d8462005, 0x7d3e9cb6cfe0549b),
    pcg128_const(0x0000000000000001, 0xda3e39cb94b95bdb),
];

impl PcgInnerState128 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0 }
    }

    #[inline]
    pub const fn oneseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG128_DEFAULT_MUL)
            .wrapping_add(PCG128_DEFAULT_INC);
    }

    #[inline]
    pub const fn oneseq_advance(&mut self, delta: u128) {
        self.state = pcg128_advance_lcg(self.state, delta, PCG128_DEFAULT_MUL, PCG128_DEFAULT_INC);
    }

    #[inline]
    pub const fn mcg_step(&mut self) {
        self.state = self.state.wrapping_mul(PCG128_DEFAULT_MUL);
    }

    #[inline]
    pub const fn mcg_advance(&mut self, delta: u128) {
        self.state = pcg128_advance_lcg(self.state, delta, PCG128_DEFAULT_MUL, 0);
    }

    #[inline]
    pub fn unique_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG128_DEFAULT_MUL)
            .wrapping_add(ptr::from_mut(self) as u128 | 1);
    }

    #[inline]
    pub fn unique_advance(&mut self, delta: u128) {
        self.state = pcg128_advance_lcg(
            self.state,
            delta,
            PCG128_DEFAULT_MUL,
            ptr::from_mut(self) as u128 | 1,
        );
    }

    #[inline]
    #[must_use]
    pub const fn oneseq_seeded(initstate: u128) -> Self {
        let mut pcg = Self::zeroed();
        Self::oneseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::oneseq_step(&mut pcg);
        pcg
    }

    #[inline]
    #[must_use]
    pub const fn mcg_seeded(initstate: u128) -> Self {
        let mut pcg = Self::zeroed();
        pcg.state = initstate | 1;
        pcg
    }

    #[inline]
    #[must_use]
    pub fn unique_seeded(initstate: u128) -> Self {
        let mut pcg = Self::zeroed();
        Self::unique_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::unique_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn oneseq_xsh_rs(&mut self) -> u64 {
        Self::oneseq_step(self);
        pcg128_xsh_rs(self.state)
    }

    #[inline]
    pub const fn oneseq_xsh_rs_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::oneseq_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }
    #[inline]
    pub fn unique_xsh_rs(&mut self) -> u64 {
        Self::unique_step(self);
        pcg128_xsh_rs(self.state)
    }

    #[inline]
    pub fn unique_xsh_rs_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::unique_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsh_rs(&mut self) -> u64 {
        Self::mcg_step(self);
        pcg128_xsh_rs(self.state)
    }

    #[inline]
    pub const fn mcg_xsh_rs_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::mcg_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_xsh_rr(&mut self) -> u64 {
        Self::oneseq_step(self);
        pcg128_xsh_rr(self.state)
    }

    #[inline]
    pub const fn oneseq_xsh_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::oneseq_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsh_rr(&mut self) -> u64 {
        Self::unique_step(self);
        pcg128_xsh_rr(self.state)
    }

    #[inline]
    pub fn unique_xsh_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::unique_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsh_rr(&mut self) -> u64 {
        Self::mcg_step(self);
        pcg128_xsh_rr(self.state)
    }

    #[inline]
    pub const fn mcg_xsh_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::mcg_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs(&mut self) -> u128 {
        Self::oneseq_step(self);
        pcg128_rxs_m_xs(self.state)
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs_bounded(&mut self, bound: u128) -> u128 {
        let threshold: u128 = bound.wrapping_neg() % bound;
        loop {
            let r: u128 = Self::oneseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_rxs_m_xs(&mut self) -> u128 {
        Self::unique_step(self);
        pcg128_rxs_m_xs(self.state)
    }

    #[inline]
    pub fn unique_rxs_m_xs_bounded(&mut self, bound: u128) -> u128 {
        let threshold: u128 = bound.wrapping_neg() % bound;
        loop {
            let r: u128 = Self::unique_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_xsl_rr(&mut self) -> u64 {
        Self::oneseq_step(self);
        pcg128_xsl_rr(self.state)
    }

    #[inline]
    pub const fn oneseq_xsl_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::oneseq_xsl_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsl_rr(&mut self) -> u64 {
        Self::unique_step(self);
        pcg128_xsl_rr(self.state)
    }

    #[inline]
    pub fn unique_xsl_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::unique_xsl_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsl_rr(&mut self) -> u64 {
        Self::mcg_step(self);
        pcg128_xsl_rr(self.state)
    }

    #[inline]
    pub const fn mcg_xsl_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::mcg_xsl_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_xsl_rr_rr(&mut self) -> u128 {
        let oldstate: u128 = self.state;
        Self::oneseq_step(self);
        pcg128_xsl_rr_rr(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsl_rr_rr_bounded(&mut self, bound: u128) -> u128 {
        let threshold: u128 = bound.wrapping_neg() % bound;
        loop {
            let r: u128 = Self::oneseq_xsl_rr_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsl_rr_rr(&mut self) -> u128 {
        let oldstate: u128 = self.state;
        Self::unique_step(self);
        pcg128_xsl_rr_rr(oldstate)
    }

    #[inline]
    pub fn unique_xsl_rr_rr_bounded(&mut self, bound: u128) -> u128 {
        let threshold: u128 = bound.wrapping_neg() % bound;
        loop {
            let r: u128 = Self::unique_xsl_rr_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

impl PcgInnerStateSetseq128 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0, inc: 0 }
    }

    #[inline]
    pub const fn setseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG128_DEFAULT_MUL)
            .wrapping_add(self.inc);
    }

    #[inline]
    pub const fn setseq_advance(&mut self, delta: u128) {
        self.state = pcg128_advance_lcg(self.state, delta, PCG128_DEFAULT_MUL, self.inc);
    }

    #[inline]
    #[must_use]
    pub const fn setseq_seeded(initstate: u128, initseq: u128) -> Self {
        let mut pcg = Self::zeroed();
        pcg.inc = (initseq << 1) | 1;
        Self::setseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::setseq_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn setseq_xsh_rs(&mut self) -> u64 {
        Self::setseq_step(self);
        pcg128_xsh_rs(self.state)
    }

    #[inline]
    pub const fn setseq_xsh_rs_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::setseq_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_xsh_rr(&mut self) -> u64 {
        Self::setseq_step(self);
        pcg128_xsh_rr(self.state)
    }

    #[inline]
    pub const fn setseq_xsh_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::setseq_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_rxs_m_xs(&mut self) -> u128 {
        Self::setseq_step(self);
        pcg128_rxs_m_xs(self.state)
    }

    #[inline]
    pub const fn setseq_rxs_m_xs_bounded(&mut self, bound: u128) -> u128 {
        let threshold: u128 = bound.wrapping_neg() % bound;
        loop {
            let r: u128 = Self::setseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_xsl_rr(&mut self) -> u64 {
        Self::setseq_step(self);
        pcg128_xsl_rr(self.state)
    }

    #[inline]
    pub const fn setseq_xsl_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::setseq_xsl_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_xsl_rr_rr(&mut self) -> u128 {
        let oldstate: u128 = self.state;
        Self::setseq_step(self);
        pcg128_xsl_rr_rr(oldstate)
    }

    #[inline]
    pub const fn setseq_xsl_rr_rr_bounded(&mut self, bound: u128) -> u128 {
        let threshold: u128 = bound.wrapping_neg() % bound;
        loop {
            let r: u128 = Self::setseq_xsl_rr_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

#[inline]
#[must_use]
pub const fn pcg128_xsl_rr_rr(state: u128) -> u128 {
    let rot1 = (state >> 122) as u32;
    let high = (state >> 64) as u64;
    let low = state as u64;
    let xored = high ^ low;
    let newlow = rotr64(xored, rot1);
    let newhigh = rotr64(high, (newlow & 63) as u32);
    ((newhigh as u128) << 64) | newlow as u128
}

#[inline]
#[must_use]
pub const fn rotr64(value: u64, rot: u32) -> u64 {
    value.rotate_right(rot)
}

#[inline]
#[must_use]
pub const fn pcg128_xsl_rr(state: u128) -> u64 {
    rotr64(((state >> 64) as u64) ^ state as u64, (state >> 122) as u32)
}

#[inline]
#[must_use]
pub const fn pcg128_rxs_m_xs(state: u128) -> u128 {
    let word = ((state >> ((state >> 122).wrapping_add(6))) ^ state)
        .wrapping_mul(pcg128_const(17766728186571221404, 12605985483714917081));
    (word >> 86) ^ word
}

#[inline]
#[must_use]
pub const fn pcg128_const(high: u64, low: u64) -> u128 {
    (high as u128) << 64 | (low as u128)
}

#[inline]
#[must_use]
pub const fn pcg128_xsh_rr(state: u128) -> u64 {
    rotr64(
        (((state >> 29) ^ state) >> 58) as u64,
        (state >> 122) as u32,
    )
}

#[inline]
#[must_use]
pub const fn pcg128_xsh_rs(state: u128) -> u64 {
    let res = ((state >> 43) ^ state) >> ((state >> 124).wrapping_add(45));
    res as u64
}

#[inline]
#[must_use]
pub const fn pcg128_advance_lcg(
    state: u128,
    mut delta: u128,
    mut cur_mult: u128,
    mut cur_plus: u128,
) -> u128 {
    let mut acc_mult: u128 = 1;
    let mut acc_plus: u128 = 0;
    while (delta > 0) {
        if (delta & 1) != 0 {
            acc_mult = acc_mult.wrapping_mul(cur_mult);
            acc_plus = acc_plus.wrapping_mul(cur_mult).wrapping_add(cur_plus);
        }
        cur_plus = (cur_mult.wrapping_add(1)).wrapping_mul(cur_plus);
        cur_mult = cur_mult.wrapping_mul(cur_mult);
        delta = delta.wrapping_div(2);
    }
    acc_mult.wrapping_mul(state).wrapping_add(acc_plus)
}
