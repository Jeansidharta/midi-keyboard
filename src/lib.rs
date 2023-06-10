use futures_util::StreamExt;
use midir::MidiInput;
use midly::{
    live::{LiveEvent, SystemRealtime},
    MidiMessage,
};
use service::lamps;
use std::sync::mpsc;
use std::thread;
pub use tokio_tungstenite::{connect_async, tungstenite::Message};
use utils::note_name::NoteName;

mod constants;
mod service;
mod utils;

#[derive(Debug, Clone, Default)]
struct MidiState {
    bank: u16,
    program: u8,
    selected_notes: [bool; 12],
}

impl MidiState {
    fn get_targets(&self) -> String {
        let notes = self.selected_notes;
        notes
            .iter()
            .enumerate()
            .map(|(index, is_set)| match (is_set, index) {
                (false, _) => None,
                (true, index) => NoteName::parse(index as u8).into_lamp_id(),
            })
            .filter(|note| match note {
                None => false,
                Some(_) => true,
            })
            .map(|note| note.unwrap().to_string())
            .reduce(|acc, note| format!("{}, {}", acc, note))
            .unwrap_or_else(|| String::new())
    }
}

fn read_midi_message(
    tx: &mpsc::SyncSender<String>,
    _timestamp: u64,
    data: &[u8],
    midi_state: &mut MidiState,
) -> midly::Result<()> {
    let event = LiveEvent::parse(data)?;

    match event {
        LiveEvent::Realtime(SystemRealtime::ActiveSensing) => (),
        LiveEvent::Midi {
            channel: _,
            message,
        } => match message {
            MidiMessage::NoteOn { key, vel } => {
                let note_name = NoteName::parse(key.as_int());
                note_name.display_with_velocity(vel.as_int());
                match (midi_state.program, midi_state.bank) {
                    (63, 0) => {
                        tx.send(lamps::set_scene_color(
                            midi_state.get_targets(),
                            note_name.into_hue(),
                            1.0,
                            100,
                        ))
                        .unwrap_or_else(|_err| println!("Failed to send message to queue"));
                    }
                    (64, 0) => {
                        tx.send(lamps::set_scene_color_temperature(
                            midi_state.get_targets(),
                            (note_name.into_index() as f32 * 4800.0 / 127.0) as u32 + 1700,
                            100,
                        ))
                        .unwrap_or_else(|_err| println!("Failed to send message to queue"));
                    }
                    (65, 0) => {
                        tx.send(lamps::toggle_lamp(midi_state.get_targets()))
                            .unwrap_or_else(|_err| println!("Failed to send message to queue"));
                    }
                    (66, 0) => {
                        let index = note_name.into_scale_index() as usize;
                        midi_state.selected_notes[index] = !midi_state.selected_notes[index];
                        note_name.into_lamp_id().and_then(|lamp_id| -> Option<u64> {
                            if midi_state.selected_notes[index] {
                                tx.send(lamps::blink_lamp_green(lamp_id.to_string()))
                            } else {
                                tx.send(lamps::blink_lamp_red(lamp_id.to_string()))
                            }
                            .unwrap_or_else(|_err| println!("Failed to send message to queue"));
                            None
                        });
                    }
                    _ => {}
                }
            }
            MidiMessage::Controller { controller, value } if controller.as_int() == 0 => {
                midi_state.bank = ((midi_state.bank as u16) & 0b00000000_11111111)
                    | ((value.as_int() as u16) << 8)
            }
            MidiMessage::Controller { controller, value } if controller.as_int() == 32 => {
                midi_state.bank = (midi_state.bank & 0b11111111_00000000) | (value.as_int() as u16);
            }
            MidiMessage::ProgramChange { program } => {
                midi_state.program = program.as_int();
                if program == 66 && midi_state.bank == 0 {
                    tx.send(lamps::blink_lamp_green(midi_state.get_targets()))
                        .unwrap_or_else(|_err| println!("Failed to send message to queue"));
                };
                println!(
                    "Bank is {}, program is {}",
                    midi_state.bank, midi_state.program
                );
            }

            // Ignore NoteOff events
            MidiMessage::NoteOff { key: _, vel: _ } => {}

            _ => println!("Midi: {:?}", event),
        },
        _ => println!("{:?}", event),
        // _ => (),
    };

    Ok(())
}

async fn run_websocket(rx: mpsc::Receiver<String>) -> Result<(), &'static str> {
    loop {
        const URL: &str = "ws://casa.sidharta.xyz/api/lamp/websocket";
        println!("Trying to connect...");
        let (mut websocket, _) = match connect_async(URL).await {
            Err(_) => {
                println!("Could not connect to websocket. Retrying in 5 seconds");
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }
            Ok(websocket) => websocket,
        };

        let (write, read) = websocket.split();

        for message in rx.iter() {
            match &message[..] {
                "PING" => {
                    println!("Ping!");
                    write.write(Message::Ping(Vec::new())).or_else(|err| {
                        println!("Failed to Ping: {}", err);
                        Err("Failed to Ping")
                    })?;
                }
                _ => {
                    websocket
                        .write_message(Message::text(message))
                        .or_else(|err| {
                            println!("Some error occurred on websocket {}", err);
                            Err("Failed to send message")
                        })?;
                }
            };
        }
    }
}

fn run_midi_reader(tx: mpsc::SyncSender<String>) -> Result<(), &'static str> {
    let mut selected_port: Option<midir::MidiInputPort> = None;
    let mut _connection: Option<midir::MidiInputConnection<MidiState>> = None;

    loop {
        let input = MidiInput::new("Midi Lights").or(Err("Could not create midi input"))?;
        let ports = input.ports();
        let port = ports
            .iter()
            .find(|port| match input.port_name(port) {
                Ok(port_name) => port_name.starts_with("CASIO"),
                Err(_) => false,
            })
            .map(|p| p.clone());

        if let None = port {
            if let Some(_) = selected_port {
                println!("Lost connection to keyboard");
            }
            selected_port = None;
            thread::sleep(std::time::Duration::from_secs(1));
            continue;
        }

        if port == selected_port {
            thread::sleep(std::time::Duration::from_secs(1));
            continue;
        } else {
            println!("Connected to keyboard");
            let tx = tx.clone();
            selected_port = port.clone();
            _connection = Some(
                input
                    .connect(
                        &port.unwrap(),
                        "SOME_NAME",
                        move |timestamp, data, midi_state| {
                            read_midi_message(&tx, timestamp, data, midi_state).unwrap()
                        },
                        MidiState::default(),
                    )
                    .unwrap(),
            );
        }

        thread::sleep(std::time::Duration::from_secs(1));
    }
}

pub async fn main() -> Result<(), &'static str> {
    let (tx, rx) = mpsc::sync_channel::<String>(100);

    let websocket_handle = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                match run_websocket(rx).await {
                    Ok(()) => {}
                    Err(err) => println!("Error: {}", err),
                };
            });
    });

    let midi_reader_handle = {
        let tx = tx.clone();
        thread::spawn(move || {
            match run_midi_reader(tx.clone()) {
                Ok(()) => {}
                Err(err) => println!("Error: {}", err),
            };
        })
    };
    let pipe_maintainer = {
        let tx = tx.clone();
        thread::spawn(move || loop {
            tx.clone()
                .send("PING".to_string())
                .unwrap_or_else(|_err| println!("Failed to PING"));
            thread::sleep(std::time::Duration::from_secs(1));
        })
    };

    websocket_handle.join().unwrap();
    midi_reader_handle.join().unwrap();
    pipe_maintainer.join().unwrap();

    Ok(())
}
