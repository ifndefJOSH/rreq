/**
 * Main file for RREQ. Created by Josh Jeppson on 3/24/2023
 *
 * Licensed under the GPLv3
 * */
use std::io;
use crate::curve::FiveBandEQ;

mod gui;
mod curve;

use rustfft::{FftPlanner, num_complex::Complex};
use rustfft::num_traits::Pow;

/**
 * This function should modify the mutable input channel it is given, based on the
 * values of the equalizer parameters
 *
 * @param in_channel The input channel
 * */
fn perform_equalization<'a>(in_channel : &[f32], out_channel : &mut[f32], sampling_freq : usize) { //  -> &'a [f32] {
    
    // Steps:
    // Perform FFT on in_channel
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(in_channel.len());
    let mut buffer = vec![Complex{ re: 0.0, im: 0.0 }; in_channel.len()];
    for i in 0..in_channel.len() {
        buffer[i].re = in_channel[i];
    }
    fft.process(&mut buffer);
    // Apply filter curve in f domain
    for i in 0..in_channel.len() {
        // Frequency from the nyquist-shannon sampling theorem
        let f : f32 = i as f32 * (sampling_freq as f32 ) / in_channel.len() as f32;
        // println!("Freq: {}", f);
        // TODO: Actual logic:
        if f > 500.0 {
            buffer[i].re = 0.0;
            buffer[i].im = 0.0;
        }
        // else {
        //    buffer[i].re = 1.0;
        //    buffer[i].im = 1.0;
        //}
    }
    // inverse FFT from filtered
    // let mut planner2 = FftPlanner::<f32>::new();
    //let ifft = planner2.plan_fft_forward(in_channel.len());
    //let mut ibuffer = vec![Complex{ re: 0.0, im: 0.0 }; in_channel.len()];
    //ifft.process(&mut ibuffer);
    fft.process(&mut buffer);
    for i in 0..in_channel.len() {
        buffer[i].re /= in_channel.len() as f32;
        buffer[i].im /= in_channel.len() as f32;
        // println!("output {} and {}", buffer[i].re, buffer[i].im);
        out_channel[i] = buffer[i].re; // ((buffer[i].re).pow(2) as f32 + (buffer[i].im).pow(2) as f32).sqrt() as f32;
        if out_channel[i] > in_channel[i] * 1.5 {
            out_channel[i] = in_channel[i] * 1.5;
        }

    }
}

fn create_jack_client() {
    // Create Jack client
    let (client, _status) = jack::Client::new(
        "RREQ",
        jack::ClientOptions::NO_START_SERVER,
    ).unwrap();
    
    // Register Two input ports and two output ports
    let mut in_l = client
        .register_port("rreq_in_l", jack::AudioIn::default())
        .unwrap();
    let mut in_r = client
        .register_port("rreq_in_r", jack::AudioIn::default())
        .unwrap();
    let mut out_l = client
        .register_port("rreq_out_l", jack::AudioOut::default())
        .unwrap();
    let mut out_r = client
        .register_port("rreq_out_r", jack::AudioOut::default())
        .unwrap();

    // Create a callback for jack to handle the audio coming in from
    let process_callback = move |client: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let sample_rate = client.sample_rate();
        let mut out_l_p = out_l.as_mut_slice(ps);
        let mut out_r_p = out_r.as_mut_slice(ps);
        let mut in_l_p = in_l.as_slice(ps);
        let mut in_r_p = in_r.as_slice(ps);
        perform_equalization(in_l_p, &mut out_l_p, sample_rate);
        perform_equalization(in_r_p, &mut out_r_p, sample_rate);
        // out_l_p.clone_from_slice(in_l_p);
        // out_r_p.clone_from_slice(in_r_p);
        jack::Control::Continue
    };
    let process = jack::ClosureProcessHandler::new(process_callback);

    // Activate the jack client on program activation. This starts EQ processing
    let active_client = client.activate_async(Notifications, process).unwrap();

    // Create window
    println!("[RREQ]: Creating main window");
    let mut eq : FiveBandEQ = FiveBandEQ::new();
    gui::create_window();

    // Wait to quit until user input
    println!("Press [ENTER] to exit RREQ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();
    
    // Deactivate the client
    active_client.deactivate().unwrap();
}

fn main() {
    create_jack_client();
}

// Print JACK notifications
struct Notifications;

impl jack::NotificationHandler for Notifications {
    fn thread_init(&self, _: &jack::Client) {
        println!("[JACK]: Thread has been initialized.");
        println!("[RREQ]: Note that not all notifications from JACK are printed.");
    }
    
    fn shutdown(&mut self, status: jack::ClientStatus, reason: &str) {
        println!("[JACK]: Shutdown with status {status:?} due to \"{reason}\"");
    }
}
