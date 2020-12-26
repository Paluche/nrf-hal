#![no_main]
#![no_std]

// Not tested
// #[cfg(feature = "51")]
// use nrf51_hal as hal;

// Not testable
//#[cfg(feature = "52810")]
//use nrf52810_hal as hal;

// Not testable
// #[cfg(feature = "52811")]
// use nrf52811_hal as hal;

#[cfg(feature = "pca10040")]
use nrf52832_hal as hal;

// Not tested
// #[cfg(feature = "52840")]
// use nrf52840_hal as hal;

use {
    core::{
        panic::PanicInfo,
        sync::atomic::{compiler_fence, Ordering},
    },
    embedded_hal::digital::v2::{InputPin, OutputPin},
    hal::{
        gpio,
        gpio::{Input, Level, Output, Pin, PullUp, PushPull},
    },
    rtic::app,
    rtt_target::{rprintln, rtt_init_print},
};

pub struct Board {
    led_1: Pin<Output<PushPull>>,
    led_2: Pin<Output<PushPull>>,
    led_3: Pin<Output<PushPull>>,
    led_4: Pin<Output<PushPull>>,
    btn_1: Pin<Input<PullUp>>,
    btn_2: Pin<Input<PullUp>>,
    btn_3: Pin<Input<PullUp>>,
    btn_4: Pin<Input<PullUp>>,
}

#[cfg(feature = "pca10040")]
impl Board {
    fn init(ctx: init::Context) -> Self {
        let port_0 = gpio::p0::Parts::new(ctx.device.P0);
        Self {
            led_1: port_0.p0_17.into_push_pull_output(Level::Low).degrade(),
            led_2: port_0.p0_18.into_push_pull_output(Level::Low).degrade(),
            led_3: port_0.p0_19.into_push_pull_output(Level::Low).degrade(),
            led_4: port_0.p0_20.into_push_pull_output(Level::Low).degrade(),
            btn_1: port_0.p0_13.into_pullup_input().degrade(),
            btn_2: port_0.p0_14.into_pullup_input().degrade(),
            btn_3: port_0.p0_15.into_pullup_input().degrade(),
            btn_4: port_0.p0_16.into_pullup_input().degrade(),
        }
    }
}

macro_rules! set_led {
    ($i:expr, $led:expr, $led_idx:expr) => {
        if $i & (1 << ($led_idx - 1)) != 0 {
            $led.set_low().unwrap();
        } else {
            $led.set_high().unwrap();
        }
    };
}

#[app(device=crate::hal::pac, peripherals=true)]
const APP: () = {
    struct Resources {
        board: Board,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("init");
        let mut board: Board = Board::init(ctx);

        for i in 0..10 {
            set_led!(i, board.led_1, 1);
            set_led!(i, board.led_2, 2);
            set_led!(i, board.led_3, 3);
            set_led!(i, board.led_4, 4);
            for _ in 0..32678 {
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                cortex_m::asm::nop();
                cortex_m::asm::nop();
            }
        }

        if board.btn_1.is_low().unwrap() {
            board.led_1.set_high().unwrap();
        }
        if board.btn_2.is_low().unwrap() {
            board.led_2.set_high().unwrap();
        }
        if board.btn_3.is_low().unwrap() {
            board.led_3.set_high().unwrap();
        }
        if board.btn_4.is_low().unwrap() {
            board.led_4.set_high().unwrap();
        }

        init::LateResources { board }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        rprintln!("idle");

        loop {
            cortex_m::asm::wfi();
        }
    }
};

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    cortex_m::interrupt::disable();
    rprintln!("{}", info);
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
