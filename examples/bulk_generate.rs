extern crate flaker;

fn main() {
    let mut flake = flaker::Flaker::new([0, 1, 2, 3, 4, 5], true);
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