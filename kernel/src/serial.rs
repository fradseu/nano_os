unsafe fn outb(port: u16, val: u8) {
    unsafe {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") val,
            options(nomem, nostack, preserves_flags)
        );
    }
}

unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe {
        core::arch::asm!(
            "in al, dx",
            out("al") val,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
    }
    val
}

pub fn init() {
    unsafe {
        outb(0x3F8 + 1, 0x00); // Desativa interrupções
        outb(0x3F8 + 3, 0x80); // Habilita DLAB para configurar baud rate
        outb(0x3F8 + 0, 0x03); // Divisor baixo (38400 baud)
        outb(0x3F8 + 1, 0x00); // Divisor alto
        outb(0x3F8 + 3, 0x03); // 8 bits, sem paridade, 1 stop bit
        outb(0x3F8 + 2, 0xC7); // Ativa e limpa FIFOs
        outb(0x3F8 + 4, 0x0B); // IRQs ativadas, RTS/DSR setados
    }
}

pub fn write_byte(b: u8) {
    unsafe {
        while (inb(0x3F8 + 5) & 0x20) == 0 {}
        outb(0x3F8, b);
    }
}

pub fn write_str(s: &str) {
    for byte in s.bytes() {
        if byte == b'\n' {
            write_byte(b'\r');
        }
        write_byte(byte);
    }
}


pub fn write_u64_hex(mut n: u64) {
    write_str("0x");
    if n == 0 {
        write_byte(b'0');
        return;
    }
    let mut buf = [0u8; 16];
    let mut i = 0;
    while n > 0 {
        let d = (n & 0xF) as u8;
        buf[i] = if d < 10 { b'0' + d } else { b'a' + (d - 10) };
        n >>= 4;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        write_byte(buf[i]);
    }
}