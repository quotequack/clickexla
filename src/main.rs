use rodio::{source::{SineWave, TriangleWave, SquareWave}, *};
use rand::Rng;
use gtk::{prelude::*, subclass::window};
use gtk::*;
use std::{fs, time::Duration, collections::HashSet, thread::{self, sleep}, path::Path, error::Error};
use serde::{Deserialize, Serialize};
use serde_json;
#[allow(unused)]
#[allow(deprecated)]

#[derive(Serialize, Deserialize, Debug)]
struct Device {
    enabled: bool,
    mahertz: i32,
    mihertz: i32,
    wave: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    clck: Device,
    btn: Device,
    whee: Device,
}

const APP_ID: &str = "org.quote.clickexla";
fn main() {
    // Frontend Init
    let app = Application::builder()
        .application_id(&*APP_ID)
        .build();
    app.connect_activate(build_ui);
    app.run();
}
// Wave generator functions
fn swavemake(low: i32,high: i32) -> SineWave {
    let rng = rand::rng().random_range(low..high);
    let wave = SineWave::new(rng as f32);
    wave
}
fn twavemake(low: i32,high: i32) -> TriangleWave {
    let rng = rand::rng().random_range(low..high);
    let wave = TriangleWave::new(rng as f32);
    wave
}
fn sqwavemake(low: i32,high: i32) -> SquareWave {
    let rng = rand::rng().random_range(low..high);
    let wave = SquareWave::new(rng as f32);
    wave
}

fn build_ui(app: &Application) {
    // Load settings
    let settings = load_settings("settings.json").unwrap();
    // Ui builder
    let clickopt = ["Sinewave", "TriangleWave", "SquareWave"];
    let clistr = StringList::new(&clickopt);
    let window= ApplicationWindow::builder()
        .application(app)
        .title("ClickExla")
        .maximized(false)
        .icon_name("clickexla")
        .build();
    let execute = Button::builder()
        .label("Execute")
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
        .selected(settings.clck.wave)
        .build();
    let buttonoptions = DropDown::builder()
        .model(&clistr)
        .selected(settings.btn.wave)
        .build();
    let wheeloptions = DropDown::builder()
        .model(&clistr)
        .selected(settings.whee.wave)
        .build();
    let  maxhertzbtn = Entry::builder()
        .placeholder_text("Max Hertz")
        .text(format!("{}",settings.btn.mahertz))
        .build();
    let  minhertzbtn = Entry::builder()
        .placeholder_text("Min Hertz")
        .text(format!("{}",settings.btn.mihertz))
        .build();
    let maxhertzclck = Entry::builder()
        .placeholder_text("Max Hertz")
        .text(format!("{}",settings.clck.mahertz))
        .build();
    let minhertzclck = Entry::builder()
        .placeholder_text("Min Hertz")
        .text(format!("{}",settings.clck.mihertz))
        .build();
    let maxhertzwhe = Entry::builder()
        .placeholder_text("Max Hertz")
        .text(format!("{}",settings.whee.mahertz))
        .build();
    let minhertzwhe = Entry::builder()
        .placeholder_text("Min Hertz")
        .text(format!("{}",settings.whee.mihertz))
        .build();
    let enable_clck = CheckButton::builder()
        .label("Enable")
        .build();
    enable_clck.set_active(settings.clck.enabled);
    let enable_btn = CheckButton::builder()
        .label("Enable")
        .build();
    enable_btn.set_active(settings.btn.enabled);
    let enable_whee = CheckButton::builder()
        .label("Enable")
        .build();
    enable_whee.set_active(settings.whee.enabled);
    let main = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(3)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    let btn = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(1)
        .build();
    btn.append(&blabel);
    btn.append(&buttonoptions);
    btn.append(&minhertzbtn);
    btn.append(&maxhertzbtn);
    btn.append(&enable_btn);
    let clk = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(1)
        .build();
    clk.append(&clabel);
    clk.append(&clickoptions);
    clk.append(&minhertzclck);
    clk.append(&maxhertzclck);
    clk.append(&enable_clck);
    let whe = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(1)
        .build();
    whe.append(&wheellabel);
    whe.append(&wheeloptions);
    whe.append(&minhertzwhe);
    whe.append(&maxhertzwhe);
    whe.append(&enable_whee);
    main.append(&btn);
    main.append(&clk);
    main.append(&whe);
    main.append(&execute);
    window.set_child(Some(&main));
    let windows = window.clone();
    execute.connect_clicked(move |_| {
        windows.hide();
        soundgen(
            clickoptions.clone(),
            buttonoptions.clone(),
            wheeloptions.clone(),
            minhertzclck.clone(),
            maxhertzclck.clone(),
            minhertzbtn.clone(),
            maxhertzbtn.clone(),
            minhertzwhe.clone(),
            maxhertzwhe.clone(),
            enable_clck.clone(),
            enable_btn.clone(),
            enable_whee.clone()
        );
    });
    window.present();
}

