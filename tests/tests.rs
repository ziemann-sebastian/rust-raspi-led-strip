use rust_raspi_led_strip::*;
use std::{thread, time};

fn toggle_led<T>(mut led: T)
where
    T: LEDStrip,
{
    led.set_all(255, 0, 0, 1.0);
    led.show().unwrap();

    thread::sleep(time::Duration::from_millis(1000));

    led.clear();
    led.show().unwrap();
}

#[test]
fn first_test() {
    //let led = APA102::new(50);
    let talking_led = TalkingLED::new();

    //toggle_led(led);
    toggle_led(talking_led);
}
