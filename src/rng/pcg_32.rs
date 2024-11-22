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
pub struct Pcg32 {
    state: PcgInnerState32,
}

impl Pcg32 {
    #[inline]
    #[must_use]
    #[must_use]
    pub fn new(seed: u32) -> Self {
        let state = PcgInnerState32::unique_seeded(seed);
        Self { state }
    }

    #[inline]
    pub fn step(&mut self) -> u32 {
        self.state.oneseq_rxs_m_xs()
    }
}

impl RngCore for Pcg32 {
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut chunksmut = dest.chunks_exact_mut(4);
        for chunk in chunksmut.by_ref() {
            let next = self.next_u32();
            let bytes = next.to_le_bytes();
            chunk.copy_from_slice(&bytes);
        }
        let a = chunksmut.into_remainder();
        if !a.is_empty() {
            let next = self.next_u32();
            let bytes = next.to_le_bytes();
            a.copy_from_slice(&bytes[0..a.len()]);
        }
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.step()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        u64::from(self.step()) << 32 | u64::from(self.step())
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerState32 {
    state: u32,
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerStateSetseq32 {
    state: u32,
    inc: u32,
}

impl BranchRng<Self> for Pcg32 {
    #[inline]
    fn branch_rng(&mut self) -> Self {
        let mut oldstate = self.state.clone();
        PcgInnerState32::oneseq_advance(&mut oldstate, 1);
        Self { state: oldstate }
    }
}

const PCG32_DEFAULT_MULT: u32 = 747796405;
const PCG32_DEFAULT_INC: u32 = 2891336453;
const PCG32_ONESEQ_INIT: u32 = 0x46b56677;
const PCG32_UNIQUE_INIT: u32 = PCG32_ONESEQ_INIT;
const PCG32_MCG_INIT: u32 = 0xd15ea5e5;
const PCG32_SETSEQ_INIT: [u32; 2] = [0xec02d89b, 0x94b95bdb];

impl PcgInnerState32 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0 }
    }

    #[inline]
    pub const fn oneseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG32_DEFAULT_MULT)
            .wrapping_add(PCG32_DEFAULT_INC);
    }

    #[inline]
    pub const fn oneseq_advance(&mut self, delta: u32) {
        self.state = pcg32_advance_lcg(self.state, delta, PCG32_DEFAULT_MULT, PCG32_DEFAULT_INC);
    }

    #[inline]
    pub const fn mcg_step(&mut self) {
        self.state = self.state.wrapping_mul(PCG32_DEFAULT_MULT);
    }

    #[inline]
    pub const fn mcg_advance(&mut self, delta: u32) {
        self.state = pcg32_advance_lcg(self.state, delta, PCG32_DEFAULT_MULT, 0);
    }

    #[inline]
    pub fn unique_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG32_DEFAULT_MULT)
            .wrapping_add(ptr::from_mut(self) as u32 | 1);
    }

    #[inline]
    pub fn unique_advance(&mut self, delta: u32) {
        self.state = pcg32_advance_lcg(
            self.state,
            delta,
            PCG32_DEFAULT_MULT,
            ptr::from_mut(self) as u32 | 1,
        );
    }

    #[inline]
    #[must_use]
    pub const fn oneseq_seeded(initstate: u32) -> Self {
        let mut pcg = Self::zeroed();
        Self::oneseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::oneseq_step(&mut pcg);
        pcg
    }

    #[inline]
    #[must_use]
    pub const fn mcg_seeded(initstate: u32) -> Self {
        let mut pcg = Self::zeroed();
        pcg.state = initstate | 1;
        pcg
    }

    #[inline]
    #[must_use]
    pub fn unique_seeded(initstate: u32) -> Self {
        let mut pcg = Self::zeroed();
        Self::unique_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::unique_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn oneseq_xsh_rs(&mut self) -> u16 {
        let oldstate: u32 = self.state;
        Self::oneseq_step(self);
        pcg32_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsh_rs_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::oneseq_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsh_rs(&mut self) -> u16 {
        let oldstate: u32 = self.state;
        Self::unique_step(self);
        pcg32_xsh_rs(oldstate)
    }

    #[inline]
    pub fn unique_xsh_rs_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::unique_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsh_rs(&mut self) -> u16 {
        let oldstate: u32 = self.state;
        Self::mcg_step(self);
        pcg32_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn mcg_xsh_rs_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::mcg_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_xsh_rr(&mut self) -> u16 {
        let oldstate: u32 = self.state;
        Self::oneseq_step(self);
        pcg32_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsh_rr_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::oneseq_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsh_rr(&mut self) -> u16 {
        let oldstate: u32 = self.state;
        Self::unique_step(self);
        pcg32_xsh_rr(oldstate)
    }

    #[inline]
    pub fn unique_xsh_rr_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::unique_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsh_rr(&mut self) -> u16 {
        let oldstate: u32 = self.state;
        Self::mcg_step(self);
        pcg32_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn mcg_xsh_rr_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::mcg_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs(&mut self) -> u32 {
        let oldstate: u32 = self.state;
        Self::oneseq_step(self);
        pcg32_rxs_m_xs(oldstate)
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::oneseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_rxs_m_xs(&mut self) -> u32 {
        let oldstate: u32 = self.state;
        Self::unique_step(self);
        pcg32_rxs_m_xs(oldstate)
    }

    #[inline]
    pub fn unique_rxs_m_xs_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::unique_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

impl PcgInnerStateSetseq32 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0, inc: 0 }
    }

    #[inline]
    pub const fn setseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG32_DEFAULT_MULT)
            .wrapping_add(self.inc);
    }

    #[inline]
    pub const fn setseq_advance(&mut self, delta: u32) {
        self.state = pcg32_advance_lcg(self.state, delta, PCG32_DEFAULT_MULT, self.inc);
    }

    #[inline]
    #[must_use]
    pub const fn setseq_seeded(initstate: u32, initseq: u32) -> Self {
        let mut pcg = Self::zeroed();
        pcg.inc = (initseq << 1) | 1;
        Self::setseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::setseq_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn setseq_xsh_rs(&mut self) -> u16 {
        let oldstate: u32 = self.state;
        Self::setseq_step(self);
        pcg32_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn setseq_xsh_rs_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::setseq_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_xsh_rr(&mut self) -> u16 {
        let oldstate: u32 = self.state;
        Self::setseq_step(self);
        pcg32_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn setseq_xsh_rr_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::setseq_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_rxs_m_xs(&mut self) -> u32 {
        let oldstate: u32 = self.state;
        Self::setseq_step(self);
        pcg32_rxs_m_xs(oldstate)
    }

    #[inline]
    pub const fn setseq_rxs_m_xs_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::setseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

#[inline]
#[must_use]
pub const fn pcg32_rxs_m_xs(state: u32) -> u32 {
    let word = ((state >> ((state >> 28).wrapping_add(4))) ^ state).wrapping_mul(277803737);
    (word >> 22) ^ word
}

#[inline]
#[must_use]
pub const fn pcg32_advance_lcg(
    state: u32,
    mut delta: u32,
    mut cur_mult: u32,
    mut cur_plus: u32,
) -> u32 {
    let mut acc_mult: u32 = 1;
    let mut acc_plus: u32 = 0;
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

#[inline]
#[must_use]
pub const fn pcg32_xsh_rs(state: u32) -> u16 {
    let res = (((state >> 11) ^ state) >> ((state >> 30).wrapping_add(11)));
    res as u16
}

#[inline]
#[must_use]
pub const fn pcg32_xsh_rr(state: u32) -> u16 {
    rotr_16((((state >> 10) ^ state) >> 12) as u16, state >> 28)
}

#[inline]
#[must_use]
pub const fn rotr_16(value: u16, rot: u32) -> u16 {
    value.rotate_right(rot)
}
