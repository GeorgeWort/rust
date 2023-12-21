use super::{InlineAsmArch, InlineAsmType};
use crate::spec::{RelocModel, Target};
use rustc_data_structures::fx::FxIndexSet;
use rustc_macros::HashStable_Generic;
use rustc_span::Symbol;
use std::fmt;

def_reg_class! {
    AArch64 AArch64InlineAsmRegClass {
        reg,
        vreg,
        vreg_low16,
        zreg,
        zreg_low16,
        zreg_low8,
        preg,
        preg_low8,
        ffr_reg,
    }
}

impl AArch64InlineAsmRegClass {
    pub fn valid_modifiers(self, _arch: super::InlineAsmArch) -> &'static [char] {
        match self {
            Self::reg => &['w', 'x'],
            Self::vreg | Self::vreg_low16 => &['b', 'h', 's', 'd', 'q', 'v'],
            Self::zreg | Self::zreg_low16 | Self::zreg_low8 => &['b', 'h', 's', 'd', 'q', 'z'],
            Self::preg | Self::preg_low8 => &['p'],
            Self::ffr_reg => &[],
        }
    }

    pub fn suggest_class(self, _arch: InlineAsmArch, ty: InlineAsmType) -> Option<Self> {
        if ty.size().bytes() == 0 {
            match self {
                Self::vreg => Some(Self::zreg),
                Self::vreg_low16 => Some(Self::zreg_low16),
                _ => None,
            }
        } else {
            match self {
                Self::zreg | Self::zreg_low8 => Some(Self::vreg),
                Self::zreg_low16 => Some(Self::vreg_low16),
                _ => None,
            }
        }
    }

    pub fn suggest_modifier(
        self,
        _arch: InlineAsmArch,
        ty: InlineAsmType,
    ) -> Option<(char, &'static str)> {
        match self {
            Self::reg => match ty.size().bits() {
                64 => None,
                _ => Some(('w', "w0")),
            },
            Self::vreg | Self::vreg_low16 | Self::zreg | Self::zreg_low16 | Self::zreg_low8 => {
                match ty.size().bits() {
                    8 => Some(('b', "b0")),
                    16 => Some(('h', "h0")),
                    32 => Some(('s', "s0")),
                    64 => Some(('d', "d0")),
                    128 => Some(('q', "q0")),
                    _ => None,
                }
            }

            Self::preg | Self::preg_low8 => None,
            Self::ffr_reg => None,
        }
    }

    pub fn default_modifier(self, _arch: InlineAsmArch) -> Option<(char, &'static str)> {
        match self {
            Self::reg => Some(('x', "x0")),
            Self::vreg | Self::vreg_low16 => Some(('v', "v0")),
            Self::zreg | Self::zreg_low16 | Self::zreg_low8 => Some(('z', "z0")),
            Self::preg | Self::preg_low8 => Some(('p', "p0")),
            Self::ffr_reg => None,
        }
    }

    pub fn supported_types(
        self,
        _arch: InlineAsmArch,
    ) -> &'static [(InlineAsmType, Option<Symbol>)] {
        match self {
            Self::reg => types! { _: I8, I16, I32, I64, F32, F64; },
            Self::vreg | Self::vreg_low16 => types! {
                neon: I8, I16, I32, I64, F32, F64,
                    VecI8(8), VecI16(4), VecI32(2), VecI64(1), VecF32(2), VecF64(1),
                    VecI8(16), VecI16(8), VecI32(4), VecI64(2), VecF32(4), VecF64(2);
            },
            Self::zreg => types! {
                sve: I16, I32, I64, F32, F64,
                    VecI8(0), VecI16(0), VecI32(0), VecI64(0), VecF32(0), VecF64(0);
            },
            Self::zreg_low16 | Self::zreg_low8 => types! {
                sve: VecI8(0), VecI16(0), VecI32(0), VecI64(0), VecF32(0), VecF64(0);
            },
            Self::preg | Self::preg_low8 => types! { sve: VecBool(0); },
            Self::ffr_reg => &[],
        }
    }
}

pub fn target_reserves_x18(target: &Target) -> bool {
    target.os == "android" || target.os == "fuchsia" || target.is_like_osx || target.is_like_windows
}

fn reserved_x18(
    _arch: InlineAsmArch,
    _reloc_model: RelocModel,
    _target_features: &FxIndexSet<Symbol>,
    target: &Target,
    _is_clobber: bool,
) -> Result<(), &'static str> {
    if target_reserves_x18(target) {
        Err("x18 is a reserved register on this target")
    } else {
        Ok(())
    }
}

