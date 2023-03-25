/**
 * GUI file. Created by Joshua Jeppson on 3/24/2023
 *
 * Licensed under the GPLv3
 * */
use iced::widget::{column, row, container, vertical_slider, text};
use iced::{Element, Length, Sandbox, Settings};

use crate::curve::FiveBandEQ;

pub fn create_window(eq : &mut FiveBandEQ) {
   EQWindow::run(Settings::default());
}

#[derive(Debug, Clone)]
pub enum Message {
    Band0Changed(f64),
    Band1Changed(f64),
    Band2Changed(f64),
    Band3Changed(f64),
    Band4Changed(f64)
}

pub struct EQWindow<'a> {
    eq : &'a mut FiveBandEQ
}

// See https://github.com/iced-rs/iced/blob/master/examples/slider/src/main.rs

impl<'a> Sandbox for EQWindow<'a> {
    type Message = Message;

    fn new(eq : &'a mut FiveBandEQ) -> EQWindow<'a> {
        EQWindow {
           eq 
        }
    }

    fn title(&self) -> String {
        String::from("Reasonable Rust Equalizer")
    }

    // TODO: Update
    fn update(&mut self, message : Message) {
        match message {
           Message::Band0Changed(value) => {
                self.eq.band0.gain = value;
            }
            
            Message::Band1Changed(value) => {
                self.eq.band0.gain = value;
            }
            
            Message::Band2Changed(value) => {
                self.eq.band0.gain = value;
            }
            
            Message::Band3Changed(value) => {
                self.eq.band0.gain = value;
            }
            
            Message::Band4Changed(value) => {
                self.eq.band0.gain = value;
            } 
        }
    }

    fn view(&self) -> Element<Message> {
        let band0_value = self.eq.band0.gain;
        let band1_value = self.eq.band1.gain;
        let band2_value = self.eq.band2.gain;
        let band3_value = self.eq.band3.gain;
        let band4_value = self.eq.band4.gain;
        
        let band0_slider = 
            container(vertical_slider(0.0..=1.5, band0_value, Message::Band0Changed))
                .height(200);
        let band1_slider = 
            container(vertical_slider(0.0..=1.5, band1_value, Message::Band1Changed))
                .height(200);
        let band2_slider = 
            container(vertical_slider(0.0..=1.5, band2_value, Message::Band2Changed))
                .height(200);
        let band3_slider = 
            container(vertical_slider(0.0..=1.5, band3_value, Message::Band3Changed))
                .height(200);
        let band4_slider = 
            container(vertical_slider(0.0..=1.5, band4_value, Message::Band4Changed))
                .height(200);

        container(
            row![
                container(band0_slider).width(Length::Fill).center_x(),
                container(band1_slider).width(Length::Fill).center_x(),
                container(band2_slider).width(Length::Fill).center_x(),
                container(band3_slider).width(Length::Fill).center_x(),
                container(band4_slider).width(Length::Fill).center_x()
            ]
            .spacing(25)
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}
