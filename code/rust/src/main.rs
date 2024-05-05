#![no_main]
#![no_std]
#[macro_use]
extern crate alloc;

mod falang;

use cortex_m_rt::entry;
use hal::gpio::PushPull;

use core::fmt::Debug;
use core::fmt::Write;
use micro_rand::*;
//use cortex_m_semihosting::hio;
//use embedded_graphics::fonts::Font12x16;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Rectangle};
use embedded_graphics::Drawing;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use falang::State::ButtonsState;
use falang::State::Config;
use hal::gpio::Output;
// use generic_array::{ArrayLength, GenericArray};
use crate::hal::{
    prelude::*,
    serial::{self, Serial},
    spi::Spi,
    stm32,
};
use ili9341::{self, Interface};
use panic_semihosting as _;
use stm32f4xx_hal as hal;

// fn draw_rand_rect<SPI, CS, DC, RESET>(&mut lcd: ili9341::Ili9341<SPI, CS, DC, RESET>) {

use embedded_alloc::Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

// }
#[derive(PartialEq)]
#[derive(Clone)]
enum PressedButton {
    None,
    Left,
    Right,
    Top,
    Bottom,
}

struct ApiRealization<IFACE, RESET, PB> {
    lcd: ili9341::Ili9341<IFACE, RESET>,
    delay: hal::delay::Delay,
    keyboard: PB,
    rand: Random,
    pressedButton: PressedButton,
}
/*
impl<IfaceE, PinE, IFACE, RESET> Ili9341<IFACE, RESET>
    where
        IFACE: Interface<Error=IfaceE>,
        RESET: OutputPin<Error = PinE>,*/

/**impl<SPI, CS, DC, SpiE, PinE> SpiInterface<SPI, CS, DC>
where SPI: spi::Transfer<u8, Error = SpiE> + spi::Write<u8, Error = SpiE>,
      CS: OutputPin<Error = PinE>,
      DC: OutputPin<Error = PinE>, */

impl<IFACE, RESET, PB> ApiRealization<IFACE, RESET, PB>
where
    PB: GetPressedButtonImpl,
{
    fn new(
        lcd: ili9341::Ili9341<IFACE, RESET>,
        delay: hal::delay::Delay,
        keyboard: PB,
    ) -> ApiRealization<IFACE, RESET, PB> {
        ApiRealization {
            lcd,
            delay,
            keyboard,
            rand: Random::new(1234),
            pressedButton: PressedButton::None,
        }
    }
}

//ili9341::spi::SpiInterface<Spi<stm32::SPI5, (hal::gpio::gpiof::PF7<hal::gpio::Alternate<hal::gpio::AF5>>, hal::spi::NoMiso, hal::gpio::gpiof::PF9<hal::gpio::Alternate<hal::gpio::AF5>>)>, hal::gpio::gpioc::PC2<hal::gpio::Output<hal::gpio::PushPull>>, hal::gpio::gpiod::PD13<hal::gpio::Output<hal::gpio::PushPull>>>, hal::gpio::gpiof::PF10<hal::gpio::Output<hal::gpio::PushPull>>
/**impl<IfaceE, PinE, IFACE, RESET> Ili9341<IFACE, RESET>
where
    IFACE: Interface<Error=IfaceE>,
    RESET: OutputPin<Error = PinE>, */

impl<IfaceE, PB, IFACE, RESET, PinE> crate::falang::falang_global::Apis
    for ApiRealization<IFACE, RESET, PB>
where
    IFACE: Interface<Error = IfaceE>,
    RESET: OutputPin<Error = PinE>,
    PinE: core::fmt::Debug,
    IfaceE: core::fmt::Debug,
    PB: GetPressedButtonImpl,
{
    fn GameApi_Drawing_DrawCircle(
        &mut self,
        x: i32,
        y: i32,
        r: i32,
        color: &crate::falang::State::Color,
    ) -> () {
        let c = Rgb565::new(color.r as u8, color.g as u8, color.b as u8);
        let circle = Circle::new(Point::new(x, y), r as u32).fill(Some(c));
        self.lcd.draw(circle);
    }

    fn GameApi_Drawing_DrawRect(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: &crate::falang::State::Color,
    ) -> () {
        let c = Rgb565::new(color.r as u8, color.g as u8, color.b as u8);
        let rect =
            Rectangle::new(Point::new(x, y), Point::new(x + w as i32, y + h as i32)).fill(Some(c));
        self.lcd.draw(rect);
    }

    fn GameApi_Application_GetButtonsState(&mut self) -> crate::falang::State::ButtonsState {
        let mut btn = self.keyboard.get_pressed_button();
        if btn == PressedButton::None {
          btn = self.pressedButton.clone();
        }
        self.pressedButton = PressedButton::None;
        crate::falang::State::ButtonsState {
            bottom: btn == PressedButton::Bottom,
            left: btn == PressedButton::Left,
            right: btn == PressedButton::Right,
            top: btn == PressedButton::Top,
        }
    }

    fn GameApi_Application_Sleep(&mut self) -> () {
      for _i in 1..10 {
        let btn = self.keyboard.get_pressed_button();
        if btn != PressedButton::None {
          self.pressedButton = btn;
        }
        self.delay.delay_ms(30 as u16);
      }
        
    }

    fn GameApi_Application_GetRandom(&mut self, min: i32, max: i32) -> i32 {
        (self.rand.next_f32() * (max as f32 - min as f32 - 1 as f32) + min as f32) as i32
    }
}

use alloc::boxed::Box;

struct KeyboardDriver<COL0, COL1, COL2, COL3, ROW0, ROW1, ROW2, ROW3> {
    col0: COL0,
    col1: COL1,
    col2: COL2,
    col3: COL3,
    row0: ROW0,
    row1: ROW1,
    row2: ROW2,
    row3: ROW3,
}

