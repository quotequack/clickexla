use std::{thread::sleep, time::Duration};
use rodio::{source::SineWave, *};
use rand::Rng;

fn main() {
    let streamhandle = rodio::OutputStreamBuilder::open_default_stream().expect("oops");
    let callback = move |event: rdev::Event| {
        match event.event_type {
            rdev::EventType::KeyPress(_key) => {
                let wave = wavemake(400);
                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                sleep(Duration::from_millis(20));
            }
            rdev::EventType::Wheel { delta_x: _, delta_y: _ } => {
                let wave = wavemake(600);
                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.10));
                sleep(Duration::from_millis(10));
            }
            _ => {}
        }
    };
    if let Err(error) = rdev::listen(callback) {
        println!("Error: {:?}", error);
    }
}
fn wavemake(high: i32) -> SineWave {
    let rng = rand::rng().random_range(100..high);
    let wave = SineWave::new(rng as f32);
    wave
}

