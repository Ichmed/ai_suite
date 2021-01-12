use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;

use gdnative::nativescript::init::property::hint::*;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Stub {
	result: TreeNodeState,
}


#[methods]
impl Stub {
	pub fn new(_owner: &Node) -> Stub {
		Stub { result: TreeNodeState::SUCCESS }
	}

	fn get_result(&self, _owner: TRef<Node>) -> String {
		self.result.to_string()
	}

	fn set_result(&mut self, _owner: TRef<Node>, result: String) {
		self.result = TreeNodeState::from_string(result);
	}
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}

	fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("result")
			.with_default(TreeNodeState::SUCCESS.to_string())
			.with_getter(Self::get_result)
			.with_setter(Self::set_result)
			.with_hint(StringHint::Enum(EnumHint::new(vec!["SUCCESS".into(), "FAILURE".into(), "BUSY".into()])))
			.done();
			
	}

}

impl Tick for Stub {
	unsafe fn tick(&mut self, _owner: TRef<Node>, _manager: Ref<Node, Shared>) -> TreeNodeState {
		self.result
	}
}

impl TreeNode for Stub {

	fn describe(&self) -> String {
		String::from("A TreeNode that always returns the same TreeNodeState")
	}

	fn tree_node_type(&self) -> TreeNodeType {
		TreeNodeType::LEAF
	}
}