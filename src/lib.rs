#![doc = include_str!("../README.md")]

pub mod format;
pub mod swizzle;

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
                    use $crate::{swizzle::{Swizzler, Deswizzler, Format}};

                    let swizzled_data = &mut include_bytes!(concat!("../testdata/", concat!($file_path, ".bin"))).to_vec();
                    let unswizzled_data = &mut include_bytes!(concat!("../testdata/", concat!($file_path, "-unswizzled.bin"))).to_vec();

                    let mut dest = vec![0u8; ($width * $height * $depth * $image_format.bpp()) / 8];

                    let result = match $operation {
                        Swizzle => {
                            if !$width.is_power_of_two() || !$height.is_power_of_two() {
                                dest = vec![0u8;(dest.len() as f32*1.25).floor() as usize];
                            }
                            <$swizzle_type as Swizzler>::swizzle(
                                unswizzled_data,
                                &mut dest,
                                ($width, $height, $depth),
                                $image_format,
                                $align_resolution,
                            )
                        }
                        Deswizzle => {
                            <$swizzle_type as Deswizzler>::deswizzle(
                                swizzled_data,
                                &mut dest,
                                ($width, $height, $depth),
                                $image_format,
                                $align_resolution,
                            )
                        }
                    };

                    assert!(
                        result.is_err(),
                        "{} operation failed with error: {:?}",
                        stringify!($operation),
                        result.err()
                    );

                    match $operation {
                        Swizzle => assert!(*swizzled_data == dest, "Swizzled data did not match reference"),
                        Deswizzle => assert!(*unswizzled_data == dest, "Deswizzled data did not match reference")
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
        900_usize,
        1080_usize,
        1,
        BC7,
        false
    );
    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Swizzle,
        "ps4-bc7-900x1080",
        900_usize,
        1080_usize,
        1,
        BC7,
        false
    );

    // PS4 171 x 171 RGBA8

    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Deswizzle,
        "ps4-rgba8-171x171",
        171_usize,
        171_usize,
        1,
        Format8_8_8_8,
        true
    );
    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Swizzle,
        "ps4-rgba8-171x171",
        171_usize,
        171_usize,
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
        512_usize,
        512_usize,
        1,
        BC5,
        false
    );
    test_impl!(
        ps4,
        crate::swizzle::ps::Ps4,
        Swizzle,
        "ps4-bc5-512x512",
        512_usize,
        512_usize,
        1,
        BC5,
        false
    );

    // PS3 64 x 64 RGBA8

    test_impl!(
        ps3,
        crate::swizzle::ps::Ps3,
        Deswizzle,
        "ps3-rgba8-64x64",
        64_usize,
        64_usize,
        1,
        A8R8G8B8,
        true
    );
    test_impl!(
        ps3,
        crate::swizzle::ps::Ps3,
        Swizzle,
        "ps3-rgba8-64x64",
        64_usize,
        64_usize,
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
        128_usize,
        128_usize,
        1,
        COMPRESSED_DXT45,
        true
    );
    test_impl!(
        ps3,
        crate::swizzle::ps::Ps3,
        Swizzle,
        "ps3-bc3-128x128",
        128_usize,
        128_usize,
        1,
        COMPRESSED_DXT45,
        true
    );
}
