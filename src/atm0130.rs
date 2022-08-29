#![allow(unused_assignments)]

use cortex_m::{delay::Delay, prelude::*};
use embedded_hal::digital::v2::OutputPin;
use rp_pico::hal::{
    gpio::{Output, Pin, PinId, PushPull},
    spi::{Enabled, SpiDevice},
    Spi,
};

use crate::artemis;

const CHARS: [u8; 475] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x08, 0x40, 0x10, 0x00, 0x52, 0x94, 0x00, 0x00, 0x00, 0x52,
    0x95, 0xF5, 0x7D, 0x4A, 0x23, 0xE8, 0xE2, 0xF8, 0x80, 0xC6, 0x44, 0x44, 0x4C, 0x60, 0x64, 0xA8,
    0x8A, 0xC9, 0xA0, 0x61, 0x10, 0x00, 0x00, 0x00, 0x11, 0x10, 0x84, 0x10, 0x40, 0x41, 0x04, 0x21,
    0x11, 0x00, 0x01, 0x2A, 0xEA, 0x90, 0x00, 0x01, 0x08, 0xE2, 0x10, 0x00, 0x00, 0x00, 0x06, 0x11,
    0x00, 0x00, 0x01, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x00, 0x00, 0x44, 0x44, 0x40, 0x00,
    0x74, 0x67, 0x5C, 0xC5, 0xC0, 0x23, 0x08, 0x42, 0x11, 0xC0, 0x74, 0x42, 0x22, 0x23, 0xE0, 0xF8,
    0x88, 0x20, 0xC5, 0xC0, 0x11, 0x95, 0x2F, 0x88, 0x40, 0xFC, 0x3C, 0x10, 0xC5, 0xC0, 0x32, 0x11,
    0xE8, 0xC5, 0xC0, 0xF8, 0x44, 0x44, 0x21, 0x00, 0x74, 0x62, 0xE8, 0xC5, 0xC0, 0x74, 0x62, 0xF0,
    0x89, 0x80, 0x03, 0x18, 0x06, 0x30, 0x00, 0x03, 0x18, 0x06, 0x11, 0x00, 0x11, 0x11, 0x04, 0x10,
    0x40, 0x00, 0x3E, 0x0F, 0x80, 0x00, 0x41, 0x04, 0x11, 0x11, 0x00, 0x74, 0x42, 0x22, 0x00, 0x80,
    0x74, 0x42, 0xDA, 0xD5, 0xC0, 0x74, 0x63, 0x1F, 0xC6, 0x20, 0xF4, 0x63, 0xE8, 0xC7, 0xC0, 0x74,
    0x61, 0x08, 0x45, 0xC0, 0xE4, 0xA3, 0x18, 0xCB, 0x80, 0xFC, 0x21, 0xE8, 0x43, 0xE0, 0xFC, 0x21,
    0xE8, 0x42, 0x00, 0x74, 0x61, 0x78, 0xC5, 0xE0, 0x8C, 0x63, 0xF8, 0xC6, 0x20, 0x71, 0x08, 0x42,
    0x11, 0xC0, 0x38, 0x84, 0x21, 0x49, 0x80, 0x8C, 0xA9, 0x8A, 0x4A, 0x20, 0x84, 0x21, 0x08, 0x43,
    0xE0, 0x8E, 0xEB, 0x58, 0xC6, 0x20, 0x8C, 0x73, 0x59, 0xC6, 0x20, 0x74, 0x63, 0x18, 0xC5, 0xC0,
    0xF4, 0x63, 0xE8, 0x42, 0x00, 0x74, 0x63, 0x1A, 0xC9, 0xA0, 0xF4, 0x63, 0xEA, 0x4A, 0x20, 0x74,
    0x20, 0xE0, 0x87, 0xC0, 0xF9, 0x08, 0x42, 0x10, 0x80, 0x8C, 0x63, 0x18, 0xC5, 0xC0, 0x8C, 0x63,
    0x18, 0xA8, 0x80, 0x8C, 0x63, 0x5A, 0xD5, 0x40, 0x8C, 0x54, 0x45, 0x46, 0x20, 0x8C, 0x62, 0xA2,
    0x10, 0x80, 0xF8, 0x44, 0x44, 0x43, 0xE0, 0x72, 0x10, 0x84, 0x21, 0xC0, 0x8A, 0xBE, 0x4F, 0x90,
    0x80, 0x70, 0x84, 0x21, 0x09, 0xC0, 0x22, 0xA2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03, 0xE0,
    0x41, 0x04, 0x00, 0x00, 0x00, 0x00, 0x1C, 0x17, 0xC5, 0xE0, 0x84, 0x2D, 0x98, 0xC7, 0xC0, 0x00,
    0x1D, 0x08, 0x45, 0xC0, 0x08, 0x5B, 0x38, 0xC5, 0xE0, 0x00, 0x1D, 0x1F, 0xC1, 0xC0, 0x32, 0x51,
    0xC4, 0x21, 0x00, 0x03, 0xE3, 0x17, 0x85, 0xC0, 0x84, 0x2D, 0x98, 0xC6, 0x20, 0x20, 0x18, 0x42,
    0x11, 0xC0, 0x10, 0x0C, 0x21, 0x49, 0x80, 0x84, 0x25, 0x4C, 0x52, 0x40, 0x61, 0x08, 0x42, 0x11,
    0xC0, 0x00, 0x35, 0x5A, 0xC6, 0x20, 0x00, 0x2D, 0x98, 0xC6, 0x20, 0x00, 0x1D, 0x18, 0xC5, 0xC0,
    0x00, 0x3D, 0x1F, 0x42, 0x00, 0x00, 0x1B, 0x37, 0x84, 0x20, 0x00, 0x2D, 0x98, 0x42, 0x00, 0x00,
    0x1D, 0x07, 0x07, 0xC0, 0x42, 0x38, 0x84, 0x24, 0xC0, 0x00, 0x23, 0x18, 0xCD, 0xA0, 0x00, 0x23,
    0x18, 0xA8, 0x80, 0x00, 0x23, 0x1A, 0xD5, 0x40, 0x00, 0x22, 0xA2, 0x2A, 0x20, 0x00, 0x23, 0x17,
    0x85, 0xC0, 0x00, 0x3E, 0x22, 0x23, 0xE0, 0x11, 0x08, 0x82, 0x10, 0x40, 0x21, 0x08, 0x42, 0x10,
    0x80, 0x41, 0x08, 0x22, 0x11, 0x00, 0x00, 0x11, 0x51, 0x00, 0x00,
];

