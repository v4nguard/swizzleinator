pub mod ps;
pub mod xbox;

pub trait Swizzler {
    fn swizzle<T: Format>(
        source: &[u8],
        width: usize,
        height: usize,
        depth: usize,
        format: T,
        align_resolution: bool,
    ) -> Result<Vec<u8>, crate::SwizzleError>;
}

pub trait Deswizzler {
    fn deswizzle<T: Format>(
        source: &[u8],
        width: usize,
        height: usize,
        depth: usize,
        format: T,
        align_resolution: bool,
    ) -> Result<Vec<u8>, crate::SwizzleError>;
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
