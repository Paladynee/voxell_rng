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
pub struct Pcg16 {
    state: PcgInnerState16,
}

impl BranchRng<Self> for Pcg16 {
    #[inline]
    fn branch_rng(&mut self) -> Self {
        let mut oldstate = self.state.clone();
        PcgInnerState16::oneseq_advance(&mut oldstate, 1);
        Self { state: oldstate }
    }
}

impl Pcg16 {
    #[inline]
    #[must_use]
    pub fn new(seed: u16) -> Self {
        let state = PcgInnerState16::unique_seeded(seed);
        Self { state }
    }

    #[inline]
    pub fn next_u16(&mut self) -> u16 {
        self.state.unique_rxs_m_xs()
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerState16 {
    state: u16,
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerStateSetseq16 {
    state: u16,
    inc: u16,
}

const PCG16_DEFAULT_MULT: u16 = 12829;
const PCG16_DEFAULT_INC: u16 = 47989;
const PCG16_ONESEQ_INIT: u16 = 0x20df;
const PCG16_UNIQUE_INIT: u16 = PCG16_ONESEQ_INIT;
const PCG16_MCG_INIT: u16 = 0xa5e5;
const PCG16_SETSEQ_INIT: [u16; 2] = [0xe39b, 0x5bdb];

impl PcgInnerState16 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0 }
    }

    #[inline]
    pub const fn oneseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG16_DEFAULT_MULT)
            .wrapping_add(PCG16_DEFAULT_INC);
    }

    #[inline]
    pub const fn oneseq_advance(&mut self, delta: u16) {
        self.state = pcg16_advance_lcg(self.state, delta, PCG16_DEFAULT_MULT, PCG16_DEFAULT_INC);
    }

    #[inline]
    pub const fn mcg_step(&mut self) {
        self.state = self.state.wrapping_mul(PCG16_DEFAULT_MULT);
    }

    #[inline]
    pub const fn mcg_advance(&mut self, delta: u16) {
        self.state = pcg16_advance_lcg(self.state, delta, PCG16_DEFAULT_MULT, 0);
    }

    #[inline]
    pub fn unique_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG16_DEFAULT_MULT)
            .wrapping_add(ptr::from_mut(self) as u16 | 1);
    }

    #[inline]
    pub fn unique_advance(&mut self, delta: u16) {
        self.state = pcg16_advance_lcg(
            self.state,
            delta,
            PCG16_DEFAULT_MULT,
            ptr::from_mut(self) as u16 | 1,
        );
    }

    #[inline]
    #[must_use]
    pub const fn oneseq_seeded(initstate: u16) -> Self {
        let mut pcg = Self::zeroed();
        pcg.state = 0;
        Self::oneseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::oneseq_step(&mut pcg);
        pcg
    }

    #[inline]
    #[must_use]
    pub const fn mcg_seeded(initstate: u16) -> Self {
        let mut pcg = Self::zeroed();
        pcg.state = initstate | 1;
        pcg
    }

    #[inline]
    #[must_use]
    pub fn unique_seeded(initstate: u16) -> Self {
        let mut pcg = Self::zeroed();
        Self::unique_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::unique_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn oneseq_xsh_rs(&mut self) -> u8 {
        let oldstate: u16 = self.state;
        Self::oneseq_step(self);
        pcg16_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsh_rs_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::oneseq_xsh_rs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsh_rs(&mut self) -> u8 {
        let oldstate: u16 = self.state;
        Self::unique_step(self);
        pcg16_xsh_rs(oldstate)
    }

    #[inline]
    pub fn unique_xsh_rs_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::unique_xsh_rs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsh_rs(&mut self) -> u8 {
        let oldstate: u16 = self.state;
        Self::mcg_step(self);
        pcg16_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn mcg_xsh_rs_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::mcg_xsh_rs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_xsh_rr(&mut self) -> u8 {
        let oldstate: u16 = self.state;
        Self::oneseq_step(self);
        pcg16_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn oneseq_xsh_rr_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::oneseq_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_xsh_rr(&mut self) -> u8 {
        let oldstate: u16 = self.state;
        Self::unique_step(self);
        pcg16_xsh_rr(oldstate)
    }

    #[inline]
    pub fn unique_shr_rr_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::unique_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn mcg_xsh_rr(&mut self) -> u8 {
        let oldstate: u16 = self.state;
        Self::mcg_step(self);
        pcg16_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn mcg_xsh_rr_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::mcg_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs(&mut self) -> u16 {
        let oldstate: u16 = self.state;
        Self::oneseq_step(self);
        pcg16_rxs_m_xs(oldstate)
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::oneseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_rxs_m_xs(&mut self) -> u16 {
        let oldstate: u16 = self.state;
        Self::unique_step(self);
        pcg16_rxs_m_xs(oldstate)
    }

    #[inline]
    pub fn unique_rxs_m_xs_bounded(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::unique_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

impl PcgInnerStateSetseq16 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0, inc: 0 }
    }

    #[inline]
    pub const fn setseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG16_DEFAULT_MULT)
            .wrapping_add(self.inc);
    }

    #[inline]
    pub const fn setseq_advance(&mut self, delta: u16) {
        self.state = pcg16_advance_lcg(self.state, delta, PCG16_DEFAULT_MULT, self.inc);
    }

    #[inline]
    #[must_use]
    pub const fn setseq_seeded(initstate: u16, initseq: u16) -> Self {
        let mut pcg = Self::zeroed();
        pcg.inc = (initseq << 1) | 1;
        Self::setseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::setseq_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn setseq_xsh_rs(&mut self) -> u8 {
        let oldstate: u16 = self.state;
        Self::setseq_step(self);
        pcg16_xsh_rs(oldstate)
    }

    #[inline]
    pub const fn setseq_xsh_rs_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::setseq_xsh_rs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_xsh_rr(&mut self) -> u8 {
        let oldstate: u16 = self.state;
        Self::setseq_step(self);
        pcg16_xsh_rr(oldstate)
    }

    #[inline]
    pub const fn pcg_setseq_16_xsh_rr_8_boundedrand_r(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::setseq_xsh_rr(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub const fn setseq_rxs_m_xs(&mut self) -> u16 {
        let oldstate: u16 = self.state;
        Self::setseq_step(self);
        pcg16_rxs_m_xs(oldstate)
    }

    #[inline]
    pub const fn pcg_setseq_16_rxs_m_xs_16_boundedrand_r(&mut self, bound: u16) -> u16 {
        let threshold: u16 = bound.wrapping_neg() % bound;
        loop {
            let r: u16 = Self::setseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

#[inline]
const fn pcg16_advance_lcg(
    state: u16,
    mut delta: u16,
    mut cur_mult: u16,
    mut cur_plus: u16,
) -> u16 {
    let mut acc_mult: u16 = 1;
    let mut acc_plus: u16 = 0;
    while delta > 0 {
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
const fn pcg16_xsh_rs(state: u16) -> u8 {
    let res = ((state >> 7) ^ state) >> ((state >> 14).wrapping_add(3));
    res as u8
}

#[inline]
const fn pcg16_xsh_rr(state: u16) -> u8 {
    rotr8((((state >> 5) ^ state) >> 5) as u8, (state >> 13) as u32)
}

#[inline]
const fn rotr8(value: u8, rot: u32) -> u8 {
    value.rotate_right(rot)
}

#[inline]
const fn pcg16_rxs_m_xs(state: u16) -> u16 {
    let word = ((state >> ((state >> 13).wrapping_add(3))) ^ state).wrapping_mul(62169);
    (word >> 11) ^ word
}
