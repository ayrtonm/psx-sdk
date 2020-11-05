use crate::bios;
use crate::util::{append, concat, intercalate, prepend};
use crate::constrain;
use crate::gpu::color::{Color, Palette, Opacity};
use crate::gpu::position::Position;

pub fn draw_line<const N: usize>(pos: &[Position; N], pal: &Palette<N>, opacity: &Opacity)
    where [(); N + 1]:,
          [(); N + 2]:,
          [(); N + N]:,
          [(); N + N + 1]: {
    internal_draw_line::<N, {N + 1}, {N + 2}, {N + N}, {N + N + 1}>(pos, pal, opacity);
}
fn internal_draw_line<const N: usize, const M: usize, const O: usize, const P: usize, const Q: usize>(pos: &[Position; N], pal: &Palette<N>, opacity: &Opacity) {
    constrain!(N > 1);
    constrain!(M = N + 1);
    constrain!(O = N + 2);
    constrain!(P = N + N);
    constrain!(Q = P + 1);
    let cmd = match (N, pal, opacity) {
        (1, _, _) | (0, _, _) => unreachable!("Attempted to draw a one-vertex line"),
        (2, Palette::Monochrome(_), Opacity::Opaque) => 0x40,
        (2, Palette::Monochrome(_), Opacity::Translucent) => 0x42,
        (_, Palette::Monochrome(_), Opacity::Opaque) => 0x48,
        (_, Palette::Monochrome(_), Opacity::Translucent) => 0x4A,

        (2, Palette::Shaded(_), Opacity::Opaque) => 0x50,
        (2, Palette::Shaded(_), Opacity::Translucent) => 0x52,
        (_, Palette::Shaded(_), Opacity::Opaque) => 0x58,
        (_, Palette::Shaded(_), Opacity::Translucent) => 0x5A,
    };
    match (N, pal) {
        (2, Palette::Monochrome(color)) => {
            let mut ar: [u32; M] = concat(&[(*color).into()], &pos.map(|p| p.into()));
            ar[0] |= cmd << 24;
            bios::gpu_command_word_params(&ar);
        },
        (2, Palette::Shaded(colors)) => {
            let mut ar: [u32; P] = intercalate(&colors.map(|c| c.into()), &pos.map(|p| p.into()));
            ar[0] |= cmd << 24;
            bios::gpu_command_word_params(&ar);
        },
        (_, Palette::Monochrome(color)) => {
            let temp: [u32; M] = prepend((*color).into(), &pos.map(|p| p.into()));
            let mut ar: [u32; O] = append(0x5555_5555, &temp);
            ar[0] |= cmd << 24;
            bios::gpu_command_word_params(&ar);
        },
        (_, Palette::Shaded(colors)) => {
            let temp: [u32; P] = intercalate(&colors.map(|c| c.into()), &pos.map(|p| p.into()));
            let mut ar: [u32; Q] = append(0x5555_5555, &temp);
            ar[0] |= cmd << 24;
            bios::gpu_command_word_params(&ar);
        },
    };
}

pub fn draw_frame<const N: usize>(pos: &[Position; N], pal: &Palette<N>, opacity: &Opacity)
    where [(); N + 1]:,
          [(); N + 2]:,
          [(); N + 3]:,
          [(); N + N + 2]:,
          [(); N + N + 3]: {
    internal_draw_frame::<N, {N + 1}, {N + 2}, {N + 3}, {N + N + 2}, {N + N + 3}>(pos, pal, opacity);
}
fn internal_draw_frame<const N: usize, const M: usize, const O: usize, const P: usize, const Q: usize, const R: usize>(pos: &[Position; N], pal: &Palette<N>, opacity: &Opacity) {
    constrain!(N > 2);
    let new_pos: &[Position; M] = &append(pos[0], &pos);
    let new_pal = &match pal {
        Palette::Monochrome(c) => Palette::Monochrome(*c),
        Palette::Shaded(colors) => {
            let new_colors: [Color; M] = append(colors[0], colors);
            Palette::Shaded(new_colors)
        },
    };
    internal_draw_line::<M, O, P, Q, R>(new_pos, new_pal, opacity);
}