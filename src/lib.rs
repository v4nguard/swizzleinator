#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

pub mod format;
pub mod swizzle;

#[derive(Debug)]
pub enum SwizzleError {
    FormatOutOfRange(u32),
}

impl Error for SwizzleError {}

impl fmt::Display for SwizzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SwizzleError::FormatOutOfRange(e) => write!(f, "format is out of range ({e})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::format::{GcmSurfaceFormat::*, GcnSurfaceFormat::*};
    use crate::tests::SwizzleState::*;

    #[derive(Copy, Clone, Debug)]
    enum SwizzleState {
        Deswizzle,
        Swizzle,
    }

    macro_rules! test_impl {
        (
            $test_name:ident,
            $swizzle_type:path,
            $operation:ident,
            $file_path:expr,
            $width:expr,
            $height:expr,
            $depth:expr,
            $image_format:expr,
            $align_resolution:expr
        ) => {
            paste::paste! {
                #[test]
                #[allow(non_snake_case)]
                fn [<$test_name _ $operation _ $width _ $height _ $depth _ $image_format>]() {
                    use $crate::{swizzle::Swizzler, swizzle::Deswizzler};

                    const SWIZZLED_DATA: &[u8] = include_bytes!(concat!("../testdata/", concat!($file_path, ".bin")));
                    const UNSWIZZLED_DATA: &[u8] = include_bytes!(concat!("../testdata/", concat!($file_path, "-unswizzled.bin")));

                    let result = match $operation {
                        Swizzle => {
                            <$swizzle_type as Swizzler>::swizzle(
                                &UNSWIZZLED_DATA,
                                $width,
                                $height,
                                $depth,
                                $image_format,
                                $align_resolution,
                            )
                        }
                        Deswizzle => {
                            <$swizzle_type as Deswizzler>::deswizzle(
                                &SWIZZLED_DATA,
                                $width,
                                $height,
                                $depth,
                                $image_format,
                                $align_resolution,
                            )
                        }
                    };

                    assert!(
                        result.is_ok(),
                        "{} operation failed with error: {:?}",
                        stringify!($operation),
                        result.err()
                    );

                    match $operation {
                        Swizzle => assert!(SWIZZLED_DATA == result.unwrap().as_slice(), "Swizzled data did not match reference"),
                        Deswizzle => assert!(UNSWIZZLED_DATA == result.unwrap().as_slice(), "Deswizzled data did not match reference")
                    }
                }
            }
        };
    }

    // PS4 900 x 1080 BC7

    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Deswizzle,
        "ps4-bc7-900x1080",
        900,
        1080,
        1,
        BC7,
        false
    );
    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Swizzle,
        "ps4-bc7-900x1080",
        900,
        1080,
        1,
        BC7,
        false
    );

    // PS4 144 x 144 RGBA8

    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Deswizzle,
        "ps4-rgba8-114x114",
        114,
        114,
        1,
        Format8_8_8_8,
        true
    );
    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Swizzle,
        "ps4-rgba8-114x114",
        114,
        114,
        1,
        Format8_8_8_8,
        true
    );

    // PS4 512 x 512 BC5

    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Deswizzle,
        "ps4-bc5-512x512",
        512,
        512,
        1,
        BC5,
        true
    );
    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Swizzle,
        "ps4-bc5-512x512",
        512,
        512,
        1,
        BC5,
        true
    );

    // PS3 64 x 64 RGBA8

    test_impl!(
        ps3,
        crate::swizzle::ps::Ps3,
        Deswizzle,
        "ps3-rgba8-64x64",
        64,
        64,
        1,
        A8R8G8B8,
        true
    );
    test_impl!(
        ps3,
        crate::swizzle::ps::Ps3,
        Swizzle,
        "ps3-rgba8-64x64",
        64,
        64,
        1,
        A8R8G8B8,
        true
    );

    // PS3 128 x 128 BC3

    test_impl!(
        ps3,
        crate::swizzle::ps::Ps3,
        Deswizzle,
        "ps3-bc3-128x128",
        128,
        128,
        1,
        COMPRESSED_DXT45,
        true
    );
    test_impl!(
        ps3,
        crate::swizzle::ps::Ps3,
        Swizzle,
        "ps3-bc3-128x128",
        128,
        128,
        1,
        COMPRESSED_DXT45,
        true
    );
}
