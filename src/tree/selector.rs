use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Selector {
	last: usize,
}

#[methods]
impl Selector {
	pub fn new(_owner: &Node) -> Self {
		Self { last: 0 }
	}

	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}

	fn register(_builder: &ClassBuilder<Self>) {
	}
}

impl Tick for Selector {
	unsafe fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		for i in self.last..owner.get_child_count() as usize {
			
			let child = owner.get_child(i as i64).unwrap().assume_safe();
			
			match child.call("run", &[Variant::from_object(manager)]).to_tree_node_state() {
				TreeNodeState::SUCCESS => {
					self.last = 0;
					return TreeNodeState::SUCCESS;
				},
				TreeNodeState::FAILURE => continue,
				TreeNodeState::BUSY => {
					self.last = i;
					return TreeNodeState::BUSY;
				}
			}
		}
		self.last = 0;
		return TreeNodeState::FAILURE;
	}
}

impl TreeNode for Selector {
	

	fn describe(&self) -> String {
		String::from("Runs all its children until one returns SUCCES. If no children return SUCCES, return FAILURE. If a child returns BUSY that child the iteration resumes at that child during the next tick (The Selector returns BUSY)")
	}
	
	fn tree_node_type(&self) -> TreeNodeType {
		TreeNodeType::COLLECTION
	}
}