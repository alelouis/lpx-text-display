extern crate midir;
mod midi;

fn xy_to_index(x: u8, y: u8) -> u8 {
    11 + x + y * 10
}

fn main() {
    // Get connection to LPX
    let mut lpx_out = midi::get_output_connection("Launchpad X LPX MIDI In".to_string());
    
    // Go to Programmer mode
    lpx_out.send(&[0xF0, 0x00, 0x20, 0x29, 0x02, 0x0C, 0x0E, 0x01, 0xF7]).unwrap();
    
    // Prefix for complete matrix command
    let mut msg = vec![240u8, 0u8, 32u8, 41u8, 2u8, 12u8, 3u8, 0u8, 11u8, 13u8];
    
    // Append messages for all LEDs
    for x in 0..8 {
        for y in 0..8 {

            // Define colors
            let r = 2*8*y;
            let v = r;
            let b = 127 - 2*8*x;

            // Compute LED index
            let idx: u8 = xy_to_index(x, y);
            let mut led = vec![3u8, idx, r, v, b];
            

            msg.append(&mut led);
        }
    }

    // Add end of message byte
    msg.append(&mut vec![247u8]);

    // Send to LPX
    lpx_out.send(&msg).unwrap();
}
