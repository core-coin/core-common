// Copyright 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Testing to and from f64 lossy for U256 primitive type.

use primitive_types::{H160, H176, H256, U256, U1368};
use serde_json;
use serde::{Serialize, Deserialize};


#[test]
#[allow(clippy::float_cmp)]
fn convert_u256_to_f64() {
	assert_eq!(U256::from(0).to_f64_lossy(), 0.0);
	assert_eq!(U256::from(42).to_f64_lossy(), 42.0);
	assert_eq!(U256::from(1_000_000_000_000_000_000u128).to_f64_lossy(), 1_000_000_000_000_000_000.0,);
}

#[test]
#[allow(clippy::excessive_precision, clippy::float_cmp, clippy::unreadable_literal)]
#[cfg(feature = "std")]
fn convert_u256_to_f64_precision_loss() {
	assert_eq!(U256::from(u64::max_value()).to_f64_lossy(), u64::max_value() as f64,);
	assert_eq!(
		U256::MAX.to_f64_lossy(),
		115792089237316195423570985008687907853269984665640564039457584007913129639935.0,
	);
	assert_eq!(
		U256::MAX.to_f64_lossy(),
		115792089237316200000000000000000000000000000000000000000000000000000000000000.0,
	);
}

#[test]
fn convert_f64_to_u256() {
	assert_eq!(U256::from_f64_lossy(0.0), 0.into());
	assert_eq!(U256::from_f64_lossy(13.37), 13.into());
	assert_eq!(U256::from_f64_lossy(42.0), 42.into());
	assert_eq!(U256::from_f64_lossy(999.999), 999.into());
	assert_eq!(U256::from_f64_lossy(1_000_000_000_000_000_000.0), 1_000_000_000_000_000_000u128.into(),);
}

#[test]
fn convert_f64_to_u256_large() {
	let value = U256::from(1) << U256::from(255);
	assert_eq!(U256::from_f64_lossy(format!("{}", value).parse::<f64>().expect("unexpected error parsing f64")), value);
}

#[test]
#[allow(clippy::unreadable_literal)]
fn convert_f64_to_u256_overflow() {
	assert_eq!(
		U256::from_f64_lossy(115792089237316200000000000000000000000000000000000000000000000000000000000000.0),
		U256::MAX,
	);
	assert_eq!(
		U256::from_f64_lossy(999999999999999999999999999999999999999999999999999999999999999999999999999999.0),
		U256::MAX,
	);
}

#[test]
fn convert_f64_to_u256_non_normal() {
	assert_eq!(U256::from_f64_lossy(f64::EPSILON), 0.into());
	assert_eq!(U256::from_f64_lossy(f64::from_bits(0)), 0.into());
	assert_eq!(U256::from_f64_lossy(f64::NAN), 0.into());
	assert_eq!(U256::from_f64_lossy(f64::NEG_INFINITY), 0.into());
	assert_eq!(U256::from_f64_lossy(f64::INFINITY), U256::MAX);
}

#[test]
fn f64_to_u256_truncation() {
	assert_eq!(U256::from_f64_lossy(10.5), 10.into());
}

#[test]
fn hash_prefix() {
	assert_eq!(H176::default().to_string(), "00000000000000000000000000000000000000000000");
	assert_eq!(
		H176::from_slice(&[0xab, 3, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6]).to_string(),
		"ab030304050607080102030405060708010203040506"
	);

	assert_eq!(H160::default().to_string(), "0x0000…0000");
	assert_eq!(
		H160::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4]).to_string(),
		"0x0102…0304"
	);

	assert_eq!(H256::default().to_string(), "0x0000…0000");
}

#[test]
#[cfg(feature = "impl-serde")]
fn serde_u1368() {
	#[derive(Debug, Serialize, Deserialize)]
	pub struct Foo {
    	pub u1368: U1368,
	}
	let u1368 = r#"{"u1368": "0x0fbac47922e6e0649343400231a15e26f4f5ab1490fa5e243470de6ca26fd3583b7fa03170600a37b29d214fa618a32d6c2a121552f556578097176bf2ccb9dee0f37e8547d8f5981b6b998f99bf24c92e08b61ca5a7da5ab3da43986881356af9ad55e9b9481432cb1194a7c1302bc72500ba277941fcb9ac8063a9b6ed64fbc86c51dd5ae6cf1f01f7bcf533cf0b0cfc5dc3fdc5bc7eaa99366ada5e7127331b862586a46c12a85f9580"}"#;

	let deserialized: Foo = serde_json::from_str(u1368).unwrap();
	assert_eq!(format!("0x{:02x}", deserialized.u1368), "0x0fbac47922e6e0649343400231a15e26f4f5ab1490fa5e243470de6ca26fd3583b7fa03170600a37b29d214fa618a32d6c2a121552f556578097176bf2ccb9dee0f37e8547d8f5981b6b998f99bf24c92e08b61ca5a7da5ab3da43986881356af9ad55e9b9481432cb1194a7c1302bc72500ba277941fcb9ac8063a9b6ed64fbc86c51dd5ae6cf1f01f7bcf533cf0b0cfc5dc3fdc5bc7eaa99366ada5e7127331b862586a46c12a85f9580");
}

