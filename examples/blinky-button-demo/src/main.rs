#![no_main]
#![no_std]

use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::OutputPin;
use nrf52832_hal as hal;
use nrf52832_hal::gpio;
use rtt_target::{rprintln, rtt_init_print};

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

struct Led {
    led: gpio::Pin<gpio::Output<gpio::PushPull>>,
    button: gpio::Pin<gpio::Input<gpio::PullUp>>,
    high: bool,
}

macro_rules! build_led {
    (
        $led_pin:expr, $button_pin:expr
    ) => {
        Led {
            led: $led_pin.into_push_pull_output(gpio::Level::Low).degrade(),
            button: $button_pin.into_pullup_input().degrade(),
            high: false,
        }
    };
}

impl Led {
    fn refresh(&mut self) {
        let high = self.button.is_high().unwrap();
        if self.high != high {
            if high {
                self.led.set_high().unwrap();
            } else {
                self.led.set_low().unwrap();
            }
            rprintln!(
                "{} now {}",
                self.led.pin(),
                if high { "high" } else { "low" }
            );
            self.high = high;
        }
    }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    let p = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let mut leds = [
        build_led!(port0.p0_17, port0.p0_13),
        build_led!(port0.p0_18, port0.p0_14),
        build_led!(port0.p0_19, port0.p0_15),
        build_led!(port0.p0_20, port0.p0_16),
    ];

    rprintln!("Blinky button demo starting");
    loop {
        for i in 0..leds.len() {
            leds[i].refresh();
        }
    }
}
