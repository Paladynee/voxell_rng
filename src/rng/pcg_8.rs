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
pub struct Pcg8 {
    state: PcgInnerState8,
}

impl Default for Pcg8 {
    #[inline]
    fn default() -> Self {
        Self {
            state: PcgInnerState8::oneseq_seeded(PCG8_ONESEQ_INIT),
        }
    }
}

impl BranchRng<Self> for Pcg8 {
    #[inline]
    fn branch_rng(&mut self) -> Self {
        let mut oldstate = self.state.clone();
        PcgInnerState8::oneseq_advance(&mut oldstate, 1);
        Self { state: oldstate }
    }
}

impl Pcg8 {
    #[inline]
    #[must_use]
    pub fn new(seed: u8) -> Self {
        let state = PcgInnerState8::unique_seeded(seed);
        Self { state }
    }

    #[inline]
    pub fn next_u8(&mut self) -> u8 {
        self.state.oneseq_rxs_m_xs()
    }
}

impl RngCore for Pcg8 {
    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for byte in dest {
            *byte = self.next_u8();
        }
    }

    #[inline]
    fn next_u32(&mut self) -> u32 {
        u32::from_le_bytes([
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
        ])
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        u64::from_le_bytes([
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
            self.next_u8(),
        ])
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerState8 {
    state: u8,
}

#[derive(Clone, PartialEq, Eq)]
pub struct PcgInnerStateSetseq8 {
    state: u8,
    inc: u8,
}

const PCG8_DEFAULT_MULT: u8 = 141;
const PCG8_DEFAULT_INC: u8 = 77;
const PCG8_ONESEQ_INIT: u8 = 0xd7;
const PCG8_UNIQUE_INIT: u8 = PCG8_ONESEQ_INIT;
const PCG8_MCG_INIT: u8 = 0xe5;
const PCG8_SETSEQ_INIT: [u8; 2] = [0x9b, 0xdb];

impl PcgInnerState8 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0 }
    }

    #[inline]
    pub const fn oneseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG8_DEFAULT_MULT)
            .wrapping_add(PCG8_DEFAULT_INC);
    }

    #[inline]
    pub const fn oneseq_advance(&mut self, delta: u8) {
        self.state = pcg8_advance_pcg(self.state, delta, PCG8_DEFAULT_MULT, PCG8_DEFAULT_INC);
    }

    #[inline]
    pub const fn mcg_step(&mut self) {
        self.state = self.state.wrapping_mul(PCG8_DEFAULT_MULT);
    }

    #[inline]
    pub const fn mcg_advance(&mut self, delta: u8) {
        self.state = pcg8_advance_pcg(self.state, delta, PCG8_DEFAULT_MULT, 0);
    }

    #[inline]
    pub fn unique_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG8_DEFAULT_MULT)
            .wrapping_add(ptr::from_mut(self) as u8 | 1);
    }

    #[inline]
    pub fn unique_advance(&mut self, delta: u8) {
        self.state = pcg8_advance_pcg(
            self.state,
            delta,
            PCG8_DEFAULT_MULT,
            ptr::from_mut(self) as u8 | 1,
        );
    }

    #[inline]
    #[must_use]
    pub const fn oneseq_seeded(initstate: u8) -> Self {
        let mut pcg = Self::zeroed();
        Self::oneseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::oneseq_step(&mut pcg);
        pcg
    }

    #[inline]
    #[must_use]
    pub const fn mcg_seeded(initstate: u8) -> Self {
        let mut pcg = Self::zeroed();
        pcg.state = initstate | 1;
        pcg
    }

    #[inline]
    #[must_use]
    pub fn unique_seeded(initstate: u8) -> Self {
        let mut pcg = Self::zeroed();
        Self::unique_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::unique_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs(&mut self) -> u8 {
        let oldstate: u8 = self.state;
        Self::oneseq_step(self);
        pcg8_rxs_m_xs(oldstate)
    }

    #[inline]
    pub const fn oneseq_rxs_m_xs_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::oneseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    #[inline]
    pub fn unique_rxs_m_xs(&mut self) -> u8 {
        let oldstate: u8 = self.state;
        Self::unique_step(self);
        pcg8_rxs_m_xs(oldstate)
    }

    #[inline]
    pub fn unique_rxs_ms_xs_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::unique_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

impl PcgInnerStateSetseq8 {
    #[inline]
    #[must_use]
    pub const fn zeroed() -> Self {
        Self { state: 0, inc: 0 }
    }

    #[inline]
    pub const fn setseq_step(&mut self) {
        self.state = self
            .state
            .wrapping_mul(PCG8_DEFAULT_MULT)
            .wrapping_add(self.inc);
    }

    #[inline]
    pub const fn setseq_advance(&mut self, delta: u8) {
        self.state = pcg8_advance_pcg(self.state, delta, PCG8_DEFAULT_MULT, self.inc);
    }

    #[inline]
    #[must_use]
    pub const fn setseq_seeded(initstate: u8, initseq: u8) -> Self {
        let mut pcg = Self::zeroed();
        pcg.inc = (initseq << 1) | 1;
        Self::setseq_step(&mut pcg);
        pcg.state = pcg.state.wrapping_add(initstate);
        Self::setseq_step(&mut pcg);
        pcg
    }

    #[inline]
    pub const fn setseq_rxs_m_xs(&mut self) -> u8 {
        let oldstate: u8 = self.state;
        Self::setseq_step(self);
        pcg8_rxs_m_xs(oldstate)
    }

    #[inline]
    pub const fn setseq_rxs_m_xs_bounded(&mut self, bound: u8) -> u8 {
        let threshold: u8 = bound.wrapping_neg() % bound;
        loop {
            let r: u8 = Self::setseq_rxs_m_xs(self);
            if r >= threshold {
                return r % bound;
            }
        }
    }
}

#[inline]
const fn pcg8_advance_pcg(state: u8, mut delta: u8, mut cur_mult: u8, mut cur_plus: u8) -> u8 {
    let mut acc_mult: u8 = 1;
    let mut acc_plus: u8 = 0;
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
const fn pcg8_rxs_m_xs(state: u8) -> u8 {
    let word = ((state >> ((state >> 6).wrapping_add(2))) ^ state).wrapping_mul(217);
    (word >> 6) ^ word
}
