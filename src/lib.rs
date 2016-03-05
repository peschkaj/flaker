#[allow(dead_code)]
mod flaker {

    extern crate time;
    extern crate num;
    extern crate byteorder;
    use self::num::BigUint;
    use self::byteorder::{LittleEndian, WriteBytesExt};



    #[derive(Debug)]
    pub enum FlakeError {
        ClockIsRunningBackwards
    }

    pub trait HasFlakes {
        fn update(&mut self) -> Result<(), FlakeError>;
        fn get_id(&mut self) -> BigUint;
    }

    pub struct Flaker {
        identifier: Vec<u8>,
        last_generated_time_ms: u64,
        counter: u32,
    }
    
    impl Flaker {
        pub fn new_from_identifier(identifier: Vec<u8>) -> Flaker {
            Flaker::new(identifier, false)
        }

        pub fn new(identifier: Vec<u8>, little_endian: bool) -> Flaker {
            let mut l_identifier = identifier.clone();
            
            if l_identifier.len() < 6 {
                panic!("Identifier must have a length of 6");
            }

            if !little_endian {
                l_identifier.reverse();
            }

            Flaker { identifier: l_identifier,
                    last_generated_time_ms: Flaker::current_time_in_ms(),
                    counter: 0
                }
        }

        fn current_time_in_ms() -> u64 {
            let now = time::now();
            let now_ts = now.to_timespec();
            

            // TODO should be `now_ts` minus `epoch`
            // changing this means we should rename this function, too
            now_ts.sec as u64 + (now_ts.nsec as u64 / 1000 / 1000)
        }
    }

    impl HasFlakes for Flaker {
        fn update(&mut self) -> Result<(), FlakeError> {
            let current_time_in_ms = Flaker::current_time_in_ms();

            if self.last_generated_time_ms > current_time_in_ms {
                return Result::Err(FlakeError::ClockIsRunningBackwards);
            }

            if self.last_generated_time_ms < current_time_in_ms {
                self.counter = 0;
            }
            else {
                self.counter += 1;
            }

            self.last_generated_time_ms = current_time_in_ms;

            Ok(())
        }

        // TODO signature needs to be changed to return a result
        fn get_id(&mut self) -> BigUint {
            // TODO check this for OK-ness
            if let Err(e) = self.update() {
                panic!(e);
            }
            
            

            // Create a new vec of bytes
            let mut bytes = Vec::new();

            // push the counter into bytes
            // TODO why did I use a u32 for counter if I only use 16 bits of it?
            bytes.push(self.counter as u8);
            bytes.push((self.counter >> 8) as u8);

            // next 6 bytes are the worker id
            for i in &self.identifier {
                bytes.push(*i);
            }

            let mut wtr = vec![];

            wtr.write_u64::<LittleEndian>(self.last_generated_time_ms).unwrap();

            for w in wtr {
                bytes.push(w);
            }

            BigUint::from_bytes_le(&bytes)
        }
    }

    #[test]
    fn ids_change_over_time() {
        let mut f1 = Flaker::new_from_identifier(vec![0, 1, 2, 3, 4, 5]);
        let id1 = f1.get_id();
        std::thread::sleep(Duration::from_millis(50));
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
}