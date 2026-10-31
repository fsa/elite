//! Арифметико-логические операции, флаги и сдвиги Z80.

use super::*;

pub(super) fn parity(v: u8) -> bool {
    v.count_ones() % 2 == 0
}

impl Z80 {
    // flags: S, Z, X/Y from result; reset H,N,C
    #[inline]
    fn sz53(&mut self, v: u8) {
        self.f = (self.f & (CF)) | if v == 0 { ZF } else { 0 } | (v & (SF | XF | YF));
        self.f &= !(HF | NF);
    }
    #[inline]
    fn sz53p(&mut self, v: u8) {
        self.f = (self.f & CF)
            | if v == 0 { ZF } else { 0 }
            | (v & (SF | XF | YF))
            | if parity(v) { PF } else { 0 };
        self.f &= !(HF | NF);
    }

    #[inline]
    pub(crate) fn inc8(&mut self, v: u8) -> u8 {
        let r = v.wrapping_add(1);
        let c = self.f & CF;
        self.f = c
            | if r == 0 { ZF } else { 0 }
            | (r & (SF | XF | YF))
            | if (v & 0x0F) == 0x0F { HF } else { 0 }
            | if v == 0x7F { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn dec8(&mut self, v: u8) -> u8 {
        let r = v.wrapping_sub(1);
        let c = self.f & CF;
        self.f = c
            | NF
            | if r == 0 { ZF } else { 0 }
            | (r & (SF | XF | YF))
            | if (v & 0x0F) == 0x00 { HF } else { 0 }
            | if v == 0x80 { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn add8(&mut self, a: u8, v: u8, carry: u8) -> u8 {
        let r16 = a as u16 + v as u16 + carry as u16;
        let r = r16 as u8;
        self.f = if r == 0 { ZF } else { 0 }
            | (r & (SF | XF | YF))
            | if (a & 0x0F) + (v & 0x0F) + carry > 0x0F {
                HF
            } else {
                0
            }
            | if r16 > 0xFF { CF } else { 0 }
            | if !((a ^ v) & 0x80 != 0) && ((a ^ r) & 0x80 != 0) {
                PF
            } else {
                0
            };
        r
    }
    #[inline]
    pub(crate) fn sub8(&mut self, a: u8, v: u8, carry: u8) -> u8 {
        let r = a.wrapping_sub(v).wrapping_sub(carry);
        self.f = NF
            | if r == 0 { ZF } else { 0 }
            | (r & (SF | XF | YF))
            | if (a & 0x0F) < (v & 0x0F) + carry {
                HF
            } else {
                0
            }
            | if (a as u16) < (v as u16) + carry as u16 {
                CF
            } else {
                0
            }
            | if ((a ^ v) & 0x80 != 0) && ((a ^ r) & 0x80 != 0) {
                PF
            } else {
                0
            };
        r
    }
    #[inline]
    pub(crate) fn and8(&mut self, a: u8, v: u8) -> u8 {
        let r = a & v;
        self.f = HF
            | (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn or8(&mut self, a: u8, v: u8) -> u8 {
        let r = a | v;
        self.f =
            (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn xor8(&mut self, a: u8, v: u8) -> u8 {
        let r = a ^ v;
        self.f =
            (r & (SF | XF | YF)) | if r == 0 { ZF } else { 0 } | if parity(r) { PF } else { 0 };
        r
    }
    /// `RLCA` — циклический сдвиг `A` влево. Флаги `S/Z/PV` сохраняются,
    /// `H`/`N` сбрасываются, `C` — выдвинутый бит, `X/Y` — из `A`.
    #[inline]
    pub(crate) fn rlca(&mut self) {
        let c = self.a >> 7;
        self.a = (self.a << 1) | c;
        self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { CF } else { 0 };
    }
    /// `RRA` — циклический сдвиг `A` вправо через флаг переноса.
    #[inline]
    pub(crate) fn rra(&mut self) {
        let c = self.a & 1;
        self.a = (self.a >> 1) | if self.f & CF != 0 { 0x80 } else { 0 };
        self.f = (self.f & (SF | ZF | PF)) | (self.a & (XF | YF)) | if c != 0 { CF } else { 0 };
    }
    #[inline]
    pub(crate) fn add16(&mut self, a: u16, b: u16) -> u16 {
        let r32 = a as u32 + b as u32;
        let r = r32 as u16;
        self.f = (self.f & (SF | ZF | PF))
            | ((r >> 8) as u8 & (XF | YF))
            | if ((a & 0x0FFF) + (b & 0x0FFF)) > 0x0FFF {
                HF
            } else {
                0
            }
            | if r32 > 0xFFFF { CF } else { 0 };
        r
    }
    #[inline]
    pub(super) fn adc16(&mut self, a: u16, b: u16) -> u16 {
        let carry = if self.f & CF != 0 { 1u32 } else { 0 };
        let r32 = a as u32 + b as u32 + carry;
        let r = r32 as u16;
        self.f = if r == 0 { ZF } else { 0 }
            | (r32 as u8 & (XF | YF))
            | ((r >> 8) as u8 & SF)
            | if ((a & 0x0FFF) + (b & 0x0FFF) + carry as u16) > 0x0FFF {
                HF
            } else {
                0
            }
            | if r32 > 0xFFFF { CF } else { 0 }
            | if !((a ^ b) & 0x8000 != 0) && ((a ^ r) & 0x8000 != 0) {
                PF
            } else {
                0
            };
        r
    }
    #[inline]
    pub(crate) fn sbc16(&mut self, a: u16, b: u16) -> u16 {
        let carry = if self.f & CF != 0 { 1u32 } else { 0 };
        let r32 = (a as u32).wrapping_sub(b as u32).wrapping_sub(carry);
        let r = r32 as u16;
        self.f = NF
            | if r == 0 { ZF } else { 0 }
            | (r32 as u8 & (XF | YF))
            | ((r >> 8) as u8 & SF)
            | if (a & 0x0FFF) < (b & 0x0FFF) + carry as u16 {
                HF
            } else {
                0
            }
            | if (a as u32) < (b as u32) + carry {
                CF
            } else {
                0
            }
            | if ((a ^ b) & 0x8000 != 0) && ((a ^ r) & 0x8000 != 0) {
                PF
            } else {
                0
            };
        r
    }
    #[inline]
    pub(super) fn daa(&mut self) -> u8 {
        let mut correction = 0u8;
        let mut carry = self.f & CF != 0;
        if self.f & HF != 0 || (self.a & 0x0F) > 9 {
            correction |= 0x06;
        }
        if carry || self.a > 0x99 {
            correction |= 0x60;
            carry = true;
        }
        let old_a = self.a;
        let a = if self.f & NF != 0 {
            old_a.wrapping_sub(correction)
        } else {
            old_a.wrapping_add(correction)
        };
        self.a = a;
        let h = if self.f & NF != 0 {
            (old_a & 0x0F) < (correction & 0x0F)
        } else {
            ((old_a & 0x0F) + (correction & 0x0F)) > 0x0F
        };
        self.f = (self.f & NF)
            | if a == 0 { ZF } else { 0 }
            | (a & (SF | XF | YF))
            | if h { HF } else { 0 }
            | if carry { CF } else { 0 }
            | if parity(a) { PF } else { 0 };
        a
    }
    #[inline]
    pub(crate) fn rlc(&mut self, v: u8) -> u8 {
        let c = v >> 7;
        let r = (v << 1) | c;
        self.f = (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if c != 0 { CF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(super) fn rrc(&mut self, v: u8) -> u8 {
        let c = v & 1;
        let r = (v >> 1) | (c << 7);
        self.f = (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if c != 0 { CF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn rl(&mut self, v: u8) -> u8 {
        let c = v >> 7;
        let r = (v << 1) | if self.f & CF != 0 { 1 } else { 0 };
        self.f = (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if c != 0 { CF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn rr(&mut self, v: u8) -> u8 {
        let c = v & 1;
        let r = (v >> 1) | if self.f & CF != 0 { 0x80 } else { 0 };
        self.f = (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if c != 0 { CF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn sla(&mut self, v: u8) -> u8 {
        let c = v >> 7;
        let r = v << 1;
        self.f = (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if c != 0 { CF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(super) fn sra(&mut self, v: u8) -> u8 {
        let c = v & 1;
        let r = (v >> 1) | (v & 0x80);
        self.f = (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if c != 0 { CF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(super) fn sll(&mut self, v: u8) -> u8 {
        let c = v >> 7;
        let r = (v << 1) | 1;
        self.f = (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if c != 0 { CF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn srl(&mut self, v: u8) -> u8 {
        let c = v & 1;
        let r = v >> 1;
        self.f = (r & (SF | XF | YF))
            | if r == 0 { ZF } else { 0 }
            | if c != 0 { CF } else { 0 }
            | if parity(r) { PF } else { 0 };
        r
    }
    #[inline]
    pub(crate) fn bit(&mut self, n: u8, v: u8) {
        let bit = v & (1 << (n & 7));
        self.f =
            (self.f & CF) | HF | if bit == 0 { ZF | PF } else { 0 } | (bit & SF) | (v & (XF | YF));
    }
}
