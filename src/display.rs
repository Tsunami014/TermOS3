use kudos::vga_buffer::{
    ColorCode, Color,
    WRITER,
    DEFAULT_FG, DEFAULT_BG,
    BUFFER_WIDTH, BUFFER_HEIGHT
};
use x86_64::instructions::interrupts;

pub fn display() {
    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        const HORIZ_PIPE: u8 = 0xC4;
        for col in 1..BUFFER_WIDTH-1 {
            writer.set_char_at(0, col, HORIZ_PIPE);
            writer.set_char_at(BUFFER_HEIGHT-1, col, HORIZ_PIPE);
        }
    });

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        const VERT_PIPE: u8 = 0xB3;
        for row in 1..BUFFER_HEIGHT-1 {
            writer.set_char_at(row, 0, VERT_PIPE);
            writer.set_char_at(row, BUFFER_WIDTH-1, VERT_PIPE);
        }
    });

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        writer.set_char_at(0, 0, 0xDA); // Top left
        writer.set_char_at(0, BUFFER_WIDTH-1, 0xBF); // Top right
        writer.set_char_at(BUFFER_HEIGHT-1, BUFFER_WIDTH-1, 0xD9); // Bottom right
        writer.set_char_at(BUFFER_HEIGHT-1, 0, 0xC0); // Bottom left
    });

    interrupts::without_interrupts(|| {
        let mut writer = WRITER.lock();
        for row in 1..BUFFER_HEIGHT-1 {
            for col in 1..BUFFER_WIDTH-1 {
                writer.set_char_at(row, col, b'#');
            }
        }
    });
}
