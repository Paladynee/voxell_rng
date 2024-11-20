#![allow(unused)]
#![allow(
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::unreadable_literal,
    missing_docs
)]

use core::ptr;

use crate::branch_rng::BranchRng;

#[derive(Clone, PartialEq, Eq)]
pub struct Pcg64 {
    state: PcgInnerState64,
}

impl BranchRng<Self> for Pcg64 {
    #[inline]
    fn branch_rng(&mut self) -> Self {
        let mut oldstate = self.state.clone();
        PcgInnerState64::oneseq_advance(&mut oldstate, 1);
        Self { state: oldstate }
    }
}

impl Pcg64 {
    #[inline]
    #[must_use]
    pub fn new(seed: u64) -> Self {
        let state = PcgInnerState64::unique_seeded(seed);
        Self { state }
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state.unique_rxs_m_xs()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerState64 {
    state: u64,
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerStateSetseq64 {
    state: u64,
    inc: u64,
}

const PCG64_DEFAULT_MULT: u64 = 6364136223846793005;
const PCG64_DEFAULT_INC: u64 = 1442695040888963407;
const PCG64_ONESEQ_INIT: u64 = 0x4d595df4d0f33173;
const PCG64_UNIQUE_INIT: u64 = PCG64_ONESEQ_INIT;
const PCG64_MCG_INIT: u64 = 0xcafef00dd15ea5e5;
const PCG64_SETSEQ_INIT: [u64; 2] = [0x853c49e6748fea9b, 0xda3e39cb94b95bdb];

impl PcgInnerState64 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0 }
    }

