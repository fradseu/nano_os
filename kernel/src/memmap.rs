use limine::memory_map::EntryType;
use limine::request::MemoryMapRequest;

use crate::serial;

#[used]
#[unsafe(link_section = ".requests")]
pub static MEMORY_MAP_REQUEST: MemoryMapRequest = MemoryMapRequest::new();



fn write_u64_dec(mut n: u64) {
    if n == 0 {
        serial::write_byte(b'0');
        return;
    }
    let mut buf = [0u8; 20];
    let mut i = 0;
    while n > 0 {
        buf[i] = b'0' + ((n % 10) as u8);
        n /= 10;
        i += 1;
    }
    while i > 0 {
        i -= 1;
        serial::write_byte(buf[i]);
    }
}

fn entry_type_label(entry_type: EntryType) -> &'static str {
    match entry_type {
        EntryType::USABLE => "USABLE",
        EntryType::RESERVED => "RESERVED",
        EntryType::ACPI_RECLAIMABLE => "ACPI_RECLAIMABLE",
        EntryType::ACPI_NVS => "ACPI_NVS",
        EntryType::BAD_MEMORY => "BAD_MEMORY",
        EntryType::BOOTLOADER_RECLAIMABLE => "BOOTLOADER_RECLAIMABLE",
        EntryType::EXECUTABLE_AND_MODULES => "EXECUTABLE_AND_MODULES",
        EntryType::FRAMEBUFFER => "FRAMEBUFFER",
        _ => "OTHER",
    }
}

pub fn print_memory_map() {
    let Some(response) = MEMORY_MAP_REQUEST.get_response() else {
        serial::write_str("memmap: sem resposta do Limine\n");
        return;
    };

    let entries = response.entries();
    serial::write_str("memmap: total de regioes = ");
    write_u64_dec(entries.len() as u64);
    serial::write_str("\n");

    let mut usable_bytes: u64 = 0;

    for entry in entries {
        serial::write_str("  base=");
        crate::serial::write_u64_hex(entry.base);
        serial::write_str(" len=");
        crate::serial::write_u64_hex(entry.length);
        serial::write_str(" tipo=");
        serial::write_str(entry_type_label(entry.entry_type));
        serial::write_str("\n");

        if entry.entry_type == EntryType::USABLE {
            usable_bytes = usable_bytes.saturating_add(entry.length);
        }
    }

    serial::write_str("memmap: RAM utilizavel (MiB) = ");
    write_u64_dec(usable_bytes / (1024 * 1024));
    serial::write_str("\n");
}