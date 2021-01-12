use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;
use crate::empty_signal;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct Emitter {
	#[property]
	values: VariantArray
}

#[methods]
impl Emitter {
	fn new(_owner: &Node) -> Self {
		Self {
			values: VariantArray::default()
		}
	}

	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
	
	fn register(builder: &ClassBuilder<Self>) {
		empty_signal!(builder, "call");
	}
}

impl Tick for Emitter {
	unsafe fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {

		let manager = manager.assume_safe();
		

		owner.emit_signal("call", self.values.iter().map(|x| match x.try_to_string() {
			Some(s) => match s.strip_prefix("#") {
				Some(np) => manager.call("get", &[np.to_string().to_variant()]),
				None => x.clone()
			},
			None => x.clone(),
		}).collect::<Vec<Variant>>().as_slice());

		TreeNodeState::SUCCESS
	}
}

impl TreeNode for Emitter {
	fn describe(&self) -> String {
		String::from("Emit a Godot Signal using the provided names to retrieve values from the blackboard. Return Failure if the Tree has no owning Godot Node and SUCCESS otherwise (independend of what happens to the signal)")
	}
	
	fn tree_node_type(&self) -> TreeNodeType {
		TreeNodeType::LEAF
	}
}