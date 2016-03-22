# flaker

A flake implementation for Rust

## What is this?

A flake is a 128-bit, k-ordered ID - it's real time ordered and stored in a way that sorts lexically. Flaker is derived from [Boundary's flake implementation][1] and the author's previous work on [Rustflakes][2] - a similar tool for .NET.

Basically - it's an ordered ID generation service.

Rather than rely on a central data storage repository for generating IDs, developers can implement an ID generator at the source of data using a `flaker`.

## Identifiers

Identifiers are generated as 128-bit numbers:

* 64-bit timestamp as milliseconds since the dawn of time (January 1, 1970).
* 48-bit worker identifier - typically this would be the MAC address, but you could use whatever you want.
* 16-bit sequence number that is incremented when more than one identifier is requested in the same millisecond and reset to 0 when the clock moves forward.

## Questions

### How should I use this?

Take a look at [`bulk_generate.rs`][3] in the examples. 

In longer form:

* Add the crate to `Cargo.toml`
* Tell your code to use flaker with `extern crate flaker`
* Create a new instance of flaker.
* Call `get_id()` to get a brand new ID.'

Ideally, you'd use a central service to generate IDs - preferrably one per server instance.

### When should I use flaker?

I mean, my database can generate IDs, right?

A centralized ID generator seems good until you have a large number of actors in your system generating IDs - think hundreds of servers. A large number of actors can overwhelm the ID generation capabilities of your central store. Or you may not care about gaps in the store and only care that you have time ordered, unique identifiers. Depending on the implementation of the underlying backing store, it also may not be possible to have the database generate sequential identifiers yourself (earlier versions of Azure SQL Database had this feature).

### What should I use for the worker identifier?

I've been known to pull the MAC address of the first active ethernet adapter. It doesn't matter what you're using so long as it's guaranteed to be unique per generator. You could pull the last 6 bytes of the CPU identifier if that suited you.

While machine identity should be relatively meaningless in a distributed system, that doesn't mean we can't use an arbitrary indicator to achieve distinction between functioning nodes in a given time range. If you're afraid of MAC address spoofing, then you should be able to work something out.

6 bytes gives you a lot of room for creativity. I suggest arbitrarily incrementing a number that you store in an S3 bucket. You could regenerate your worker identifier 281,474,976,710,656 times before you run out of unique values. 

### But timezones!

`flaker` uses UTC when generating IDs. I don't trust you to set your server clocks to UTC, so I just took that leap for you.

[1]: https://github.com/boundary/flake
[2]: https://github.com/peschkaj/rustflakes
[3]: https://github.com/peschkaj/flaker/blob/master/examples/bulk_generate.rs