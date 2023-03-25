/**
 * Main file for RREQ. Created by Josh Jeppson on 3/24/2023
 *
 * Licensed under the GPLv3
 * */
use std::io;

mod gui;

// use rustfft::{FftPlanner, num_complex::Complex};

/**
 * This function should modify the mutable input channel it is given, based on the
 * values of the equalizer parameters
 *
 * @param in_channel The input channel
 * */
fn perform_equalization<'a>(in_channel : &'a [f32]) { //  -> &'a [f32] {
    
    // TODO: perform equalization based on curve.
    // Steps:
        // Perform FFT on in_channel
        // Apply filter curve in f domain
        // inverse FFT from filtered
    // let mut output : Box<f32>::new();
    // return output.into_boxed_slice();
}

fn create_jack_client() {
    // Create Jack client
    let (client, _status) = jack::Client::new(
        "reasonable_realtime_rust_equalizer",
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
    let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let mut out_l_p = out_l.as_mut_slice(ps);
        let mut out_r_p = out_r.as_mut_slice(ps);
        let mut in_l_p = in_l.as_slice(ps);
        let mut in_r_p = in_r.as_slice(ps);
        perform_equalization(&mut in_l_p);
        perform_equalization(&mut in_r_p);
        out_l_p.clone_from_slice(in_l_p);
        out_r_p.clone_from_slice(in_r_p);
        jack::Control::Continue
    };
    let process = jack::ClosureProcessHandler::new(process_callback);

    // Activate the jack client on program activation. This starts EQ processing
    let active_client = client.activate_async(Notifications, process).unwrap();

    // Create window
    println!("[RREQ]: Creating main window");
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