fn soundgen(clickoptions: DropDown, 
    buttonoptions: DropDown, 
    wheeloptions: DropDown, 
    minhertzclck: Entry, 
    maxhertzclck: Entry, 
    minhertzbtn: Entry, 
    maxhertzbtn: Entry, 
    minhertzwhe: Entry, 
    maxhertzwhe: Entry,
    enable_clck: CheckButton,
    enable_btn: CheckButton,
    enable_whee: CheckButton
) {
    // Read info
    let clckopt:u32 = clickoptions.selected();
    let btnopt:u32 = buttonoptions.selected();
    let wheopt:u32 = wheeloptions.selected();
    let clckmin: i32 = minhertzclck.text().parse().expect("Please enter a valid number(CLCKMIN)");
    let clckmax: i32 = maxhertzclck.text().parse().expect("Please enter a valid number(CLCKMAX)");
    let btnmin: i32 = minhertzbtn.text().parse().expect("Please enter a valid number(BTNMIN)");
    let btnmax: i32 = maxhertzbtn.text().parse().expect("Please enter a valid number(BTNMAX)");
    let whemin: i32 = minhertzwhe.text().parse().expect("Please enter a valid number(WHEMIN)");
    let whemax: i32 = maxhertzwhe.text().parse().expect("Please enter a valid number(WHEMAX)");
    let enaclck: bool = enable_clck.is_active();
    let enabtn: bool = enable_btn.is_active();
    let enawhe: bool = enable_whee.is_active();
    // Save to json
    save_json(clckopt, btnopt, wheopt, clckmin, clckmax, btnmin, btnmax, whemin, whemax, enaclck, enabtn, enawhe);
    thread::spawn(move || {
        // Backend logic
        let mut pressed: HashSet<rdev::Key> = HashSet::new();
        let streamhandle = rodio::OutputStreamBuilder::open_default_stream().expect("oops");
        let callback = move |event: rdev::Event| {
            match event.event_type {
                rdev::EventType::KeyPress(key) => {
                    if enabtn == true {
                        if !pressed.contains(&key) {
                            pressed.insert(key);
                            match btnopt {
                                0=>{
                                    let wave=swavemake(btnmin, btnmax);
                                    streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                    sleep(Duration::from_millis(20));
                                },
                                1=>{
                                    let wave=twavemake(btnmin, btnmax);
                                    streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                    sleep(Duration::from_millis(20));
                                },
                                2=>{
                                    let wave=sqwavemake(btnmin, btnmax);
                                    streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                    sleep(Duration::from_millis(20));
                                },
                                _=>{
                                    let wave=swavemake(btnmin, btnmax);
                                    streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                    sleep(Duration::from_millis(20));
                                    println!("Error in button sound selection, defaulting to SineWave")
                                },
                            }
                        }
                    }
                }
                rdev::EventType::Wheel { delta_x: _, delta_y: _ } => {
                    if enawhe == true {
                        match btnopt {
                            0=>{
                                let wave=swavemake(whemin, whemax);
                                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                sleep(Duration::from_millis(20));
                            },
                            1=>{
                                let wave=twavemake(whemin, whemax);
                                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                sleep(Duration::from_millis(20));
                            },
                            2=>{
                                let wave=sqwavemake(whemin, whemax);
                                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                sleep(Duration::from_millis(20));
                            },
                            _=>{
                                let wave=swavemake(whemin, whemax);
                                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                sleep(Duration::from_millis(20));
                                println!("Error in button sound selection, defaulting to SineWave")
                            },
                        }
                    }
                }
                rdev::EventType::ButtonPress(_button) => {
                    if enaclck == true {
                        match clckopt {
                            0=>{
                                let wave=swavemake(clckmin, clckmax);
                                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                sleep(Duration::from_millis(20));
                            },
                            1=>{
                                let wave=twavemake(clckmin, clckmax);
                                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                sleep(Duration::from_millis(20));
                            },
                            2=>{
                                let wave=sqwavemake(clckmin, clckmax);
                                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                sleep(Duration::from_millis(20));
                            },
                            _=>{
                                let wave=swavemake(clckmin, clckmax);
                                streamhandle.mixer().add(wave.take_duration(Duration::from_millis(20)).amplify(0.20));
                                sleep(Duration::from_millis(20));
                                println!("Error in button sound selection, defaulting to SineWave")
                            },
                        }
                    }
                }
                rdev::EventType::KeyRelease(key) => {
                    pressed.remove(&key);
                }
                _ => {}
            }
        };
        if let Err(error) = rdev::listen(callback) {
            eprintln!("error: {:?}", error);
        }
    });
}
fn save_json (
    clckopt: u32, 
    btnopt: u32, 
    wheopt: u32, 
    clckmin: i32, 
    clckmax: i32, 
    btnmin: i32, 
    btnmax: i32, 
    whemin: i32, 
    whemax: i32,
    enaclck: bool,
    enabtn: bool,
    enawhe: bool
) {
    // Save it!! :D
    let data = format!(r#"{{
        "clck": {{
            "enabled": {},
            "mahertz": {},
            "mihertz": {},
            "wave": {}
        }},
        "btn": {{
            "enabled": {},
            "mahertz": {},
            "mihertz": {},
            "wave": {}
        }},
        "whee": {{
            "enabled": {},
            "mahertz": {},
            "mihertz": {},
            "wave": {}
        }}
    }}"#,
            enaclck, clckmax, clckmin, clckopt,
            enabtn, btnmax, btnmin, btnopt,
            enawhe, whemax, whemin, wheopt
        );
    fs::write("settings.json", data).expect("Unable to save data");
}
fn load_settings(path: &str) -> Result<Settings, std::boxed::Box<dyn std::error::Error>> {
    if Path::new(&path).exists() {
        let data:String= fs::read_to_string(&path)?;
        let settings: Settings = serde_json::from_str(&data)
            .expect("invalid json format in settings file");
        Ok(settings)
    } else {
        std::fs::File::create(&path)?;
        let clck:Device = Device { enabled: (true), mahertz: (400), mihertz: (200), wave: (1) };
        let btn:Device = Device { enabled: (true), mahertz: (200), mihertz: (300), wave: (0) };
        let whee:Device = Device { enabled: (true), mahertz: (400), mihertz: (500), wave: (0) };
        let settings = Settings { clck: (clck), btn: (btn), whee: (whee) };
        Ok(settings)
    }
}