const ARTEMIS_COLOR: Color = Color(0x37, 0xD8, 0xDB);

pub const FONT_WIDTH: u8 = 5;
pub const FONT_HEIGHT: u8 = 8;

pub struct Atm0130<SPI, SS, DC, RS>
where
    SPI: SpiDevice,
    SS: PinId,
    DC: PinId,
    RS: PinId,
{
    spi_enabled: Spi<Enabled, SPI, 8>,
    ss: Pin<SS, Output<PushPull>>,
    data_cmd: Pin<DC, Output<PushPull>>,
    reset: Pin<RS, Output<PushPull>>,
}

impl<SPI, SS, DC, RS> Atm0130<SPI, SS, DC, RS>
where
    SPI: SpiDevice,
    SS: PinId,
    DC: PinId,
    RS: PinId,
{
    pub fn init(
        spi: Spi<Enabled, SPI, 8>,
        ss: Pin<SS, Output<PushPull>>,
        data_cmd: Pin<DC, Output<PushPull>>,
        reset: Pin<RS, Output<PushPull>>,
    ) -> Self {
        Self {
            spi_enabled: spi,
            ss,
            data_cmd,
            reset,
        }
    }

    pub fn begin(&mut self, delay: &mut Delay) {
        self.ss.set_high().unwrap();
        self.reset_lcd(delay);
        self.ss.set_low().unwrap();

        self.write_reg(0x11);
        delay.delay_ms(100);

        self.write_reg(0x36); //MADCTL
        self.write_data(0x00);
        //MY=0
        //MX=0
        //MV=0
        //ML=0
        //RGB=0
        //MH=0
        self.write_reg(0x3A);
        self.write_data(0x55); //65K color , 16bit / pixel

        ////--------------------------------ST7789V Frame rate

        self.write_reg(0xb2);
        self.write_data(0x0c);
        self.write_data(0x0c);
        self.write_data(0x00);
        self.write_data(0x33);
        self.write_data(0x33);

        delay.delay_ms(2);

        self.write_reg(0xb7);
        self.write_data(0x75);

        delay.delay_ms(2);
        ////---------------------------------ST7789V Power
        self.write_reg(0xc2);
        self.write_data(0x01);

        delay.delay_ms(2);

        self.write_reg(0xc3);
        self.write_data(0x10);

        delay.delay_ms(2);

        self.write_reg(0xc4);
        self.write_data(0x20);

        delay.delay_ms(2);

        self.write_reg(0xc6);
        self.write_data(0x0f);

        self.write_reg(0xb0);
        self.write_data(0x00);
        self.write_data(0xf0); //RRRR RGGGG GGGB BBBB

        delay.delay_ms(2);

        self.write_reg(0xD0);
        self.write_data(0xA4);
        self.write_data(0xA1);
        delay.delay_ms(2);

        ////--------------------------------ST7789V gamma
        self.write_reg(0x21);

        delay.delay_ms(2);

        self.write_reg(0xbb);
        self.write_data(0x3b);

        delay.delay_ms(2);

        self.write_reg(0xE0); //Set Gamma
        self.write_data(0xF0);
        self.write_data(0x0b);
        self.write_data(0x11);
        self.write_data(0x0e);
        self.write_data(0x0d);
        self.write_data(0x19);
        self.write_data(0x36);
        self.write_data(0x33);
        self.write_data(0x4b);
        self.write_data(0x07);
        self.write_data(0x14);
        self.write_data(0x14);
        self.write_data(0x2c);
        self.write_data(0x2e);

        delay.delay_ms(2);

        self.write_reg(0xE1); //Set Gamma
        self.write_data(0xF0);
        self.write_data(0x0d);
        self.write_data(0x12);
        self.write_data(0x0b);
        self.write_data(0x09);
        self.write_data(0x03);
        self.write_data(0x32);
        self.write_data(0x44);
        self.write_data(0x48);
        self.write_data(0x39);
        self.write_data(0x16);
        self.write_data(0x16);
        self.write_data(0x2d);
        self.write_data(0x30);

        self.write_reg(0x2A);
        self.write_data(0x00);
        self.write_data(0x00);
        self.write_data(0x00);
        self.write_data(0xEF);

        self.write_reg(0x2B);
        self.write_data(0x00);
        self.write_data(0x00);
        self.write_data(0x00);
        self.write_data(0xEF);

        self.write_reg(0x29); //Display on

        delay.delay_ms(2);
        self.write_reg(0x2c);
        self.ss.set_high().unwrap();
    }

    pub fn draw_rect(&mut self, x: u8, y: u8, width: u8, height: u8, color: Color) {
        let fig_color = color.to_u16();
        let color_h = (fig_color >> 8) as u8;
        let color_l = (fig_color & 0x00FF) as u8;

        self.ss.set_low().unwrap();
        self.set_window(x, y, width, height);

        self.data_cmd.set_high().unwrap();
        let count: u32 = width as u32 * height as u32;
        for _ in 0..count {
            self.spi_enabled.write(&[color_h, color_l]).unwrap();
        }
        self.ss.set_high().unwrap();
    }

    pub fn draw_text(
        &mut self,
        text: &str,
        mut x: u8,
        y: u8,
        size_scalar: u32,
        text_color: Color,
        background_color: Color,
    ) {
        for char in text.chars().rev() {
            self.draw_char(char as i8, x, y, size_scalar, text_color, background_color);
            x += (FONT_WIDTH + 1) * size_scalar as u8;
        }
    }

    pub fn draw_char(
        &mut self,
        mut c: i8,
        x: u8,
        y: u8,
        size_scalar: u32,
        text_color: Color,
        background_color: Color,
    ) {
        let mut char_queue = [0u8; 5];
        if (0x20..=0x7E).contains(&c) {
            c -= 0x20;
            for i in 0..5 {
                char_queue[i] = CHARS[5 * c as usize + i];
            }
        } else {
            for mut _elem in char_queue {
                _elem = 0xFF;
            }
        }
        self.ss.set_low().unwrap();

        let width = FONT_WIDTH;
        let height = FONT_HEIGHT;

        self.set_window(x, y, width * size_scalar as u8, height * size_scalar as u8);
        for i in 0..height {
            for _ in 0..size_scalar {
                for j in 0..width {
                    let now_bit = i * width + width - 1 - j;
                    let b =
                        char_queue[now_bit as usize / height as usize] & 0x80 >> (now_bit % height);
                    for _ in 0..size_scalar {
                        if b > 0 {
                            self.put_pixel(text_color);
                        } else {
                            self.put_pixel(background_color);
                        }
                    }
                }
            }
        }

        // if not edge
        if x + (FONT_WIDTH + 1) * size_scalar as u8 <= 240 {
            self.set_window(
                x + FONT_WIDTH * size_scalar as u8,
                y,
                size_scalar as u8,
                height * size_scalar as u8,
            );
            for _ in 0..FONT_HEIGHT * size_scalar as u8 {
                self.put_pixel(background_color);
            }
        }
        self.ss.set_high().unwrap();
    }

    pub fn draw_text_fast(
        &mut self,
        text: &str,
        mut x: u8,
        y: u8,
        text_color: Color,
        background_color: Color,
    ) {
        for char in text.chars().rev() {
            self.draw_char_fast(char as i8, x, y, text_color, background_color);
            x += FONT_WIDTH + 1;
        }
    }

    pub fn draw_char_fast(
        &mut self,
        mut c: i8,
        x: u8,
        y: u8,
        text_color: Color,
        background_color: Color,
    ) {
        let mut char_queue = [0u8; 5];
        if (0x20..=0x7E).contains(&c) {
            c -= 0x20;
            for i in 0..5 {
                char_queue[i] = CHARS[5 * c as usize + i];
            }
        } else {
            for mut _elem in char_queue {
                _elem = 0xFF;
            }
        }
        self.ss.set_low().unwrap();

        let width = FONT_WIDTH;
        let height = FONT_HEIGHT;

        self.set_window(x, y, width, height);

        let mut buff = [0u8; FONT_HEIGHT as usize * FONT_WIDTH as usize * 2];
        for i in 0..height {
            for j in 0..width {
                let now_bit = i * width + width - 1 - j;
                let b = char_queue[now_bit as usize / height as usize] & 0x80 >> (now_bit % height);
                if b > 0 {
                    let index = (i as usize * width as usize + j as usize) * 2;
                    buff[index] = (text_color.to_u16() >> 8) as u8;
                    buff[index + 1] = (text_color.to_u16() & 0xFF) as u8;
                } else {
                    let index = (i as usize * width as usize + j as usize) * 2;
                    buff[index] = (background_color.to_u16() >> 8) as u8;
                    buff[index + 1] = (background_color.to_u16() & 0xFF) as u8;
                }
            }
        }

        self.data_cmd.set_high().unwrap();
        self.spi_enabled.write(&buff).unwrap();

        // if not edge
        if x + FONT_WIDTH < 240 {
            self.set_window(x + FONT_WIDTH, y, 1, height);
            for _ in 0..FONT_HEIGHT {
                self.put_pixel(background_color);
            }
        }
        self.ss.set_high().unwrap();
    }

    pub fn draw_logo(&mut self, x: u8, y: u8) {
        self.ss.set_low().unwrap();
        self.set_window(x, y, artemis::IMG_WIDTH, artemis::IMG_HEIGHT);
        self.data_cmd.set_high().unwrap();
        for color in artemis::IMG_DATA {
            self.spi_enabled
                .write(&[(color >> 8) as u8, (color & 0xFF) as u8])
                .unwrap();
        }
        self.ss.set_high().unwrap();
    }

    pub fn draw_img(&mut self, x: u8, y: u8, width: u8, height: u8, data: &[u8]) {
        self.ss.set_low().unwrap();
        self.set_window(x, y, width, height);
        self.data_cmd.set_high().unwrap();
        for i in 0..height as usize {
            for j in 0..width as usize {
                let index =
                    (height as usize - i - 1) * height as usize * 3 + (width as usize - j - 1) * 3;
                let color = Color(data[index], data[index + 1], data[index + 2]).to_u16();
                self.spi_enabled
                    .write(&[(color >> 8) as u8, (color & 0xFF) as u8])
                    .unwrap();
            }
        }

        self.ss.set_high().unwrap();
    }

    pub fn draw_info(&mut self, text: &str) {
        let text_size = text_size(text, 1);
        let x = 120 - text_size.0 / 2;
        let y = 120 - text_size.1 / 2;
        self.draw_text(text, x, y, 1, ARTEMIS_COLOR, Color(0, 0, 0));
    }

    fn put_pixel(&mut self, color: Color) {
        self.data_cmd.set_high().unwrap();
        self.spi_enabled
            .write(&[(color.to_u16() >> 8) as u8, (color.to_u16() & 0xFF) as u8])
            .unwrap();
    }

    fn set_window(&mut self, x: u8, y: u8, width: u8, height: u8) {
        self.write_reg(0x2A);
        self.write_data(0x00);
        self.write_data(x);
        self.write_data(0x00);
        self.write_data(x + width - 1);

        self.write_reg(0x2B);
        self.write_data(0x00);
        self.write_data(y);
        self.write_data(0x00);
        self.write_data(y + height - 1);

        self.write_reg(0x2c);
    }

    fn reset_lcd(&mut self, delay: &mut Delay) {
        self.reset.set_high().unwrap();
        delay.delay_ms(20);
        self.reset.set_low().unwrap();
        delay.delay_ms(20);
        self.reset.set_high().unwrap();
        delay.delay_ms(20);
    }

    fn write_reg(&mut self, data: u8) {
        self.data_cmd.set_low().unwrap();
        self.spi_enabled.write(&[data]).unwrap();
    }

    fn write_data(&mut self, data: u8) {
        self.data_cmd.set_high().unwrap();
        self.spi_enabled.write(&[data]).unwrap();
    }
}

#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    fn to_u16(self) -> u16 {
        let mut color = self.2 as u16 >> 3;
        color |= (self.1 as u16 & 0xFC) << 3;
        color | (self.0 as u16 & 0xF8) << 8
    }
}

pub fn text_size(str: &str, size_scalar: u32) -> (u8, u8) {
    (
        str.len() as u8 * (FONT_WIDTH + 1) * size_scalar as u8,
        FONT_HEIGHT * size_scalar as u8,
    )
}
