/**
 * Filter curve file. Created by Joshua Jeppson on 3/24/2023
 *
 * Licensed under the GPLv3
 * */

struct BandReject {
    // Buffer size
    buf_size: i32,
    // Center frequency
    center: f64,
    // Bandwidth
    bandwidth: f64,
    // Resonance (how wide this is)
    gain: f64
}

impl BandReject { 
    pub fn new(
        buf_size: i32,
        center: f64,
        bandwidth: f64,
        gain: f64
    ) -> Self {
        Self {
            center,
            bandwidth,
            gain
        }
    }

    pub curve_at_index(index : u32) -> f64 {
        // Use  Function for this
    }
}

/**
 * Simple first-order lowpass/highpass filter
 * */
struct Filter {
    // Buffer size
    buf_size: i32,
    // Is this a lowpass or highpass filter?
    low: bool,
    // What is the frequency cutoff?
    cutoff: f64,
    // What is the bandwidth?
    bandwidth: f64
}

impl Filter {
    pub fn new(
        buf_size: i32, 
        low: bool, 
        cutoff: f64, 
        time_constant: f64
    ) -> Self {
        Self {
            buf_size,
            low,
            cutoff
        }
    }

    pub curve_at_index(index : u32) -> f64 {
        // Use first-order polynomial function for this
        let s : f64 = index / Self.buf_size; // TODO: cast to f64
        let time_constant : f64 = 1.0;
        if (Self.low) {
            // H(s) = 1 / (1 + Rs)
            return 1 / (1 + time_constant * s);
        }
        else {
            // H(s) = Rs / (1 + R*s)
            return time_constant / (1 + time_constant * s);
        }
    }

}

pub struct FiveBandEQ {
    // Buffer size
    buf_size: i32,
    // The lowpass filter
    lowpass : Filter,
    // The highpass filter
    highpass : Filter,
    // The five bands
    bands : [BandReject: 5]
}

impl FiveBandEq {
    pub fn new(buf_size : i32) -> Self {
        Self {
            buf_size,
            Filter::new(
                buf_size,
                true,
            ),
            Filter::new(
                buf_size,
                false,
            ),
            [BandReject::new(
                buf_size,
                0.0,
                0.0,
                1.0
            ); 5]
        }
    }
}
