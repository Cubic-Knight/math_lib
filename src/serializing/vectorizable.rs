use super::{
    Vectorizable,
    RpnBlock
};
use crate::compiling::{
    Placeholder, Reference
};

impl Vectorizable for usize {
    type BinaryForm = [u8; 5];
    fn to_binary_in_vec(self) -> Self::BinaryForm {
        let mut res = [0; 5];
        for (i, data) in (self as u32).to_le_bytes().into_iter().enumerate() {
            res[i+1] = data
        };
        res
    }
    fn from_binary_in_vec(source: Self::BinaryForm) -> Option<Self> {
        let mut bytes = [0; 4];
        for (i, data) in source.into_iter().skip(1).enumerate() {
            bytes[i] = data
        };
        Some(u32::from_le_bytes(bytes) as usize)
    }
    const TERMINATOR: Self::BinaryForm = [0xfe; 5];
    const TERMINATOR2: Self::BinaryForm = [0xff; 5];
}

impl Vectorizable for Placeholder {
    type BinaryForm = [u8; 5];
    fn to_binary_in_vec(self) -> Self::BinaryForm {
        let mut res = [0; 5];
        let num = match self {
            Placeholder::LiteralChar(c) => {res[0] = 0x00; c as u32},
            Placeholder::WellFormedFormula(id) => {res[0] = 0x01; id as u32},
            Placeholder::Object(id) => {res[0] = 0x02; id as u32},
            Placeholder::Repetition => {res[0] = 0x03; 0}
        };
        for (i, data) in (num as u32).to_le_bytes().into_iter().enumerate() {
            res[i+1] = data
        };
        res
    }
    fn from_binary_in_vec(source: Self::BinaryForm) -> Option<Self> {
        let first_byte = source[0];
        let mut bytes = [0; 4];
        for (i, data) in source.into_iter().skip(1).enumerate() {
            bytes[i] = data
        };
        let res = match first_byte {
            0x00 => Placeholder::LiteralChar(char::from_u32(u32::from_le_bytes(bytes))?),
            0x01 => Placeholder::WellFormedFormula(u32::from_le_bytes(bytes) as usize),
            0x02 => Placeholder::Object(u32::from_le_bytes(bytes) as usize),
            0x03 => Placeholder::Repetition,
            _ => return None
        };
        Some(res)
    }
    const TERMINATOR: Self::BinaryForm = [0xfe; 5];
    const TERMINATOR2: Self::BinaryForm = [0xff; 5];
}

impl Vectorizable for RpnBlock {
    type BinaryForm = [u8; 5];
    fn to_binary_in_vec(self) -> Self::BinaryForm {
        let mut res = [0; 5];
        let num = match self {
            RpnBlock::WffAtomic(id) => {res[0] = 0x00; id as u32},
            RpnBlock::WffComposite(id) => {res[0] = 0x01; id as u32},
            RpnBlock::ObjectAtomic(id) => {res[0] = 0x02; id as u32},
            RpnBlock::ObjectComposite(id) => {res[0] = 0x03; id as u32}
        };
        for (i, data) in (num as u32).to_le_bytes().into_iter().enumerate() {
            res[i+1] = data
        };
        res
    }
    fn from_binary_in_vec(source: Self::BinaryForm) -> Option<Self> {
        let first_byte = source[0];
        let mut bytes = [0; 4];
        for (i, data) in source.into_iter().skip(1).enumerate() {
            bytes[i] = data
        };
        let res = match first_byte {
            0x00 => RpnBlock::WffAtomic(u32::from_le_bytes(bytes) as usize),
            0x01 => RpnBlock::WffComposite(u32::from_le_bytes(bytes) as usize),
            0x02 => RpnBlock::ObjectAtomic(u32::from_le_bytes(bytes) as usize),
            0x03 => RpnBlock::ObjectComposite(u32::from_le_bytes(bytes) as usize),
            _ => return None
        };
        Some(res)
    }
    const TERMINATOR: Self::BinaryForm = [0xfe; 5];
    const TERMINATOR2: Self::BinaryForm = [0xff; 5];
}

impl Vectorizable for Reference {
    type BinaryForm = [u8; 9];
    fn to_binary_in_vec(self) -> Self::BinaryForm {
        let mut res = [0; 9];
        let (id, sub_id) = match self {
            Reference::HypothesisReference(id) => {res[0] = 0x00; (id as u32, 0)},
            Reference::DefinitionReference(id) => {res[0] = 0x01; (id as u32, 0)},
            Reference::AxiomReference(id, sub_id) => {res[0] = 0x02; (id as u32, sub_id as u32)},
            Reference::TheoremReference(id, sub_id) => {res[0] = 0x03; (id as u32, sub_id as u32)}
        };
        for (i, data) in (id as u32).to_le_bytes().into_iter().enumerate() {
            res[i+1] = data
        };
        for (i, data) in (sub_id as u32).to_le_bytes().into_iter().enumerate() {
            res[i+5] = data
        };
        res
    }
    fn from_binary_in_vec(source: Self::BinaryForm) -> Option<Self> {
        let first_byte = source[0];
        let mut bytes = [0; 8];
        for (i, data) in source.into_iter().skip(1).enumerate() {
            bytes[i] = data
        };
        let num = u64::from_le_bytes(bytes);
        let id = (num & 0xFF_FF_FF_FF) as usize;
        let sub_id = (num >> 32) as usize;
        let res = match first_byte {
            0x00 => Reference::HypothesisReference(id),
            0x01 => Reference::DefinitionReference(id),
            0x02 => Reference::AxiomReference(id, sub_id),
            0x03 => Reference::TheoremReference(id, sub_id),
            _ => return None
        };
        Some(res)
    }
    const TERMINATOR: Self::BinaryForm = [0xfe; 9];
    const TERMINATOR2: Self::BinaryForm = [0xff; 9];
}