    #[inline]
    pub const fn oneseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG64_DEFAULT_MULT)
            .wrapping_add(PCG64_DEFAULT_INC);
    }

    #[inline]
    pub const fn oneseq_advance(&mut self, delta: u64) {
        self.state = pcg64_advance_lcg(self.state, delta, PCG64_DEFAULT_MULT, PCG64_DEFAULT_INC);
    }

    #[inline]
    pub const fn mcg_step(&mut self) {
        self.state = self.state.wrapping_mul(PCG64_DEFAULT_MULT);
    }

    #[inline]
    pub const fn mcg_advance(&mut self, delta: u64) {
        self.state = pcg64_advance_lcg(self.state, delta, PCG64_DEFAULT_MULT, 0);
    }

    #[inline]
    pub fn unique_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG64_DEFAULT_MULT)
            .wrapping_add(ptr::from_mut(self) as u64 | 1);
    }

    #[inline]
    pub fn unique_advance(&mut self, delta: u64) {
        self.state = pcg64_advance_lcg(
            self.state,
            delta,
            PCG64_DEFAULT_MULT,
            ptr::from_mut(self) as u64 | 1,
        );
    }

    #[inline]
    #[must_use]
    pub const fn oneseq_seeded(initstate: u64) -> Self {
        let mut pcg = Self::zeroed();
        Self::oneseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::oneseq_step(&mut pcg);
        pcg
    }

    #[inline]
    #[must_use]
    pub const fn mcg_seeded(initstate: u64) -> Self {
        let mut pcg = Self::zeroed();
        pcg.state = initstate | 1;
        pcg
    }

    #[inline]
    #[must_use]
    pub fn unique_seeded(initstate: u64) -> Self {
        let mut pcg = Self::zeroed();
        Self::unique_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::oneseq_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn oneseq_xsh_rs(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::oneseq_step(self);
        pcg64_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsh_rs_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::oneseq_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsh_rs(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::unique_step(self);
        pcg64_xsh_rs(oldstate)
    }

    #[inline]
    pub fn unique_xsh_rs_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::unique_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsh_rs(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::mcg_step(self);
        pcg64_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn mcg_xsh_rs_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::mcg_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_xsh_rr(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::oneseq_step(self);
        pcg64_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsh_rr_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::oneseq_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsh_rr(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::unique_step(self);
        pcg64_xsh_rr(oldstate)
    }

    #[inline]
    pub fn unique_xsh_rr_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::unique_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsh_rr(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::mcg_step(self);
        pcg64_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn mcg_xsh_rr_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::mcg_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs(&mut self) -> u64 {
        let oldstate: u64 = self.state;
        Self::oneseq_step(self);
        pcg64_rxs_m_xs(oldstate)
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::oneseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_rxs_m_xs(&mut self) -> u64 {
        let oldstate: u64 = self.state;
        Self::unique_step(self);
        pcg64_rxs_m_xs(oldstate)
    }

    #[inline]
    pub fn unique_rxs_m_xs_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::unique_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_xsl_rr(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::oneseq_step(self);
        pcg64_xsl_rr(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsl_rr_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::oneseq_xsl_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsl_rr(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::unique_step(self);
        pcg64_xsl_rr(oldstate)
    }

    #[inline]
    pub fn unique_xsl_rr_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::unique_xsl_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsl_rr(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::mcg_step(self);
        pcg64_xsl_rr(oldstate)
    }

    #[inline]
    pub const fn mcg_xsl_rr_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::mcg_xsl_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_xsl_rr_rr(&mut self) -> u64 {
        let oldstate: u64 = self.state;
        Self::oneseq_step(self);
        pcg64_xsl_rr_rr(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsl_rr_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::oneseq_xsl_rr_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsl_rr_rr(&mut self) -> u64 {
        let oldstate: u64 = self.state;
        Self::unique_step(self);
        pcg64_xsl_rr_rr(oldstate)
    }

    #[inline]
    pub fn unique_xsl_rr_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::unique_xsl_rr_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

impl PcgInnerStateSetseq64 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0, inc: 0 }
    }

    #[inline]
    pub const fn setseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG64_DEFAULT_MULT)
            .wrapping_add(self.inc);
    }

    #[inline]
    pub const fn setseq_advance(&mut self, delta: u64) {
        self.state = pcg64_advance_lcg(self.state, delta, PCG64_DEFAULT_MULT, self.inc);
    }

    #[inline]
    #[must_use]
    pub const fn setseq_seeded(initstate: u64, initseq: u64) -> Self {
        let mut pcg = Self::zeroed();
        pcg.inc = (initseq << 1) | 1;
        Self::setseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::setseq_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn setseq_xsh_rs(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::setseq_step(self);
        pcg64_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn setseq_xsh_rs_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::setseq_xsh_rs(self);
            if (r >= threshold) {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_xsh_rr(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::setseq_step(self);
        pcg64_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn setseq_xsh_rr_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::setseq_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_rxs_m_xs(&mut self) -> u64 {
        let oldstate: u64 = self.state;
        Self::setseq_step(self);
        pcg64_rxs_m_xs(oldstate)
    }

    #[inline]
    pub const fn setseq_rxs_m_xs_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::setseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_xsl_rr(&mut self) -> u32 {
        let oldstate: u64 = self.state;
        Self::setseq_step(self);
        pcg64_xsl_rr(oldstate)
    }

    #[inline]
    pub const fn setseq_xsl_rr_bounded(&mut self, bound: u32) -> u32 {
        let threshold: u32 = bound.wrapping_neg() % bound;
        loop {
            let r: u32 = Self::setseq_xsl_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_xsl_rr_rr(&mut self) -> u64 {
        let oldstate: u64 = self.state;
        Self::setseq_step(self);
        pcg64_xsl_rr_rr(oldstate)
    }

    #[inline]
    pub const fn setseq_xsl_rr_rr_bounded(&mut self, bound: u64) -> u64 {
        let threshold: u64 = bound.wrapping_neg() % bound;
        loop {
            let r: u64 = Self::setseq_xsl_rr_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

#[inline]
#[must_use]
pub const fn pcg64_xsh_rs(state: u64) -> u32 {
    let res = ((state >> 22) ^ state) >> ((state >> 61).wrapping_add(22));
    res as u32
}

#[inline]
#[must_use]
pub const fn pcg64_advance_lcg(
    state: u64,
    mut delta: u64,
    mut cur_mult: u64,
    mut cur_plus: u64,
) -> u64 {
    let mut acc_mult: u64 = 1;
    let mut acc_plus: u64 = 0;
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
pub const fn pcg64_xsh_rr(state: u64) -> u32 {
    rotr32((((state >> 18) ^ state) >> 27) as u32, (state >> 59) as u32)
}

#[inline]
#[must_use]
pub const fn rotr32(value: u32, rot: u32) -> u32 {
    value.rotate_right(rot)
}

#[inline]
#[must_use]
pub const fn pcg64_rxs_m_xs(state: u64) -> u64 {
    let word =
        ((state >> ((state >> 59).wrapping_add(5))) ^ state).wrapping_mul(12605985483714917081);
    (word >> 43) ^ word
}

#[inline]
#[must_use]
pub const fn pcg64_xsl_rr(state: u64) -> u32 {
    rotr32(((state >> 32) as u32) ^ state as u32, (state >> 59) as u32)
}

#[inline]
#[must_use]
pub const fn pcg64_xsl_rr_rr(state: u64) -> u64 {
    let rot1 = (state >> 59) as u32;
    let high = (state >> 32) as u32;
    let low = state as u32;
    let xored = high ^ low;
    let newlow = rotr32(xored, rot1);
    let newhigh = rotr32(high, newlow & 31);
    ((newhigh as u64) << 32) | newlow as u64
}
