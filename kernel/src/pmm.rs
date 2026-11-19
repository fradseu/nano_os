use limine::memory_map::EntryType;

use crate::memmap::MEMORY_MAP_REQUEST;
use crate::serial;

const PAGE_SIZE: u64 = 0x1000; // 4 KiB

struct BumpAllocator {
    initialized: bool,
    next_addr: u64,
    end_addr: u64,
}

static mut ALLOCATOR: BumpAllocator = BumpAllocator {
    initialized: false,
    next_addr: 0,
    end_addr: 0,
};

fn align_up(addr: u64, align: u64) -> u64 {
    let mask = align - 1;
    (addr + mask) & !mask
}


pub fn init() {
    let Some(response) = MEMORY_MAP_REQUEST.get_response() else {
        serial::write_str("pmm: sem mapa de memoria\n");
        return;
    };

    let mut best_start = 0u64;
    let mut best_end = 0u64;
    let mut best_len = 0u64;

    for entry in response.entries() {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }

        // Evita regioes muito baixas (< 1 MiB) por seguranca didatica
        let start = align_up(entry.base.max(0x10_0000), PAGE_SIZE);
        let end = entry.base.saturating_add(entry.length) & !(PAGE_SIZE - 1);

        if end > start {
            let len = end - start;
            if len > best_len {
                best_len = len;
                best_start = start;
                best_end = end;
            }
        }
    }

    if best_len == 0 {
        serial::write_str("pmm: nenhuma regiao USABLE adequada encontrada\n");
        return;
    }

    unsafe {
        ALLOCATOR.initialized = true;
        ALLOCATOR.next_addr = best_start;
        ALLOCATOR.end_addr = best_end;
    }

    serial::write_str("pmm: pool inicial em ");
    crate::serial::write_u64_hex(best_start);
    serial::write_str(" ate ");
    crate::serial::write_u64_hex(best_end);
    serial::write_str("\n");
}

pub fn alloc_pages(count: usize) -> Option<u64> {
    if count == 0 {
        return None;
    }

    let size = (count as u64).checked_mul(PAGE_SIZE)?;

    unsafe {
        if !ALLOCATOR.initialized {
            return None;
        }

        let aligned = align_up(ALLOCATOR.next_addr, PAGE_SIZE);
        let next = aligned.checked_add(size)?;

        if next > ALLOCATOR.end_addr {
            return None;
        }

        ALLOCATOR.next_addr = next;
        Some(aligned)
    }
}