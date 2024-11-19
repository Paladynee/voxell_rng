#![allow(unused)]
#![allow(
    clippy::arithmetic_side_effects,
    clippy::cast_possible_truncation,
    clippy::unreadable_literal,
    missing_docs
)]

use core::ptr;

pub struct Pcg32 {
    state: PcgInnerState32,
}

impl Pcg32 {
    #[inline]
    pub fn new(seed: u32) -> Self {
        let mut state = PcgInnerState32 { state: 0 };
        PcgInnerState32::unique_seeded(&mut state, seed);

        Self { state }
    }

    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        self.state.unique_rxs_m_xs()
    }
}

pub struct PcgInnerState32 {
    state: u32,
}

pub struct PcgInnerStateSetseq32 {
    state: u32,
    inc: u32,
}

const PCG32_DEFAULT_MULT: u32 = 747796405;
const PCG32_DEFAULT_INC: u32 = 2891336453;
const PCG32_ONESEQ_INIT: u32 = 0x46b56677;
const PCG32_UNIQUE_INIT: u32 = PCG32_ONESEQ_INIT;
const PCG32_MCG_INIT: u32 = 0xd15ea5e5;
const PCG32_SETSEQ_INIT: [u32; 2] = [0xec02d89b, 0x94b95bdb];

impl PcgInnerState32 {
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
    pub const fn oneseq_seeded(&mut self, initstate: u32) {
        self.state = 0;
        Self::oneseq_step(self);
        self.state = self.state.wrapping_add(initstate);
        Self::oneseq_step(self);
    }

    #[inline]
    pub const fn mcg_seeded(&mut self, initstate: u32) {
        self.state = initstate | 1;
    }

    #[inline]
    pub fn unique_seeded(&mut self, initstate: u32) {
        self.state = 0;
        Self::unique_step(self);
        self.state = self.state.wrapping_add(initstate);
        Self::oneseq_step(self);
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
    pub const fn setseq_seeded(&mut self, initstate: u32, initseq: u32) {
        self.state = 0;
        self.inc = (initseq << 1) | 1;
        Self::setseq_step(self);
        self.state = self.state.wrapping_add(initstate);
        Self::setseq_step(self);
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
pub const fn pcg32_rxs_m_xs(state: u32) -> u32 {
    let word = ((state >> ((state >> 28).wrapping_add(4))) ^ state).wrapping_mul(277803737);
    (word >> 22) ^ word
}

#[inline]
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
pub const fn pcg32_xsh_rs(state: u32) -> u16 {
    let res = (((state >> 11) ^ state) >> ((state >> 30).wrapping_add(11)));
    res as u16
}

#[inline]
pub const fn pcg32_xsh_rr(state: u32) -> u16 {
    rotr_16((((state >> 10) ^ state) >> 12) as u16, state >> 28)
}

#[inline]
pub const fn rotr_16(value: u16, rot: u32) -> u16 {
    value.rotate_right(rot)
}
