// automatically generated by rust-bindgen 0.71.1

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::types::*;

extern "C" {
    /// Converts four variables (alpha, red, green, and blue) into a 32-bit (ARGB) variable that describes a color.
    ///
    ///
    /// Required System Capabilities: SystemCapability.Graphic.Graphic2D.NativeDrawing
    /// # Arguments
    ///
    /// * `alpha` - Indicates a variable that describes alpha. The value ranges from 0x00 to 0xFF.
    ///
    /// * `red` - Indicates a variable that describes red. The value ranges from 0x00 to 0xFF.
    ///
    /// * `green` - Indicates a variable that describes green. The value ranges from 0x00 to 0xFF.
    ///
    /// * `blue` - Indicates a variable that describes blue. The value ranges from 0x00 to 0xFF.
    ///
    /// # Returns
    ///
    /// * Returns a 32-bit (ARGB) variable that describes the color.
    ///
    /// Available since API-level: 8
    ///
    /// Version: 1.0
    pub fn OH_Drawing_ColorSetArgb(alpha: u32, red: u32, green: u32, blue: u32) -> u32;
}
