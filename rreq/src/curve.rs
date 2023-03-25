/**
 * Filter curve file. Created by Joshua Jeppson on 3/24/2023
 *
 * Licensed under the GPLv3
 * */

struct BandReject {
    // Center frequency
    center: f64,
    // Bandwidth
    bandwidth: f64,
    // Resonance (how wide this is)
    gain: f64
}

impl BandReject { 
    pub fn new(
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

    pub fn curve_at_index(&mut self, f : f32) -> f32 {
        // Use  Function for this
        return 1.0
    }
}

/**
 * Simple first-order lowpass/highpass filter
 * */
struct Filter {
    // Is this a lowpass or highpass filter?
    low: bool,
    // What is the frequency cutoff?
    cutoff: f64,
    // What is the bandwidth?
    bandwidth: f64
}

impl Filter {
    pub fn new(
        low: bool, 
        cutoff: f64, 
        time_constant: f64
    ) -> Self {
        Self {
            low,
            cutoff,
            time_constant
        }
    }

    pub fn curve_at_index(&mut self, f : f64) -> f32 {
        // Use first-order polynomial function for this
        let time_constant : f64 = 1.0;
        if self.low {
            // H(s) = 1 / (1 + Rs)
            return 1.0 / (1.0 + time_constant * f);
        }
        else {
            // H(s) = Rs / (1 + R*s)
            return time_constant / (1.0 + time_constant * f);
        }
    }

}

pub struct FiveBandEQ {
    // The lowpass filter
    lowpass : Filter,
    // The highpass filter
    highpass : Filter,
    // The five bands
    bands : [BandReject; 5]
}

impl FiveBandEQ {
    pub fn new() -> Self {
        Self {
            Filter::new(
                true,
            ),
            Filter::new(
                false,
            ),
            [BandReject::new(
                0.0,
                0.0,
                1.0
            ); 5]
        }
    }
}
