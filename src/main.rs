use picovoice::{rhino::{RhinoInference, Rhino, RhinoBuilder}, PicovoiceBuilder};
use pv_recorder::PvRecorderBuilder;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::Local;
use enigo::{self, Enigo, Keyboard};
use std::env;

static LISTENING: AtomicBool = AtomicBool::new(false);

static CONTEXT_PATH: &str = "./picovoice/gamer_v3.rhn";

fn main() {
    show_audio_devices();
    let access_key = env::var("ACCESS_KEY").unwrap();
    let audio_device_index = 2;
    
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
                    on_inference(inference, &mut enigo);
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

fn on_inference(inference: RhinoInference, enigo: &mut Enigo) {
        if inference.is_understood {
            println!("[{}] Inferred:", Local::now().format("%F %T"));
            println!("{{");
            let intent = inference.intent.unwrap();
            println!("\tintent : '{}'", intent);
            println!("\tslots : {{");
            for (slot, value) in inference.slots.iter() {
                println!("\t\t{} : {}", slot, value);
                // handle_command(intent, slot, value, &mut enigo);
            }
            println!("\t}}");
            println!("}}\n");
            // add code to take action based on inferred intent and slot values
        } else {
            println!("Unknown command XD")
            // add code to handle unsupported commands
        }
}

fn handle_command(intent: String, slot: &String, value: &String, enigo: &mut Enigo) {
                match intent.as_str() {
                    "moveJump" =>  {
                        enigo.key(enigo::Key::Unicode('w'), enigo::Direction::Click);
                        println!("pressed w");
                    },
                    "moveLeft" => {
                        enigo.key(enigo::Key::Unicode('a'), enigo::Direction::Click);
                        println!("pressed a");

                    }
                    "moveRight" => {
                        enigo.key(enigo::Key::Unicode('d'), enigo::Direction::Click);
                        println!("pressed d");
                    }
                    "moveDown" => {
                        enigo.key(enigo::Key::Unicode('s'), enigo::Direction::Click);
                        println!("pressed s");
                    }
                    _ => println!("was something else"),
                }
}
