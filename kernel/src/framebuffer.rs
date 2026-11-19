use core::ptr::write_volatile;
use limine::request::FramebufferRequest;

use crate::serial;

#[used]
#[unsafe(link_section = ".requests")]
pub static FRAMEBUFFER_REQUEST: FramebufferRequest = FramebufferRequest::new();

pub fn draw_rect(x0: usize, y0: usize, width: usize, height: usize, rgb: u32) {
    let Some(response) = FRAMEBUFFER_REQUEST.get_response() else {
        serial::write_str("fb: sem resposta do Limine\n");
        return;
    };

    let Some(fb) = response.framebuffers().next() else {
        serial::write_str("fb: nenhum framebuffer disponivel\n");
        return;
    };

    let fb_width = fb.width() as usize;
    let fb_height = fb.height() as usize;
    let pitch = fb.pitch() as usize;
    let bpp = fb.bpp() as usize;
    let addr = fb.addr();

    if bpp != 32 {
        serial::write_str("fb: bpp diferente de 32 nao suportado ainda\n");
        return;
    }

    let x1 = (x0.saturating_add(width)).min(fb_width);
    let y1 = (y0.saturating_add(height)).min(fb_height);
    let start_x = x0.min(fb_width);
    let start_y = y0.min(fb_height);

    // Formato comum em framebuffers 32-bit: 0x00RRGGBB em memória little-endian (B, G, R, 0)
    let color = rgb & 0x00FF_FFFF;

    for y in start_y..y1 {
        for x in start_x..x1 {
            let offset = y * pitch + x * 4;
            unsafe {
                let pixel_ptr = addr.add(offset) as *mut u32;
                write_volatile(pixel_ptr, color);
            }
        }
    }

    serial::write_str("fb: retangulo desenhado\n");
}