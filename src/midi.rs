//! Midi send and receive helpers

use midir::{MidiIO, MidiOutput, MidiOutputConnection, MidiOutputPort};

/// Open connection
pub fn get_output_connection(s: String) -> MidiOutputConnection {
    let midi_out = MidiOutput::new("midi_out").expect("Could not open midi output.");
    let out_ports = midi_out.ports();
    let device_port: Option<&MidiOutputPort> = match get_port_index_by_name(&midi_out, s) {
        Some(i) => out_ports.get(i),
        None => None,
    };
    midi_out
        .connect(device_port.unwrap(), "midir-test")
        .unwrap()
}

/// Finds port for a given string name
fn get_port_index_by_name<T: MidiIO>(midi_in: &T, name: String) -> Option<usize> {
    let mut port_index: Option<usize> = None;
    for (i, p) in midi_in.ports().iter().enumerate() {
        if midi_in.port_name(&p).unwrap().eq(&name) {
            port_index = Some(i);
            break;
        }
    }
    port_index
}

