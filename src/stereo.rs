//! 
//! Utility functions to assist with stereo panning.
//!


pub const MINUS_3_DECIBELS: f32 = 0.708;
pub const MINUS_6_DECIBELS: f32 = 0.501;


/// Return the amp for left and right channels according to a given pan between -1.0 and 1.0.
/// `pan` is assumed to always be between -1.0 and 1.0 (L and R) and will not perform any checks.
/// The minimum and maximum amplitude of each channel will be between 0.0 and 1.0 respectively.
/// A centered pan (0.0) will leave both channels at -3db (0.708).
/// A hard pan to either side will result in the weighted channel at 0db (1.0).
pub fn pan(pan: f32) -> [f32; 2] {
    const CENTRE: f32 = 0.0;
    if pan == CENTRE {
        [MINUS_3_DECIBELS, MINUS_3_DECIBELS]
    } else {
        // Use pan^2 in order for a smoother, continuous, "bowl" shaped pan.
        let pan_pow2 = pan.powf(2.0);

        let less = MINUS_3_DECIBELS - pan_pow2 * MINUS_3_DECIBELS;
        let more = MINUS_3_DECIBELS + pan_pow2 * (1.0 - MINUS_3_DECIBELS);

        // If the pan is greater than 0.0 we pan to the right.
        if pan > 0.0 { [less, more] }
        // If the pan is less than 0.0 we pan to the left.
        else         { [more, less] }
    }
}

