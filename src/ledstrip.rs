pub trait LEDStrip {
    /// Set the RGB value and optionally brightness of all pixels.
    /// # Arguments
    ///
    /// * `r` - Amount of red: 0 to 255
    /// * `g` - Amount of green: 0 to 255
    /// * `b` - Amount of blue: 0 to 255
    /// * `brightness` - Brightness: 0.0 to 1.0
    fn set_all(&mut self, r: u8, g: u8, b: u8, brightness: f32);
    /// Set the RGB value, and optionally brightness, of a single pixel.
    /// # Arguments
    ///
    /// * `x` - The horizontal position of the pixel: 0 to 7
    /// * `r` - Amount of red: 0 to 255
    /// * `g` - Amount of green: 0 to 255
    /// * `b` - Amount of blue: 0 to 255
    /// * `brightness` - Brightness: 0.0 to 1.0
    fn set_pixel(&mut self, x: usize, r: u8, g: u8, b: u8, brightness: f32);
    /// Set the brightness of all pixels.
    /// # Arguments
    ///
    /// * `brightness` - Brightness: 0.0 to 1.0.
    fn set_brightness(&mut self, brightness: f32);
    /// Clear the pixel buffer.
    fn clear(&mut self);
    /// Output the buffer.
    fn show(&mut self) -> Result<(), String>;
}
