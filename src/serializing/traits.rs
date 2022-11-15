use crate::compiling::Syntax;

pub trait BinaryConvert<const N: usize> where Self: Sized {
    fn to_binary(self) -> Vec<u8>;
    fn from_binary<I: Iterator<Item = u8>>(source: &mut I) -> Option<Self>;
    fn from_binary_syntaxes<I: Iterator<Item = u8>>(source: &mut I, syntaxes: &Vec<Syntax>) -> Option<Self>;
}

pub trait Vectorizable where Self: Sized {
    type BinaryForm;
    fn to_binary_in_vec(self) -> Self::BinaryForm;
    fn from_binary_in_vec(source: Self::BinaryForm) -> Option<Self>;
    const TERMINATOR: Self::BinaryForm;  // Terminator to use when in a vector
    const TERMINATOR2: Self::BinaryForm;  // Terminator to use when in a vector of vectors
}
