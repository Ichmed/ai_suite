use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
struct BusyCounter {
	max: usize,
	current: usize,
}

#[methods]
impl BusyCounter {
	pub fn new (owner: &Node) -> Self {
		Self { 
			max: 1,
			current: 0,
		}
	}
	
	fn set_max(&mut self, max: usize) {
		self.max = max;
	}

	fn get_max(&self) -> usize {
		self.max
	}

	fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("max")
			.with_default(1)
			.with_getter(Self::get_max)
			.with_setter(Self::set_max)
			.done();
	}
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
}

impl Tick for BusyCounter {
	fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		self.current += 1;
		if self.current == self.max {
			self.current = 0;
			return TreeNodeState::SUCCESS
		}
		TreeNodeState::BUSY
	}
}

impl TreeNode for BusyCounter {	
	fn describe(&self) -> String {
		String::from("Counts up and returns BUSY every tick until a maximum value is reached, then resets the counter and returns SUCCESS (never returns FAILURE)")
	}
}