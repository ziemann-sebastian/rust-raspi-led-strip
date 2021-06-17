mod apa102;
mod ledstrip;
mod talking_led;

use apa102::APA102;
use ledstrip::LEDStrip;
use std::{thread, time};
use talking_led::TalkingLED;

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

fn main() {
    let led = APA102::new().unwrap();
    let talking_led = TalkingLED::new().unwrap();

    toggle_led(talking_led);
}
