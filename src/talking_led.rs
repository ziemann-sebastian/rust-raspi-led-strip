use super::ledstrip::LEDStrip;

pub struct TalkingLED {}

impl TalkingLED {
    pub fn new() -> Self {
        Self {}
    }
}

impl LEDStrip for TalkingLED {
    fn set_pixel(&mut self, x: usize, r: u8, g: u8, b: u8, brightness: f32) {
        println!("set_pixel() called");
    }
    fn set_all(&mut self, r: u8, g: u8, b: u8, brightness: f32) {
        println!("set_all() called");
    }
    fn show(&mut self) -> Result<(), String> {
        println!("show() called");
        Ok(())
    }

    fn clear(&mut self) {
        println!("clear() called");
    }

    fn set_brightness(&mut self, brightness: f32) {
        println!("set_brightness() called");
    }
}
