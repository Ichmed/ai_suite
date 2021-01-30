use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;

use crate::blackboard;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Caller {
	#[property]
	method_name: String,
	#[property]
	values: VariantArray,
	#[property]
	path: String,
	#[property]
	target: String,
	#[property]
	evaluate_result: bool,
}

#[methods]
impl Caller {
	fn new(_owner: &Node) -> Self {
		Self { 
			method_name: "f".to_string(),
			values: VariantArray::default(),
			path: String::from(".."),
			target: String::new(),
			evaluate_result: true,
		}
	}

	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}

}

impl Tick for Caller {
	unsafe fn tick(&mut self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {

		let manager = manager.assume_safe();
		
		let node = match manager.get_node(self.path.clone()) {
			Some(node) => node,
			None => {
				godot_error!("Could not find node {}/{}", manager.get_path().to_string(), self.path);
				return TreeNodeState::FAILURE;
			}
		}.assume_safe();

		if !node.has_method(self.method_name.clone()) {
			godot_error!("Calling unknown method {} on {}", self.method_name, self.path.to_string());
			return TreeNodeState::FAILURE;
		}

		let r = node.call(self.method_name.clone(), &blackboard!(self.values, manager));

		if self.target != "" {
			manager.call("insert", &[Variant::from_str(self.target.clone()), r.clone()]);
		}

		// godot_print!("{:?} -> {:?}", r, r.to_tree_node_state());

		if !self.evaluate_result {
			return TreeNodeState::SUCCESS
		}

		if r.get_type() == VariantType::Nil {
			return TreeNodeState::SUCCESS;
		}

		r.to_tree_node_state()
	}
}

impl TreeNode for Caller {
	fn describe(&self) -> String {
		String::from("Call any Godot function on any Node. The path to the node is given relative *TO THE AIManager* (allows Caller nodes to be moved without adjusting Nodepath).
		If evaluate_result is set the engine will try to coerce the result into a TreeNodeState which is then returned, otherwise the result is ignored an SUCCES is returned.
		If target is set to a blackboard path the result of the function will be written to the blackboard")
	}
	
}