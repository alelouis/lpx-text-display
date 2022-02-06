extern crate midir;
extern crate font8x8;

mod midi;

use std::env;
use std::{thread, time};
use font8x8::{BASIC_FONTS, UnicodeFonts};

/// Convert from x, y (right, up) to LED index
fn xy_to_index(x: u8, y: u8) -> u8 {
    11 + x + y * 10
}

fn main() {

    // Get arguments 
    let args: Vec<String> = env::args().collect();
    let text = format!(" {} ", &args[1]);

    // Get connection to LPX
    let mut lpx_out = midi::get_output_connection("Launchpad X LPX MIDI In".to_string());
    
    // Go to Programmer mode
    lpx_out.send(&[0xF0, 0x00, 0x20, 0x29, 0x02, 0x0C, 0x0E, 0x01, 0xF7]).unwrap();
     
    // Placeholder matrix for sentence
    let mut matrix = vec![[false; 8]; 8*text.len()];
    
    // Fill matrix with boolean value of chars
    for (i, c) in text.chars().enumerate() {
        for x in 0..8 {
            for y in 0..8 {
                if let Some(glyph) = BASIC_FONTS.get(c) {
                    match glyph[7-y] & 1 << x {
                        0 => matrix[i*8+x][y] = true,
                        _ => matrix[i*8+x][y] = false,
                    };
                }
            }
        }
    }
    
    // Append messages for all LEDs
    for frame in 0..matrix.len()-8 {
        
        // Prefix for complete matrix command
        let mut msg = vec![240u8, 0u8, 32u8, 41u8, 2u8, 12u8, 3u8];
    
        let wait = time::Duration::from_millis(100);
        thread::sleep(wait);

        for x in 0..8 {
            for y in 0..8 {
                let color = match matrix[frame+x][y] {
                    true => vec![0u8, 0u8, 0u8],
                    false => vec![127u8, 0u8, 0u8],
                };

                // Define colors
                let r = color[0];
                let v = color[1];
                let b = color[2];

                // Compute LED index
                let idx: u8 = xy_to_index(x as u8, y as u8);
                let mut led = vec![3u8, idx, r, v, b];

                msg.append(&mut led);
            }
        }
        // Add end of message byte
        msg.append(&mut vec![247u8]);

        // Send to LPX
        lpx_out.send(&msg).unwrap();
    }
}
