#![no_std]
#![no_main]

mod framebuffer;
mod gdt;
mod memmap;
mod pmm;
mod serial;

use core::panic::PanicInfo;
use limine::BaseRevision;

#[used]
#[unsafe(link_section = ".requests")]
static BASE_REVISION: BaseRevision = BaseRevision::new();

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    serial::init();
    serial::write_str("nano_os: kernel iniciado\n");
    memmap::print_memory_map();
    pmm::init();
    gdt::init();

    framebuffer::draw_rect(80, 60, 240, 120, 0x003B82F6);

    if let Some(p1) = pmm::alloc_pages(1) {
        serial::write_str("pmm: pagina 1 = ");
        serial::write_u64_hex(p1);
        serial::write_str("\n");
    }

    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial::write_str("nano_os: PANIC!\n");
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}
