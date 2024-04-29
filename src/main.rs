use picovoice::{rhino::{RhinoInference, Rhino, RhinoBuilder}, PicovoiceBuilder};
use pv_recorder::PvRecorderBuilder;
use std::{os::windows::thread, sync::atomic::{AtomicBool, Ordering}};
use chrono::Local;
use enigo::{
    Button, Coordinate,
    Direction::{self, Click, Press, Release},
    Enigo, Key, Keyboard, Mouse, Settings,
};
use std::env;

static LISTENING: AtomicBool = AtomicBool::new(false);

static CONTEXT_PATH: &str = "./pv/windows/gaming_en_windows_v3_0_0.rhn";


const KEY_JUMP: enigo::Key = Key::Unicode('l');
const KEY_DOWN: enigo::Key = Key::Unicode('s');
const KEY_UP: enigo::Key = Key::Unicode('w');
const KEY_LEFT: enigo::Key = Key::Unicode('a');
const KEY_RIGHT: enigo::Key = Key::Unicode('d');

fn main() {
    show_audio_devices();
    //let access_key = env::var("ACCESS_KEY").unwrap();
    let access_key = "";
    let audio_device_index = 0;
    
    let rhino: Rhino = RhinoBuilder::new(
        access_key,
        CONTEXT_PATH,
    )
    .endpoint_duration_sec(0.5)
    .init()
    .expect("Unable to create Rhino");
    let recorder = PvRecorderBuilder::new(rhino.frame_length() as i32)
        .device_index(audio_device_index)
        .init()
        .expect("Failed to initialize pvrecorder");
    recorder.start().expect("Failed to start audio recording");

    LISTENING.store(true, Ordering::SeqCst);
    ctrlc::set_handler(|| {
        LISTENING.store(false, Ordering::SeqCst);
    })
    .expect("Unable to setup signal handler");
    let mut enigo = enigo::Enigo::new(&enigo::Settings::default()).unwrap();
    while LISTENING.load(Ordering::SeqCst) {
        let frame = recorder.read().expect("Failed to read audio frame");
        if let Ok(is_finalized) = rhino.process(&frame) {
            if is_finalized {
                if let Ok(inference) = rhino.get_inference() {
                    let mut inf_result: (String, String, String) = on_inference(inference);
                    handle_command(inf_result.0, inf_result.1, inf_result.2, &mut enigo)
                }
            }
        }
    }

    println!("\nStopping...");
    recorder.stop().expect("Failed to stop audio recording");
}

fn show_audio_devices() {
    let audio_devices = PvRecorderBuilder::default().get_available_devices();
    match audio_devices {
        Ok(audio_devices) => {
            for (idx, device) in audio_devices.iter().enumerate() {
                println!("index: {}, device name: {:?}", idx, device);
            }
        }
        Err(err) => panic!("Failed to get audio devices: {}", err),
    };
}

fn on_inference(inference: RhinoInference) -> (String, String, String) {
        if inference.is_understood {
            println!("[{}] Inferred:", Local::now().format("%F %T"));
            println!("{{");
            let intent = inference.intent.unwrap();
            println!("\tintent : '{}'", intent);
            println!("\tslots : {{");
            let mut sl: String = "".to_string();
            let mut val: String = "".to_string();
            for (slot, value) in inference.slots.iter() {
                println!("\t\t{} : {}", slot, value);
                // handle_command(intent, slot, value, &mut enigo);
                sl = slot.clone();
                val = value.clone();
            }
            println!("\t}}");
            println!("}}\n");
            return (intent.clone(), sl, val);
            // add code to take action based on inferred intent and slot values
        } else {
            println!("Unknown command XD");
            return ("".to_string(), "".to_string() , "".to_string())
            // add code to handle unsupported commands
        }
}

