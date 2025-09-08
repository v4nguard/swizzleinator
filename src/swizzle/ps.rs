use super::{Deswizzler, Format, SwizzleError, Swizzler};

pub struct Ps3;

impl Swizzler for Ps3 {
    fn swizzle<T: Format>(
        source: &mut [u8],
        dest: &mut [u8],
        dimensions: (usize, usize, usize),
        format: T,
        align_resolution: bool,
    ) -> Result<(), SwizzleError> {
        ps3::do_swizzle(source, dest, dimensions, format, false, align_resolution);
        Ok(())
    }
}

impl Deswizzler for Ps3 {
    fn deswizzle<T: Format>(
        source: &mut [u8],
        dest: &mut [u8],
        dimensions: (usize, usize, usize),
        format: T,
        align_resolution: bool,
    ) -> Result<(), SwizzleError> {
        ps3::do_swizzle(source, dest, dimensions, format, true, align_resolution);
        Ok(())
    }
}

mod ps3 {
    use crate::swizzle::{Format, SwizzleError};

    pub fn do_swizzle<T: Format>(
        source: &mut [u8],
        dest: &mut [u8],
        dimensions: (usize, usize, usize),
        format: T,
        unswizzle: bool,
        align_resolution: bool,
    ) {
        let (width, height, depth) = dimensions;
        let pixel_block_size = format.pixel_block_size();
        let block_size = format.block_size();

        let (width_src, height_src) = if align_resolution && format.is_compressed() {
            (width.next_power_of_two(), height.next_power_of_two())
        } else {
            (width, height)
        };

        let width_texels = width_src / pixel_block_size;
        let height_texels = height_src / pixel_block_size;

        let mut data_index = 0;

        let texel_size = width_texels * height_texels;

        for z in 0..depth {
            let slice_dest = &mut dest[(z * width * height * format.bpp()) / 8..];

            for t in 0..texel_size {
                let pixel_index = crate::swizzle::morton(t, width_texels, height_texels);
                let dest_index = block_size * pixel_index;
                let (src, dst) = if unswizzle {
                    (data_index, dest_index)
                } else {
                    (dest_index, data_index)
                };

                if (src + block_size) <= source.len() && (dst + block_size) <= slice_dest.len() {
                    slice_dest[dst..dst + block_size]
                        .copy_from_slice(&source[src..src + block_size]);
                }

                data_index += block_size;
            }
        }
    }
}

pub struct Ps4;

impl Swizzler for Ps4 {
    fn swizzle<T: Format>(
        source: &mut [u8],
        dest: &mut [u8],
        dimensions: (usize, usize, usize),
        format: T,
        align_resolution: bool,
    ) -> Result<(), SwizzleError> {
        ps4::do_swizzle(source, dest, dimensions, format, false, align_resolution);
        Ok(())
    }
}

impl Deswizzler for Ps4 {
    fn deswizzle<T: Format>(
        source: &mut [u8],
        dest: &mut [u8],
        dimensions: (usize, usize, usize),
        format: T,
        align_resolution: bool,
    ) -> Result<(), SwizzleError> {
        ps4::do_swizzle(source, dest, dimensions, format, true, align_resolution);
        Ok(())
    }
}

mod ps4 {
    use crate::swizzle::Format;

    pub fn do_swizzle<T: Format>(
        source: &mut [u8],
        dest: &mut [u8],
        dimensions: (usize, usize, usize),
        format: T,
        unswizzle: bool,
        align_resolution: bool,
    ) {
        let (width, height, depth) = dimensions;
        let pixel_block_size = format.pixel_block_size();
        let block_size = format.block_size();

        let (width_src, height_src) = if align_resolution && format.is_compressed() {
            (width.next_power_of_two(), height.next_power_of_two())
        } else {
            (width, height)
        };

        let width_texels_dest = width / pixel_block_size;
        let height_texels_dest = height / pixel_block_size;

        let width_texels = width_src / pixel_block_size;
        let width_texels_aligned = width_texels.div_ceil(8);
        let height_texels = height_src / pixel_block_size;
        let height_texels_aligned = height_texels.div_ceil(8);
        let mut data_index = 0;

        for z in 0..depth {
            let slice_dest = &mut dest[(z * width * height * format.bpp()) / 8..];

            for y in 0..height_texels_aligned {
                for x in 0..width_texels_aligned {
                    for t in 0..64 {
                        let pixel_index = crate::swizzle::morton(t, 8, 8);
                        let div = pixel_index / 8;
                        let rem = pixel_index % 8;
                        let x_offset = (x * 8) + rem;
                        let y_offset = (y * 8) + div;

                        if x_offset < width_texels_dest && y_offset < height_texels_dest {
                            let dest_pixel_index = y_offset * width_texels_dest + x_offset;
                            let dest_index = block_size * dest_pixel_index;
                            let (src, dst) = if unswizzle {
                                (data_index, dest_index)
                            } else {
                                (dest_index, data_index)
                            };

                            if (src + block_size) <= source.len()
                                && (dst + block_size) <= slice_dest.len()
                            {
                                slice_dest[dst..dst + block_size]
                                    .copy_from_slice(&source[src..src + block_size]);
                            }
                        }

                        data_index += block_size;
                    }
                }
            }
        }
    }
}
