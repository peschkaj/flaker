extern crate time;
extern crate num;
use num::BigUint;
use num::bigint::{ToBigInt};
use time::Timespec;
use time::Tm;
use std::error;

struct Flaker {
    identifier: u32,
    epoch: u64,
    last_oxidized_in_ms: u64,
    counter: u32
}

impl Flaker {
	pub fn new_from_identifier(identifier: u32) -> Flaker {
		let default_epoch_ts = Timespec::new(0, 0);
		let default_epoch_ms = default_epoch_ts.sec as u64 + default_epoch_ts.nsec as u64 / 1000 / 1000;

		Flaker::new(identifier, default_epoch_ms)
	}

	pub fn new(identifier: u32, epoch: u64) -> Flaker {
		Flaker { identifier: identifier,
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

	pub fn get_id(&self) -> BigUint {
		let mut bytes: Vec<u8> = Vec::new();

		// first two bytes are the key space counter
		bytes[0] = self.counter as u8;
		bytes[1] = (self.counter >> 8) as u8;

		// next 6 bytes are the worker id
		(0..5).map(|x| bytes[x + 1] = (self.identifier >> (x * 8)) as u8);

		// last 8 bytes are the time counter
		(0..7).map(|x| bytes[x + 8] = (self.last_oxidized_in_ms >> (x * 8)) as u8);

		BigUint::from_bytes_le(&bytes)
	}

	fn current_time_in_ms() -> u64 {
		let now = time::now();
		let now_ts = now.to_timespec();

		now_ts.sec as u64 + now_ts.nsec as u64 / 1000 / 1000
	}


}

#[test]
fn it_works() {
}
