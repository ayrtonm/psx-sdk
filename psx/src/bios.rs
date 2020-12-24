// This file was automatically generated by build.rs
#![allow(unused_variables)]

#[naked]
#[inline(never)]
/// [BIOS Function A(33h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    let ret: *mut u8;
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x33",
                lateout("$2") ret);
    }
    ret
}

#[naked]
#[inline(never)]
/// [BIOS Function A(34h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn free(buf: *mut u8) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x34",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(37h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn calloc(sizex: usize, sizey: usize) -> *const u8 {
    let ret: *const u8;
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x37",
                lateout("$2") ret);
    }
    ret
}

#[naked]
#[inline(never)]
/// [BIOS Function A(38h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn realloc(old_buf: *const u8, new_size: usize) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x38",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(39h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn init_heap(addr: usize, size: usize) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x39",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(3Fh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn printf(s: *const u8, v: u32) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x3F",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(47h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn gpu_send_dma(xdst: u16, ydst: u16, xsiz: u16, ysize: u16, src: u32) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x47",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(48h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn gpu_gp1_command_word(cmd: u32) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x48",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(49h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn gpu_command_word(cmd: u32) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x49",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(4Ah)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn gpu_command_word_params(src: *const u32, num: usize) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x4A",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(4Dh)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn gpu_get_status() -> u32 {
    let ret: u32;
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x4D",
                lateout("$2") ret);
    }
    ret
}

#[naked]
#[inline(never)]
/// [BIOS Function A(00h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn file_open(filename: *const u8, accessmode: u32) -> u8 {
    let ret: u8;
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x00",
                lateout("$2") ret);
    }
    ret
}

#[naked]
#[inline(never)]
/// [BIOS Function A(41h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn load_exe_header(filename: *const u8, headerbuf: *mut u8) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x41",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(42h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn load_exe_file(filename: *const u8, headerbuf: *mut u8) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x42",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(43h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn do_execute(headerbuf: *mut u8, param1: u32, param2: u32) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x43",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(51h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn load_and_execute(filename: *const u8, stackbase: u32, stackoffset: u32) {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x51",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function A(44h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn flush_cache() {
    unsafe {
        asm!(".set noreorder
              j 0xA0
              li $9, 0x44",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function B(12h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn init_pad(buf1: *mut u8, siz1: usize, buf2: *mut u8, siz2: usize) {
    unsafe {
        asm!(".set noreorder
              j 0xB0
              li $9, 0x12",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function B(13h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn start_pad() {
    unsafe {
        asm!(".set noreorder
              j 0xB0
              li $9, 0x13",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function B(14h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn stop_pad() {
    unsafe {
        asm!(".set noreorder
              j 0xB0
              li $9, 0x14",
                lateout("$2") _);
    }
}

#[naked]
#[inline(never)]
/// [BIOS Function SYS(01h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn enter_critical_section() -> u8 {
    let ret: u8;
    unsafe {
        asm!(".set noreorder
              li $4, 0x01
              syscall 0x0",
                lateout("$2") ret);
    }
    ret
}

#[naked]
#[inline(never)]
/// [BIOS Function SYS(02h)](http://problemkaputt.de/psx-spx.htm#biosfunctionsummary)
pub extern "C" fn exit_critical_section() {
    unsafe {
        asm!(".set noreorder
              li $4, 0x02
              syscall 0x0",
                lateout("$2") _);
    }
}
