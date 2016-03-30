// Copyright (c) 2016 - Jeremiah Peschka
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
extern crate flaker;

use flaker::Endianness;

fn main() {
    let mut flake = flaker::Flaker::new([0, 1, 2, 3, 4, 5], Endianness::LittleEndian);
    let mut error_count = 0;
    let mut range = 0..10_000;
    
    loop {
        match range.next() {
            Some(_) => {
                match flake.get_id() {
                    Err(_) => error_count += 1,
                    Ok(id) =>println!("{}", id),
                }
            },
            None => { break },
        }
    }
    
    println!("There were {} errors during execution.", error_count);
}