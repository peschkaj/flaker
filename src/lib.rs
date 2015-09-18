extern crate time;
extern crate num;
extern crate byteorder;
use num::BigUint;
use num::bigint::{ToBigInt};
use time::Timespec;
use time::Tm;
use std::error;
use byteorder::{LittleEndian, WriteBytesExt};

struct Flaker {
    identifier: Vec<u8>,
    epoch: u64,
    last_oxidized_in_ms: u64,
    counter: u32,
}

impl Flaker {
	pub fn new_from_identifier(identifier: Vec<u8>) -> Flaker {
		let default_epoch_ts = Timespec::new(0, 0);
		let default_epoch_ms = default_epoch_ts.sec as u64 + default_epoch_ts.nsec as u64 / 1000 / 1000;

		Flaker::new(identifier, default_epoch_ms, false)
	}

	pub fn new(identifier: Vec<u8>, epoch: u64, littleEndian: bool) -> Flaker {
		// TODO : check that identifier has a length of 6
		let mut l_identifier = identifier.clone();

		if !littleEndian {
			l_identifier.reverse();
		}

		Flaker { identifier: l_identifier,
				 epoch: epoch,
				 last_oxidized_in_ms: Flaker::current_time_in_ms(), 
				 counter: 0
			   }
	}

	fn update(&mut self) -> Result<(), String> {
		let current_time_in_ms = Flaker::current_time_in_ms();

		if self.last_oxidized_in_ms > current_time_in_ms {
			return Result::Err("The clock is running backwards".to_owned());
		}

		if self.last_oxidized_in_ms < current_time_in_ms {
			self.counter = 0;
		}
		else {
			self.counter += 1;
		}

		self.last_oxidized_in_ms = current_time_in_ms;

		Ok(())
	}

	pub fn get_id(&mut self) -> BigUint {
		self.update();

		let mut bytes = Vec::new();

		bytes.push(self.counter as u8);
		bytes.push((self.counter >> 8) as u8);

		// next 6 bytes are the worker id
		for i in &self.identifier {
			bytes.push(*i);
		}

		println!("BLURPLE!");

		let mut wtr = vec![];

		wtr.write_u64::<LittleEndian>(self.last_oxidized_in_ms).unwrap();

		for w in wtr {
			bytes.push(w);
		}

		// (0..7).map(|x| {
		// 	println!("{:?}", x);
		// 	let pos = x * 8;
		// 	println!("{:?} is {:0>8b}", x, (self.last_oxidized_in_ms >> pos) as u8);
		// 	bytes.push((self.last_oxidized_in_ms >> pos) as u8);
		// });

		BigUint::from_bytes_le(&bytes)
	}

	fn current_time_in_ms() -> u64 {
		let now = time::now();
		let now_ts = now.to_timespec();

		now_ts.sec as u64 + now_ts.nsec as u64 / 1000 / 1000
	}


}

#[test]
fn ids_change_over_time() {
	let mut f1 = Flaker::new_from_identifier(vec![0, 1, 2, 3, 4, 5]);
	let id1 = f1.get_id();
	std::thread::sleep_ms(50);
	let id2 = f1.get_id();

	println!("{} < {}", id1, id2);

	assert!(id1 < id2);
}

#[test]
fn ids_change_quickly() {
	let mut f1 = Flaker::new_from_identifier(vec![0, 1, 2, 3, 4, 5]);

	let id3 = f1.get_id();
	let id4 = f1.get_id();
	
	assert!(id3 < id4);
}
