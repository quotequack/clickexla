use std::{thread::{self, sleep}, time::Duration};
use rodio::{source::{SineWave, TriangleWave}, *};
use rand::Rng;
use std::collections::HashSet;
use gtk::prelude::*;
use gtk::*;

const APP_ID: &str = "org.quote.clickexla";
fn main() {
    thread::spawn(|| {
        // Frontend Init
        let app = Application::builder()
            .application_id(&*APP_ID)
            .build();
        app.connect_activate(build_ui);
        app.run();
    });
    // Sound init
    soundgen();
}
// Wave generator functions
fn wavemake(low: i32,high: i32) -> SineWave {
    let rng = rand::rng().random_range(low..high);
    let wave = SineWave::new(rng as f32);
    wave
}
fn wwavemake(low: i32,high: i32) -> TriangleWave {
    let rng = rand::rng().random_range(low..high);
    let wave = TriangleWave::new(rng as f32);
    wave
}
// Ui builder
fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("ClickExla")
        .build();
    window.present();
    let label = Label::new(Some("ClickExla is running in the background.\nClose this window to stop it."));
    window.set_child(Some(&label));
}
fn soundgen() {
    // Backend logic
    let mut pressed: HashSet<rdev::Key> = HashSet::new();
    let streamhandle = rodio::OutputStreamBuilder::open_default_stream().expect("oops");
    let callback = move |event: rdev::Event| {
        match event.event_type {
            rdev::EventType::KeyPress(key) => {
                if !pressed.contains(&key) {
                    pressed.insert(key);
                    let wave = wavemake(200,300);
                    streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                    sleep(Duration::from_millis(20));
                }
            }
            rdev::EventType::Wheel { delta_x: _, delta_y: _ } => {
                let wave = wavemake(400,500);
                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.10));
                sleep(Duration::from_millis(10));
            }
            rdev::EventType::ButtonPress(_button) => {
                let wave = wwavemake(200,400);
                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.30));
                sleep(Duration::from_millis(20));
            }
            rdev::EventType::KeyRelease(key) => {
                pressed.remove(&key);
            }
            _ => {}
        }
    };
    if let Err(error) = rdev::listen(callback) {
        println!("Error: {:?}", error);
    }
}
