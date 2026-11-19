use core::mem::size_of;

use crate::serial;

const KERNEL_CODE_SEL: u16 = 0x08;
const KERNEL_DATA_SEL: u16 = 0x10;

#[repr(C, packed)]
struct DescriptorTablePointer {
    limit: u16,
    base: u64,
}

#[repr(C, align(16))]
struct Gdt {
    entries: [u64; 3],
}

// Descritor de codigo 64-bit (DPL=0, Present=1, Code=1, Readable=1, LongMode=1)
const fn code_segment_64() -> u64 {
    let access: u64 = 0b1001_1010; // P=1, DPL=00, S=1, E=1, DC=0, RW=1, A=0
    let flags: u64 = 0b0010;      // G=0, D=0, L=1, AVL=0
    (access << 40) | (flags << 52)
}

// Descritor de dados 64-bit (DPL=0, Present=1, Data=1, Writable=1)
const fn data_segment_64() -> u64 {
    let access: u64 = 0b1001_0010; // P=1, DPL=00, S=1, E=0, DC=0, RW=1, A=0
    let flags: u64 = 0b0000;
    (access << 40) | (flags << 52)
}

static mut GDT: Gdt = Gdt {
    entries: [0, code_segment_64(), data_segment_64()],
};

pub fn init() {
    unsafe {
        let gdt_ptr = DescriptorTablePointer {
            limit: (size_of::<Gdt>() - 1) as u16,
            base: core::ptr::addr_of!(GDT) as u64,
        };

        core::arch::asm!(
            "lgdt [{ptr}]",
            "push {code_sel:r}",
            "lea {tmp}, [rip + 2f]",
            "push {tmp}",
            "retfq",
            "2:",
            "mov ax, {data_sel:x}",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "xor eax, eax",
            "mov fs, ax",
            "mov gs, ax",
            ptr = in(reg) &gdt_ptr,
            code_sel = in(reg) (KERNEL_CODE_SEL as u64),
            data_sel = in(reg) (KERNEL_DATA_SEL as u64),
            tmp = lateout(reg) _,
            out("rax") _,
            options(preserves_flags)
        );
    }

    serial::write_str("gdt: carregada (CS=0x08, DS/SS=0x10)\n");
}