fn handle_command(intent: String, slot: String, value: String, enigo: &mut Enigo) {
                match (intent.as_str(), value.as_str()) {
                    ("moveCharacterJump", _) =>  {
                        enigo.key(KEY_JUMP, Press);
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        enigo.key(KEY_JUMP, Release);
                        println!("jump");
                    },
                    ("moveCharacterLeft", size) => {
                        enigo.key(KEY_LEFT, Press).unwrap();
                        match size {
                            "big" => std::thread::sleep(std::time::Duration::from_secs(2)),
                            "mid" => std::thread::sleep(std::time::Duration::from_secs(1)),
                            "small" => std::thread::sleep(std::time::Duration::from_millis(500)),
                            _ => std::thread::sleep(std::time::Duration::from_millis(200))
                        }
                        enigo.key(KEY_LEFT, Release).unwrap();
                        println!("LEFT!");
                    }
                    ("moveCharacterRight", size) => {
                        enigo.key(KEY_RIGHT, Press).unwrap();
                        match size {
                            "big" => std::thread::sleep(std::time::Duration::from_secs(2)),
                            "mid" => std::thread::sleep(std::time::Duration::from_secs(1)),
                            "small" => std::thread::sleep(std::time::Duration::from_millis(500)),
                            _ => std::thread::sleep(std::time::Duration::from_millis(200))
                        }
                        enigo.key(KEY_RIGHT, Release).unwrap();
                        println!("Big right!");
                    }
                    ("moveCharacterDown", size) => {
                        enigo.key(KEY_DOWN, Press).unwrap();
                        match size {
                            "big" => std::thread::sleep(std::time::Duration::from_secs(2)),
                            "mid" => std::thread::sleep(std::time::Duration::from_secs(1)),
                            "small" => std::thread::sleep(std::time::Duration::from_millis(500)),
                            _ => std::thread::sleep(std::time::Duration::from_millis(200))
                        }
                        enigo.key(KEY_DOWN, Release).unwrap();
                        println!("Down!");
                    }                    
                    ("moveCharacterUp", size) => {
                        enigo.key(KEY_UP, Press).unwrap();
                        match size {
                            "big" => std::thread::sleep(std::time::Duration::from_secs(2)),
                            "mid" => std::thread::sleep(std::time::Duration::from_secs(1)),
                            "small" => std::thread::sleep(std::time::Duration::from_millis(500)),
                            _ => std::thread::sleep(std::time::Duration::from_millis(200))
                        }
                        enigo.key(KEY_UP, Release).unwrap();
                        println!("Up!");
                    }                    
                    ("moveCharacterRump", size) => {
                        enigo.key(KEY_RIGHT, Press).unwrap();
                        enigo.key(KEY_JUMP, Press).unwrap();
                        match size {
                            "big" => std::thread::sleep(std::time::Duration::from_secs(2)),
                            "mid" => std::thread::sleep(std::time::Duration::from_secs(1)),
                            "small" => std::thread::sleep(std::time::Duration::from_millis(500)),
                            _ => std::thread::sleep(std::time::Duration::from_millis(500))
                        }
                        enigo.key(KEY_RIGHT, Release).unwrap();
                        enigo.key(KEY_JUMP, Release).unwrap();
                        println!("Rump!");
                    }
                    ("moveCharacterLump", size) => {
                        enigo.key(KEY_RIGHT, Press).unwrap();
                        enigo.key(KEY_JUMP, Press).unwrap();
                        match size {
                            "big" => std::thread::sleep(std::time::Duration::from_secs(2)),
                            "mid" => std::thread::sleep(std::time::Duration::from_secs(1)),
                            "small" => std::thread::sleep(std::time::Duration::from_millis(500)),
                            _ => std::thread::sleep(std::time::Duration::from_millis(500))
                        }
                        enigo.key(KEY_RIGHT, Release).unwrap();
                        enigo.key(KEY_JUMP, Release).unwrap();
                        println!("Rump!");
                    }                 
                    _ => { 
                        println!("intent {}, slot {}, value {}", intent, slot, value);
                        println!("was something else")
                    },
                }
}
