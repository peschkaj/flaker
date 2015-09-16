extern crate time;
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

	fn update(&self) -> Result<(), String> {
		let current_time_in_ms = Flaker::current_time_in_ms();

		if self.last_oxidized_in_ms > current_time_in_ms {
			return Result::Err("The clock is running backwards".to_owned());
		}



		Ok(())
	}

	pub fn current_time_in_ms() -> u64 {
		let now = time::now();
		let now_ts = now.to_timespec();

		now_ts.sec as u64 + now_ts.nsec as u64 / 1000 / 1000
	}


}

#[test]
fn it_works() {
}
