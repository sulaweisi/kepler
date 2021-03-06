#![no_main]
extern crate kepler_core;
#[macro_use]
extern crate libfuzzer_sys;

use kepler_core::core::UntrustedCompactBlock;
use kepler_core::ser;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<UntrustedCompactBlock, ser::Error> =
		ser::deserialize(&mut d, ser::ProtocolVersion(2));
});
