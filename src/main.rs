mod midi;
use midir::MidiInputConnection;
use nannou::prelude::*;
use std::sync::mpsc;

fn main() {
    nannou::app(start).update(event).run();
}

struct Model {
    _connection: MidiInputConnection<()>,
    midi_receiver: mpsc::Receiver<midi::Message>,
    last_midi_message_at: Option<f32>,
    first_midi_message_at: Option<f32>,
}

fn start(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());

    app.new_window().size(512, 512).view(view).build().unwrap();

    let (midi_receiver, _connection) = midi::receive();

    Model {
        _connection,
        midi_receiver,
        last_midi_message_at: None,
        first_midi_message_at: None,
    }
}

fn event(_app: &App, model: &mut Model, _event: Update) {
    match model.midi_receiver.try_recv() {
        Ok(message) => {
            handle_midi_message(model, message);
        }
        Err(_error) => (),
    };
}

fn to_seconds(timestamp: u64) -> f32 {
    timestamp as f32 / 1_000_000.0
}

fn handle_midi_message(model: &mut Model, message: midi::Message) {
    match message.kind {
        midi::MessageType::NoteOn => {
            handle_note_on(model, message);
        }
        _ => (),
    }
}

fn handle_note_on(model: &mut Model, message: midi::Message) {
    let timestamp_in_seconds = to_seconds(message.timestamp);
    match model.first_midi_message_at {
        Some(first_midi_message_at) => {
            let time_elapsed = timestamp_in_seconds - first_midi_message_at;
            model.last_midi_message_at = Some(time_elapsed);
        }
        None => {
            model.first_midi_message_at = Some(timestamp_in_seconds);
            model.last_midi_message_at = Some(0.0);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    match model.last_midi_message_at {
        Some(last_midi_message_at) => {
            let time_since_last_midi_message = app.time - last_midi_message_at;

            // let x = app.time.sin() * 50.0;
            let x = time_since_last_midi_message * 50.0;
            // let y = app.time.sin() * 50.0;
            let y = 0.0;

            draw.background().color(PLUM);
            draw.ellipse().x_y(x, y);

            draw.to_frame(app, &frame).unwrap();
        }
        None => (),
    }
}
