use std::collections::BTreeMap;

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
pub struct Pallet {
	// A simple storage mapping from accounts (`&'static str`) to their balances (`u128`).
	/* TODO: Add a field `balances` which is a `BTreeMap` fom `&'static str` to `u128`. */
}

impl Pallet {
	/// Create a new instance of the balances module.
	pub fn new() -> Self {
		/* TODO: Return a new instance of the `Pallet` struct. */
	}
}
