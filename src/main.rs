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
    let clickopt = ["Sinewave", "TriangleWave", "SquareWave"];
    let clistr = StringList::new(&clickopt);
    let window = ApplicationWindow::builder()
        .application(app)
        .title("ClickExla")
        .build();
    let clabel = Label::builder()
        .label("Click Sound")
        .build();
    let blabel = Label::builder()
        .label("Button Sound")
        .build();
    let wheellabel = Label::builder()
        .label("Wheel Sound")
        .build();
    let clickoptions = DropDown::builder()
        .model(&clistr)
        .selected(1)
        .build();
    let buttonoptions = DropDown::builder()
        .model(&clistr)
        .selected(0)
        .build();
    let wheeloptions = DropDown::builder()
        .model(&clistr)
        .selected(0)
        .build();
    let  maxhertzbtn = Entry::builder()
        .placeholder_text("Max Hertz")
        .text("300")
        .build();
    let  minhertzbtn = Entry::builder()
        .placeholder_text("Min Hertz")
        .text("200")
        .build();
    let maxhertzclck = Entry::builder()
        .placeholder_text("Max Hertz")
        .text("400")
        .build();
    let minhertzclck = Entry::builder()
        .placeholder_text("Min Hertz")
        .text("200")
        .build();
    let maxhertzwhe = Entry::builder()
        .placeholder_text("Max Hertz")
        .text("500")
        .build();
    let minhertzwhe = Entry::builder()
        .placeholder_text("Min Hertz")
        .text("400")
        .build();
    let main = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(3)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let btn = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(1)
        .build();
    btn.append(&blabel);
    btn.append(&buttonoptions);
    btn.append(&minhertzbtn);
    btn.append(&maxhertzbtn);
    let clk = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(1)
        .build();
    clk.append(&clabel);
    clk.append(&clickoptions);
    clk.append(&minhertzclck);
    clk.append(&maxhertzclck);
    let whe = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(1)
        .build();
    whe.append(&wheellabel);
    whe.append(&wheeloptions);
    whe.append(&minhertzwhe);
    whe.append(&maxhertzwhe);
    main.append(&btn);
    main.append(&clk);
    main.append(&whe);
    window.set_child(Some(&main));
    window.present();
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
