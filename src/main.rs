#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

mod debug;

#[rtic::app(
    device = rp_pico::hal::pac,
    dispatchers = [PWM_IRQ_WRAP]
)]
mod app {
    use rp_pico::hal;
    use rp_pico::hal::pac;
    use rp_pico::hal::prelude::*;

    use embedded_hal::digital::v2::{OutputPin, ToggleableOutputPin};

    use rtic_monotonics::rp2040::*;

    rtic_monotonics::make_rp2040_monotonic_handler!();

    #[shared]
    struct Shared {
        led: hal::gpio::Pin<hal::gpio::bank0::Gpio25, hal::gpio::PushPullOutput>,
    }

    #[local]
    struct Local {}

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local) {
        defmt::info!("Booting Ampel");

        let mut watchdog = hal::Watchdog::new(ctx.device.WATCHDOG);
        let clocks = hal::clocks::init_clocks_and_plls(
            rp_pico::XOSC_CRYSTAL_FREQ,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
            &mut watchdog,
        )
        .ok()
        .unwrap();

        Timer::start(ctx.device.TIMER, &mut ctx.device.RESETS);

        let sio = hal::Sio::new(ctx.device.SIO);
        let pins = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );

        let mut led = pins.led.into_push_pull_output();

        led.set_low().unwrap();

        blink::spawn().unwrap();

        (
            Shared {
                led,
            },
            Local {},
        )
    }

    #[task(priority = 1, shared = [led])]
    async fn blink(ctx: blink::Context) {
        let mut led = ctx.shared.led;

        loop {
            led.lock(|l| l.toggle().unwrap());
            Timer::delay(200.millis()).await;
        }
    }
}