def_regs! {
    AArch64 AArch64InlineAsmReg AArch64InlineAsmRegClass {
        x0: reg = ["x0", "w0"],
        x1: reg = ["x1", "w1"],
        x2: reg = ["x2", "w2"],
        x3: reg = ["x3", "w3"],
        x4: reg = ["x4", "w4"],
        x5: reg = ["x5", "w5"],
        x6: reg = ["x6", "w6"],
        x7: reg = ["x7", "w7"],
        x8: reg = ["x8", "w8"],
        x9: reg = ["x9", "w9"],
        x10: reg = ["x10", "w10"],
        x11: reg = ["x11", "w11"],
        x12: reg = ["x12", "w12"],
        x13: reg = ["x13", "w13"],
        x14: reg = ["x14", "w14"],
        x15: reg = ["x15", "w15"],
        x16: reg = ["x16", "w16"],
        x17: reg = ["x17", "w17"],
        x18: reg = ["x18", "w18"] % reserved_x18,
        x20: reg = ["x20", "w20"],
        x21: reg = ["x21", "w21"],
        x22: reg = ["x22", "w22"],
        x23: reg = ["x23", "w23"],
        x24: reg = ["x24", "w24"],
        x25: reg = ["x25", "w25"],
        x26: reg = ["x26", "w26"],
        x27: reg = ["x27", "w27"],
        x28: reg = ["x28", "w28"],
        x30: reg = ["x30", "w30", "lr", "wlr"],
        v0: vreg, vreg_low16 = ["v0", "b0", "h0", "s0", "d0", "q0"],
        v1: vreg, vreg_low16 = ["v1", "b1", "h1", "s1", "d1", "q1"],
        v2: vreg, vreg_low16 = ["v2", "b2", "h2", "s2", "d2", "q2"],
        v3: vreg, vreg_low16 = ["v3", "b3", "h3", "s3", "d3", "q3"],
        v4: vreg, vreg_low16 = ["v4", "b4", "h4", "s4", "d4", "q4"],
        v5: vreg, vreg_low16 = ["v5", "b5", "h5", "s5", "d5", "q5"],
        v6: vreg, vreg_low16 = ["v6", "b6", "h6", "s6", "d6", "q6"],
        v7: vreg, vreg_low16 = ["v7", "b7", "h7", "s7", "d7", "q7"],
        v8: vreg, vreg_low16 = ["v8", "b8", "h8", "s8", "d8", "q8"],
        v9: vreg, vreg_low16 = ["v9", "b9", "h9", "s9", "d9", "q9"],
        v10: vreg, vreg_low16 = ["v10", "b10", "h10", "s10", "d10", "q10"],
        v11: vreg, vreg_low16 = ["v11", "b11", "h11", "s11", "d11", "q11"],
        v12: vreg, vreg_low16 = ["v12", "b12", "h12", "s12", "d12", "q12"],
        v13: vreg, vreg_low16 = ["v13", "b13", "h13", "s13", "d13", "q13"],
        v14: vreg, vreg_low16 = ["v14", "b14", "h14", "s14", "d14", "q14"],
        v15: vreg, vreg_low16 = ["v15", "b15", "h15", "s15", "d15", "q15"],
        v16: vreg = ["v16", "b16", "h16", "s16", "d16", "q16"],
        v17: vreg = ["v17", "b17", "h17", "s17", "d17", "q17"],
        v18: vreg = ["v18", "b18", "h18", "s18", "d18", "q18"],
        v19: vreg = ["v19", "b19", "h19", "s19", "d19", "q19"],
        v20: vreg = ["v20", "b20", "h20", "s20", "d20", "q20"],
        v21: vreg = ["v21", "b21", "h21", "s21", "d21", "q21"],
        v22: vreg = ["v22", "b22", "h22", "s22", "d22", "q22"],
        v23: vreg = ["v23", "b23", "h23", "s23", "d23", "q23"],
        v24: vreg = ["v24", "b24", "h24", "s24", "d24", "q24"],
        v25: vreg = ["v25", "b25", "h25", "s25", "d25", "q25"],
        v26: vreg = ["v26", "b26", "h26", "s26", "d26", "q26"],
        v27: vreg = ["v27", "b27", "h27", "s27", "d27", "q27"],
        v28: vreg = ["v28", "b28", "h28", "s28", "d28", "q28"],
        v29: vreg = ["v29", "b29", "h29", "s29", "d29", "q29"],
        v30: vreg = ["v30", "b30", "h30", "s30", "d30", "q30"],
        v31: vreg = ["v31", "b31", "h31", "s31", "d31", "q31"],
        z0: zreg, zreg_low16, zreg_low8 = ["z0"],
        z1: zreg, zreg_low16, zreg_low8 = ["z1"],
        z2: zreg, zreg_low16, zreg_low8 = ["z2"],
        z3: zreg, zreg_low16, zreg_low8 = ["z3"],
        z4: zreg, zreg_low16, zreg_low8 = ["z4"],
        z5: zreg, zreg_low16, zreg_low8 = ["z5"],
        z6: zreg, zreg_low16, zreg_low8 = ["z6"],
        z7: zreg, zreg_low16, zreg_low8 = ["z7"],
        z8: zreg, zreg_low16 = ["z8"],
        z9: zreg, zreg_low16 = ["z9"],
        z10: zreg, zreg_low16 = ["z10"],
        z11: zreg, zreg_low16 = ["z11"],
        z12: zreg, zreg_low16 = ["z12"],
        z13: zreg, zreg_low16 = ["z13"],
        z14: zreg, zreg_low16 = ["z14"],
        z15: zreg, zreg_low16 = ["z15"],
        z16: zreg = ["z16"],
        z17: zreg = ["z17"],
        z18: zreg = ["z18"],
        z19: zreg = ["z19"],
        z20: zreg = ["z20"],
        z21: zreg = ["z21"],
        z22: zreg = ["z22"],
        z23: zreg = ["z23"],
        z24: zreg = ["z24"],
        z25: zreg = ["z25"],
        z26: zreg = ["z26"],
        z27: zreg = ["z27"],
        z28: zreg = ["z28"],
        z29: zreg = ["z29"],
        z30: zreg = ["z30"],
        z31: zreg = ["z31"],
        p0: preg, preg_low8 = ["p0"],
        p1: preg, preg_low8 = ["p1"],
        p2: preg, preg_low8 = ["p2"],
        p3: preg, preg_low8 = ["p3"],
        p4: preg, preg_low8 = ["p4"],
        p5: preg, preg_low8 = ["p5"],
        p6: preg, preg_low8 = ["p6"],
        p7: preg, preg_low8 = ["p7"],
        p8: preg = ["p8"],
        p9: preg = ["p9"],
        p10: preg = ["p10"],
        p11: preg = ["p11"],
        p12: preg = ["p12"],
        p13: preg = ["p13"],
        p14: preg = ["p14"],
        p15: preg = ["p15"],
        ffr: ffr_reg = ["ffr"],
        #error = ["x19", "w19"] =>
            "x19 is used internally by LLVM and cannot be used as an operand for inline asm",
        #error = ["x29", "w29", "fp", "wfp"] =>
            "the frame pointer cannot be used as an operand for inline asm",
        #error = ["sp", "wsp"] =>
            "the stack pointer cannot be used as an operand for inline asm",
        #error = ["xzr", "wzr"] =>
            "the zero register cannot be used as an operand for inline asm",
    }
}

