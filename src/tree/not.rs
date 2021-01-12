use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Not {
}

#[methods]
impl Not {
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
	
	pub fn new(_owner: &Node) -> Not {
		Self {}
	}
	
	fn register(_builder: &ClassBuilder<Self>) {
	}
}

impl Tick for Not {
	unsafe fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		match owner.get_child(0).unwrap().assume_safe().call("run", &[Variant::from_object(manager)]).to_tree_node_state() {
			TreeNodeState::SUCCESS => TreeNodeState::FAILURE,
			TreeNodeState::BUSY => TreeNodeState::BUSY,
			TreeNodeState::FAILURE => TreeNodeState::SUCCESS
		}
	}
}

impl TreeNode for Not {
	
	fn describe(&self) -> String {
		String::from("Inverts SUCCESS and FAILURE, forwards BUSY")
	}

	fn tree_node_type(&self) -> TreeNodeType {
		TreeNodeType::DECORATOR
	}
}