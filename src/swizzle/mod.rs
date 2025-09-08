pub mod ps;
pub mod xbox;

use core::{error::Error, fmt};

#[derive(Debug, Clone, Copy)]
pub enum TextureSlice {
    Source,
    Dest,
}

#[derive(Debug, Clone, Copy)]
pub enum SwizzleError {
    FormatOutOfRange(u32),
    OutOfBounds(TextureSlice),
}

impl Error for SwizzleError {}

impl fmt::Display for SwizzleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SwizzleError::OutOfBounds(s) => write!(f, "slice {s:?} out of bounds"),
            SwizzleError::FormatOutOfRange(e) => write!(f, "format is out of range ({e})"),
        }
    }
}

/// This trait defines the function used to swizzle/tile image data
/// * `source` - Source image data.
/// * `dest` - Destination slice. When swizzling an image that has dimensions that are not a power of two,
///   it is recommended to make this larger than the size of the unswizzled texture, or else data could be lost.
/// * `dimensions` - Dimensions of the image: `(width, height, depth)`.
/// * `format` - Expected image format.
/// * `align_resolution` - Align the resolution of the image to the next power of two.
pub trait Swizzler {
    fn swizzle<T: Format>(
        source: &mut [u8],
        dest: &mut [u8],
        dimensions: (usize, usize, usize),
        format: T,
        align_resolution: bool,
    ) -> Result<(), SwizzleError>;
}

/// The trait that defines an interface to deswizzle/detile image data
/// * `source` - Source image data.
/// * `dest` - Destination slice, usually with the size `(width * height * depth * format.bpp()) / 8`.
/// * `dimensions` - Dimensions of the image: `(width, height, depth)`.
/// * `format` - Expected image format.
/// * `align_resolution` - Align the resolution of the image to the next power of two.
pub trait Deswizzler {
    fn deswizzle<T: Format>(
        source: &mut [u8],
        dest: &mut [u8],
        dimensions: (usize, usize, usize),
        format: T,
        align_resolution: bool,
    ) -> Result<(), SwizzleError>;
}

/// A trait that defines a given texture format
pub trait Format {
    fn bpp(&self) -> usize;

    fn block_size(&self) -> usize;

    fn pixel_block_size(&self) -> usize;

    fn is_compressed(&self) -> bool;

    fn x360_swap(&self) -> bool;
}

pub fn morton(t: usize, x: usize, y: usize) -> usize {
    let mut bit_position_x = 1;
    let mut bit_position_y = 1;
    let mut morton_code = t;
    let mut mask_x = x;
    let mut mask_y = y;
    let mut result_x = 0;
    let mut result_y = 0;

    while mask_x > 1 || mask_y > 1 {
        if mask_x > 1 {
            result_x += bit_position_x * (morton_code & 1);
            morton_code >>= 1;
            bit_position_x <<= 1;
            mask_x >>= 1;
        }
        if mask_y > 1 {
            result_y += bit_position_y * (morton_code & 1);
            morton_code >>= 1;
            bit_position_y <<= 1;
            mask_y >>= 1;
        }
    }

    result_y * x + result_x
}
