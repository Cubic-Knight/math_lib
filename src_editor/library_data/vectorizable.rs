use super::{
    Placeholder, Reference,
    RpnBlock
};

pub trait Vectorizable where Self: Sized {
    type BinaryForm;
    fn from_binary_in_vec(source: Self::BinaryForm) -> Option<Self>;
    const TERMINATOR: Self::BinaryForm;  // Terminator to use when in a vector
    const TERMINATOR2: Self::BinaryForm;  // Terminator to use when in a vector of vectors
}
impl Vectorizable for usize {
    type BinaryForm = [u8; 5];
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
