pub const fn slice_to_array<const N: usize>(slice: &[u8]) -> [u8; N] {
    let mut ar = [0; N];
    let mut i = 0;
    while i < N {
        ar[i] = slice[i];
        i += 1;
    }
    ar
}

type Symbol = u32;

pub fn decompress<const N: usize>(zip: &[u8]) -> [Symbol; N] {
    let zip = unsafe { zip.align_to::<u32>().1 };
    let num_symbols = zip[0] as usize;
    let num_entries = zip[1] as usize;
    let code_start = 2;
    let code_end = code_start + num_entries;
    let sym_start = code_end;
    let sym_end = sym_start + num_entries;
    let file_start = sym_end;
    let codes = &zip[code_start..code_end];
    let symbols = &zip[sym_start..sym_end];
    let mut ret = [0; N];
    let ret_by_symbol = unsafe { ret.align_to_mut::<Symbol>().1 };
    let mut possible_code = 0;
    let mut ret_idx = 0;
    for &word in &zip[file_start..] {
        let mut remaining_bits = 32;
        let mut stream = word as u64 | ((possible_code as u64) << 32);
        while remaining_bits != 0 {
            stream <<= 1;
            remaining_bits -= 1;
            possible_code = (stream >> 32) as u32;
            codes
                .binary_search(&possible_code)
                .map(|idx| {
                    if ret_idx < num_symbols {
                        ret_by_symbol[ret_idx] = symbols[idx];
                        ret_idx += 1;
                        stream &= 0x0000_0000_FFFF_FFFF;
                    }
                })
                .ok();
        }
        possible_code = (stream >> 32) as u32;
    }
    ret
}

#[macro_export]
macro_rules! unzip {
    ($file:literal) => {{
        use core::lazy::Lazy;
        use psx::unzip::decompress;
        use psx::unzip::slice_to_array;
        const N: usize = {
            let ar = include_bytes!($file);
            u32::from_le_bytes(slice_to_array::<4>(ar)) as usize
        };
        Lazy::<[u32; N]>::new(|| decompress(include_bytes!($file)))
    }};
}

#[macro_export]
macro_rules! unzip_now {
    ($file:literal) => {{
        use core::lazy::Lazy;
        use psx::unzip::decompress;
        use psx::unzip::slice_to_array;
        const N: usize = {
            let ar = include_bytes!($file);
            u32::from_le_bytes(slice_to_array::<4>(ar)) as usize
        };
        decompress(include_bytes!($file)) as [u32; N]
    }};
}