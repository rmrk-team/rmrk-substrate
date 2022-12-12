use core::cell::Cell;

pub trait Budget {
	/// Returns true while not exceeded
	fn consume(&self) -> bool {
		self.consume_custom(1)
	}
	/// Returns true while not exceeded
	/// Implementations should use interior mutabilitiy
	fn consume_custom(&self, calls: u32) -> bool;

	fn budget_left_value(&self) -> u32 {
		self.get_budget_left_value()
	}
	fn budget_consumed_value(&self) -> u32 {
		self.get_budget_consumed_value()
	}

	fn get_budget_left_value(&self) -> u32;
	fn get_budget_consumed_value(&self) -> u32;
}

pub struct Value {
	budget_left: Cell<u32>,
	budget_consumed: Cell<u32>,
}

impl Value {
	pub fn new(v: u32) -> Self {
		Self { budget_left: Cell::new(v), budget_consumed: Cell::new(0) }
	}
	pub fn refund(self) -> u32 {
		self.budget_left.get()
	}
}

impl Budget for Value {
	fn consume_custom(&self, calls: u32) -> bool {
		let (budget_left_result, sub_overflown) = self.budget_left.get().overflowing_sub(calls);
		let (budget_consumed_result, add_overflown) =
			self.budget_consumed.get().overflowing_add(calls);
		if sub_overflown || add_overflown {
			return false
		}
		self.budget_left.set(budget_left_result);
		self.budget_consumed.set(budget_consumed_result);
		true
	}
	fn get_budget_left_value(&self) -> u32 {
		self.budget_left.get()
	}
	fn get_budget_consumed_value(&self) -> u32 {
		self.budget_consumed.get()
	}
}
