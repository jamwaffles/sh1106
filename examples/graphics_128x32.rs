//! Draw a square, circle and triangle on a 128x32px display.
//!
//! This example is for the STM32F103 "Blue Pill" board using I2C1.
//!
//! Run on a Blue Pill with `cargo run --example graphics_128x32`.

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle};
use hal::i2c::{BlockingI2c, DutyCycle, Mode};
use hal::prelude::*;
use hal::stm32;
use sh1106::prelude::*;
use sh1106::Builder;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    let scl = gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh);
    let sda = gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 400_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut disp: GraphicsMode<_> = Builder::<()>::new()
        .with_size(DisplaySize::Display128x32)
        .connect_i2c(i2c)
        .into();
    disp.init().unwrap();
    disp.flush().unwrap();

    let yoffset = 8;

    disp.draw(
        Line::new(
            Point::new(8, 16 + yoffset),
            Point::new(8 + 16, 16 + yoffset),
        )
        .stroke(Some(BinaryColor::On))
        .into_iter(),
    );
    disp.draw(
        Line::new(Point::new(8, 16 + yoffset), Point::new(8 + 8, yoffset))
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );
    disp.draw(
        Line::new(Point::new(8 + 16, 16 + yoffset), Point::new(8 + 8, yoffset))
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );

    disp.draw(
        Rectangle::new(Point::new(48, yoffset), Point::new(48 + 16, 16 + yoffset))
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );

    disp.draw(
        Circle::new(Point::new(96, yoffset + 8), 8)
            .stroke(Some(BinaryColor::On))
            .into_iter(),
    );

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}