impl AArch64InlineAsmReg {
    fn index(self, min: u32, max: u32) -> Option<u32> {
        if ((self as u32) >= min) && ((self as u32) <= max) {
            Some(self as u32 - min)
        } else {
            None
        }
    }
    fn x_index(self) -> Option<u32> {
        self.index(Self::x0 as u32, Self::x30 as u32)
    }
    fn v_index(self) -> Option<u32> {
        self.index(Self::v0 as u32, Self::v31 as u32)
    }
    fn z_index(self) -> Option<u32> {
        self.index(Self::z0 as u32, Self::z31 as u32)
    }
    fn p_index(self) -> Option<u32> {
        self.index(Self::p0 as u32, Self::p15 as u32)
    }

    pub fn emit(
        self,
        out: &mut dyn fmt::Write,
        _arch: InlineAsmArch,
        modifier: Option<char>,
    ) -> fmt::Result {
        let (prefix, index) = if let Some(x) = self.x_index() {
            (modifier.unwrap_or('x'), x)
        } else if let Some(v) = self.v_index() {
            (modifier.unwrap_or('v'), v)
        } else if let Some(z) = self.z_index() {
            (modifier.unwrap_or('z'), z)
        } else {
            let p = self.p_index();
            assert!(p.is_some());
            (modifier.unwrap_or('p'), p.unwrap())
        };
        assert!(index < 32);
        write!(out, "{prefix}{index}")
    }

    pub fn overlapping_regs(self, mut cb: impl FnMut(AArch64InlineAsmReg)) {
        cb(self);
        if (self as u32) >= Self::v0 as u32 && (self as u32) < Self::z0 as u32 {
            let index = (self as usize) - Self::v0 as usize;
            cb(AArch64InlineAsmReg::parse(format!("z{}", index).as_str()).unwrap());
        } else if (self as u32) >= Self::z0 as u32 && (self as u32) < Self::p0 as u32 {
            let index = (self as usize) - Self::z0 as usize;
            cb(AArch64InlineAsmReg::parse(format!("v{}", index).as_str()).unwrap());
        };
    }
}
