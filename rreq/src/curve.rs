/**
 * Filter curve file. Created by Joshua Jeppson on 3/24/2023
 *
 * Licensed under the GPLv3
 * */

pub struct BandReject {
    // Center frequency
    pub center: f64,
    // Bandwidth
    pub bandwidth: f64,
    // Resonance (how wide this is)
    pub gain: f64
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
        let f2 = dbg!(f as f64);
        if f2 < self.center + (self.bandwidth / 2.0) 
            && f2 > self.center - (self.bandwidth / 2.0
        ) {
            return dbg!((self.gain / 100.0) as f32); // Gain is scaled by 100
        }
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
    time_constant: f32
}

impl Filter {
    pub fn new(
        low: bool, 
        cutoff: f64, 
        time_constant: f32
    ) -> Self {
        Self {
            low,
            cutoff,
            time_constant
        }
    }

    pub fn curve_at_index(&mut self, f : f32) -> f32 {
        // Use first-order polynomial function for this
        let time_constant : f32 = 1.0;
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
    // bands : [BandReject; 5]
    pub band0 : BandReject,
    pub band1 : BandReject,
    pub band2 : BandReject,
    pub band3 : BandReject,
    pub band4 : BandReject,
}

impl FiveBandEQ {
    pub fn new() -> Self {
        Self {
            lowpass : Filter::new(
                true,
                0.0,
                0.0
            ),
            highpass : Filter::new(
                false,
                4000.0,
                0.0
            ),
            // band0
            band0 : BandReject::new(0.0, 0.0, 100.0),
            // band1
            band1 : BandReject::new(0.0, 0.0, 100.0),
            // band2
            band2 : BandReject::new(0.0, 0.0, 100.0),
            // band3
            band3 : BandReject::new(0.0, 0.0, 100.0),
            // band4
            band4 : BandReject::new(0.0, 0.0, 100.0),
        }
    }
}
