#![allow(dead_code)]
use core::hint::unreachable_unchecked;

use crate::gpu::{Bpp, Clut, TexPage};
use bitmap::Bitmap;

mod bitmap;

/// Parsed TIM data.
pub struct TIM<'a> {
    bpp: Bpp,
    /// Image bitmap data.
    pub bitmap: Bitmap<'a>,
    /// Color lookup table bitmap data.
    pub clut_bitmap: Option<Bitmap<'a>>,
}

impl<'a> TIM<'a> {
    /// Parses a slice of data in TIM format.
    pub fn new(src: &'a mut [u32]) -> Self {
        let bpp = match src[1] & 0b11 {
            0 => Bpp::Bit4,
            1 => Bpp::Bit8,
            2 => Bpp::Bit15,
            _ => unsafe { unreachable_unchecked() },
        };
        let (clut_bitmap, rest) = if (src[1] & 8) != 0 {
            let (bitmap, rest) = Bitmap::new(&mut src[2..]);
            (Some(bitmap), rest)
        } else {
            (None, &mut src[2..])
        };
        let (bitmap, _) = Bitmap::new(rest);
        TIM {
            bpp,
            bitmap,
            clut_bitmap,
        }
    }

    /// Gets the TIM data's texture page.
    pub fn tex_page(&self) -> TexPage {
        let bmp = self.bitmap.offset();
        ((bmp.x / 64) % 0xff, (bmp.y / 256) % 0xff).into()
    }

    /// Gets the TIM data's color lookup table. Returns `None` if it doesn't
    /// contain one.
    pub fn clut(&self) -> Option<Clut> {
        self.clut_bitmap.as_ref().map(|clut| {
            let clut = clut.offset();
            ((clut.x & 0xff) / 16, clut.y).into()
        })
    }
}