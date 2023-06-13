use std::io::Read;

use bitvec::prelude::*;
use deku::ctx::{BitSize, ByteSize, Endian};
use deku::error::NeedSize;
use deku::prelude::*;

// TODO: test BitSlice<usize, Msb0>

/// "Reader" trait: read bits and construct type
pub trait NewDekuRead<'a, Ctx = ()> {
    /// Read bits and construct type
    /// * **input** - Input as bits
    /// * **ctx** - A context required by context-sensitive reading. A unit type `()` means no context
    /// needed.
    ///
    /// Returns the remaining bits after parsing in addition to Self.
    fn read_new(input: &'a BitSlice<u8, Msb0>, ctx: Ctx) -> Result<(usize, Self), DekuError>
    where
        Self: Sized;
}

/// "Reader" trait: implemented on DekuRead struct and enum containers. A `container` is a type which
/// doesn't need any context information.
pub trait NewDekuContainerRead<'a>: NewDekuRead<'a, ()> {
    /// Read bytes and construct type
    /// * **input** - Input given as data and bit offset
    ///
    /// Returns the remaining bytes and bit offset after parsing in addition to Self.
    fn from_bytes(input: (&'a [u8], usize)) -> Result<((usize, usize), Self), DekuError>
    where
        Self: Sized;
}

// specialize u8 for ByteSize
impl NewDekuRead<'_, ()> for u8 {
    fn read_new(input: &BitSlice<u8, Msb0>, _: ()) -> Result<(usize, Self), DekuError> {
        const MAX_TYPE_BITS: usize = BitSize::of::<u8>().0;
        if input.len() < MAX_TYPE_BITS {
            return Err(DekuError::Incomplete(NeedSize::new(MAX_TYPE_BITS)));
        }

        // PANIC: We already check that input.len() < bit_size above, so no panic will happen
        let value = input[..MAX_TYPE_BITS].load::<u8>();
        Ok((MAX_TYPE_BITS, value))
    }
}

#[derive(Debug, DekuRead)]
pub struct Test {
    a: u8,
    b: u8,
}

impl NewDekuRead<'_, ()> for Test {
    fn read_new(
        __deku_input_bits: &'_ ::deku::bitvec::BitSlice<u8, ::deku::bitvec::Msb0>,
        _: (),
    ) -> Result<(usize, Self), ::deku::DekuError> {
        use core::convert::TryFrom;
        let mut __deku_read = 0;
        let mut __deku_rest = __deku_input_bits;
        let mut total_read = 0;
        let __deku_a = {
            //assert!(__deku_read < __deku_input_bits.len());
            let (__deku_read, __deku_value) =
                <u8 as NewDekuRead<'_, _>>::read_new(&__deku_rest, ())?;
            let __deku_value: u8 = Result::<_, ::deku::DekuError>::Ok(__deku_value)?;
            __deku_rest = &__deku_rest[__deku_read..];
            total_read += __deku_read;
            __deku_value
        };
        let __deku_b = {
            //assert!(__deku_read < __deku_input_bits.len());
            let (__deku_read, __deku_value) =
                <u8 as NewDekuRead<'_, _>>::read_new(&__deku_rest, ())?;
            let __deku_value: u8 = Result::<_, ::deku::DekuError>::Ok(__deku_value)?;
            __deku_rest = &__deku_rest[__deku_read..];
            total_read += __deku_read;
            __deku_value
        };
        let __deku_value = Self {
            a: __deku_a,
            b: __deku_b,
        };
        Ok((total_read, __deku_value))
    }
}

impl Test {
    pub fn from_bytes_custom<T: Read>(reader: &mut T) -> Self {
        let (mut a, mut b, mut c) = ([0; 1], [0; 1], [0; 1]);
        let mut buf = [
            std::io::IoSliceMut::new(&mut a),
            std::io::IoSliceMut::new(&mut b),
            std::io::IoSliceMut::new(&mut c),
        ];
        let bytes_read = reader.read_vectored(&mut buf).unwrap();
        assert_eq!(bytes_read, 2);
        Self {
            a: u8::from_ne_bytes(a),
            b: u8::from_ne_bytes(b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bytes = [0x00, 0x03];
        let b = bytes.view_bits::<Msb0>();
        let (amt_read, test) = Test::read_new(b, ()).unwrap();
        assert_eq!(amt_read, (u8::BITS * 2) as usize);
    }
}