impl<COL0, COL1, COL2, COL3, ROW0, ROW1, ROW2, ROW3, PinE>
    KeyboardDriver<COL0, COL1, COL2, COL3, ROW0, ROW1, ROW2, ROW3>
where
    COL0: InputPin<Error = PinE>,
    COL1: InputPin<Error = PinE>,
    COL2: InputPin<Error = PinE>,
    COL3: InputPin<Error = PinE>,
    ROW0: OutputPin<Error = PinE>,
    ROW1: OutputPin<Error = PinE>,
    ROW2: OutputPin<Error = PinE>,
    ROW3: OutputPin<Error = PinE>,
    PinE: Debug,
{
    fn new(
        col0: COL0,
        col1: COL1,
        col2: COL2,
        col3: COL3,
        row0: ROW0,
        row1: ROW1,
        row2: ROW2,
        row3: ROW3,
    ) -> KeyboardDriver<COL0, COL1, COL2, COL3, ROW0, ROW1, ROW2, ROW3> {
        KeyboardDriver {
            col0,
            col1,
            col2,
            col3,
            row0,
            row1,
            row2,
            row3,
        }
    }
}

trait GetPressedButtonImpl {
    fn get_pressed_button(&mut self) -> PressedButton;
}

impl<COL0, COL1, COL2, COL3, ROW0, ROW1, ROW2, ROW3, PinE> GetPressedButtonImpl
    for KeyboardDriver<COL0, COL1, COL2, COL3, ROW0, ROW1, ROW2, ROW3>
where
    COL0: InputPin<Error = PinE>,
    COL1: InputPin<Error = PinE>,
    COL2: InputPin<Error = PinE>,
    COL3: InputPin<Error = PinE>,
    ROW0: OutputPin<Error = PinE>,
    ROW1: OutputPin<Error = PinE>,
    ROW2: OutputPin<Error = PinE>,
    ROW3: OutputPin<Error = PinE>,
    PinE: Debug,
{
    fn get_pressed_button(&mut self) -> PressedButton {
        let mut pressedButton = PressedButton::None;
        self.row1.set_high();
        if self.col0.is_high().unwrap() {
            pressedButton = PressedButton::Left;
        }
        self.row1.set_low().unwrap();
        if pressedButton == PressedButton::None {
            self.row1.set_high().unwrap();
            if self.col1.is_high().unwrap() {
                pressedButton = PressedButton::Bottom;
            }
            self.row1.set_low().unwrap();
        }
        if pressedButton == PressedButton::None {
            self.row0.set_high().unwrap();
            if self.col1.is_high().unwrap() {
                pressedButton = PressedButton::Top;
            }
            self.row0.set_low().unwrap();
        }
        if pressedButton == PressedButton::None {
            self.row1.set_high().unwrap();
            if self.col2.is_high().unwrap() {
                pressedButton = PressedButton::Right;
            }
            self.row1.set_low().unwrap();
        }
        pressedButton
    }
}

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        {
            use core::mem::MaybeUninit;
            const HEAP_SIZE: usize = 1024;
            static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
            unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
        }
        //let _start = cortex_m_rt::heap_start() as usize;
        let _size = 1024;

        let rcc = p.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(160.mhz()).freeze();

        //let mut stdout = hio::hstdout().unwrap();

        let gpiog = p.GPIOG.split();
        let gpiof = p.GPIOF.split();
        let gpioc = p.GPIOC.split();
        let gpiod = p.GPIOD.split();
        let gpioe = p.GPIOE.split();

        let mut row0 = gpiog.pg2.into_push_pull_output();
        let mut row1 = gpiog.pg3.into_push_pull_output();
        let mut row2 = gpioe.pe2.into_push_pull_output();
        let mut row3 = gpioe.pe3.into_push_pull_output();

        let mut col0 = gpioe.pe4.into_pull_down_input();
        let mut col1 = gpioe.pe5.into_pull_down_input();
        let mut col2 = gpioe.pe6.into_pull_down_input();
        let mut col3 = gpioc.pc13.into_pull_down_input();

        let keyboard_driver = KeyboardDriver::new(col0, col1, col2, col3, row0, row1, row2, row3);

        let mut led0 = gpiog.pg13.into_push_pull_output();
        let mut led1 = gpiog.pg14.into_push_pull_output();

        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        let en = gpiof.pf10.into_push_pull_output();
        // ------------ SPI INTERFACE SETUP ----------------

        let spi = Spi::spi5(
            p.SPI5,
            (
                gpiof.pf7.into_alternate_af5(),
                hal::spi::NoMiso,
                gpiof.pf9.into_alternate_af5(),
            ),
            ili9341::spi::MODE,
            20_000_000.hz(),
            clocks,
        );

        let cs = gpioc.pc2.into_push_pull_output();
        let dc = gpiod.pd13.into_push_pull_output();

        let if_spi = ili9341::spi::SpiInterface::new(spi, cs, dc);

        led0.set_high().unwrap();
        led1.set_high().unwrap();
        let mut lcd = ili9341::Ili9341::new(if_spi, en, &mut delay).unwrap();
        lcd.set_orientation(ili9341::Orientation::Portrait)
            .unwrap();
        led0.set_low().unwrap();
        led1.set_low().unwrap();

        let mut api_realisation = ApiRealization::new(lcd, delay, keyboard_driver);

        let config = Config {
            cellSize:20,
            foodSize: 7,
            gameHeight: 16,
            gameWidth: 12,
            pointRadius: 1,
        };

        crate::falang::main::main(&config, &mut api_realisation);
    }
    loop {}
}
