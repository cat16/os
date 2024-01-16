use bootloader_api::info::FrameBuffer;
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::{Rgb888, RgbColor},
    primitives::{
        Circle, Primitive, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment,
        Triangle,
    },
    text::{Alignment, Text},
    Drawable, Pixel,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn set_pixel_in(framebuffer: &mut FrameBuffer, position: Position, color: Color) {
    let info = framebuffer.info();
    let byte_offset = {
        let line_offset = position.y * info.stride;
        let pixel_offset = line_offset + position.x;
        pixel_offset * info.bytes_per_pixel
    };
    let pixel_buffer = &mut framebuffer.buffer_mut()[byte_offset..];
    match info.pixel_format {
        bootloader_api::info::PixelFormat::Rgb => {
            pixel_buffer[0] = color.red;
            pixel_buffer[1] = color.green;
            pixel_buffer[2] = color.blue;
        }
        bootloader_api::info::PixelFormat::Bgr => {
            pixel_buffer[0] = color.blue;
            pixel_buffer[1] = color.green;
            pixel_buffer[2] = color.red;
        }
        bootloader_api::info::PixelFormat::U8 => {
            let gray = color.red / 3 + color.green / 3 + color.blue / 3;
            pixel_buffer[0] = gray;
        }
        other => panic!("unknown pixel format {other:?}"),
    }
}

pub struct Display<'a> {
    framebuffer: &'a mut FrameBuffer,
}

impl<'a> Display<'a> {
    pub fn new(framebuffer: &'a mut FrameBuffer) -> Self {
        Self { framebuffer }
    }

    fn draw_pixel(&mut self, pos: Position, color: Color) {
        let (width, height) = {
            let info = self.framebuffer.info();
            (info.width, info.height)
        };
        if pos.x >= width || pos.y >= height {
            return;
        }
        set_pixel_in(&mut self.framebuffer, pos, color);
    }
}

impl OriginDimensions for Display<'_> {
    fn size(&self) -> embedded_graphics::prelude::Size {
        let info = self.framebuffer.info();
        Size {
            width: info.width as u32,
            height: info.height as u32,
        }
    }
}

impl DrawTarget for Display<'_> {
    type Color = Rgb888;
    type Error = core::convert::Infallible;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for Pixel(coordinates, color) in pixels.into_iter() {
            self.draw_pixel(
                Position {
                    x: coordinates.x as usize,
                    y: coordinates.y as usize,
                },
                Color {
                    red: color.r(),
                    green: color.g(),
                    blue: color.b(),
                },
            );
        }
        Ok(())
    }
}

pub fn draw_test(framebuffer: &mut FrameBuffer) {
    for b in framebuffer.buffer_mut() {
        *b = 0;
    }
    let mut display = Display::new(framebuffer);
    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(Rgb888::new(0, 255, 255), 1);
    let thick_stroke = PrimitiveStyle::with_stroke(Rgb888::new(0, 255, 255), 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::new(0, 255, 255))
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(Rgb888::new(0, 255, 255));
    let character_style = MonoTextStyle::new(&FONT_6X10, Rgb888::new(0, 255, 255));

    let yoffset = 10;

    // Draw a 3px wide outline around the display.
    display
        .bounding_box()
        .into_styled(border_stroke)
        .draw(&mut display)
        .unwrap();

    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)
        .unwrap();

    // Draw a triangle.
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(&mut display)
    .unwrap();

    // Draw a filled square
    Rectangle::new(Point::new(52, yoffset), Size::new(16, 16))
        .into_styled(fill)
        .draw(&mut display)
        .unwrap();

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(88, yoffset), 17)
        .into_styled(thick_stroke)
        .draw(&mut display)
        .unwrap();

    // Draw centered text.
    let text = "jugando";
    Text::with_alignment(
        text,
        display.bounding_box().center() + Point::new(0, 15),
        character_style,
        Alignment::Center,
    )
    .draw(&mut display)
    .unwrap();
}
