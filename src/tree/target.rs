use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;
use crate::utility::{get_pos, get_closest_node};




#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct TargetClosestPosition {
	#[property]
	group: String,
	#[property]
	name: String,
	#[property (default = true)]
	is2d: bool,
	#[property (default = 0.0)]
	range: f32
}

#[methods]
impl TargetClosestPosition {

	fn new(_owner: &Node) -> Self {
		TargetClosestPosition { group: String::new(), name: String::from("target"), is2d: true, range: 0.0 }
	}
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}

}

impl Tick for TargetClosestPosition {
	unsafe fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		
		let manager = manager.assume_safe();

		let source = get_pos(manager.get_parent().unwrap()).unwrap();

		let r = owner.get_tree()
			.unwrap()
			.assume_safe()
			.get_nodes_in_group(self.group.clone());

		let r = get_closest_node(source, r.iter().map(|x| x.try_to_object::<Node>().unwrap()).collect::<Vec<Ref<Node, Shared>>>(), self.range);
			

		match r {
			Some(node) => match get_pos(node) {
				Some(position) => {
					if self.is2d {
						manager.call("insert", &[Variant::from_str(&self.name), Variant::from_vector2(&position.xy())]);
					} else {
						manager.call("insert", &[Variant::from_str(&self.name), Variant::from_vector3(&position)]);
					}
					TreeNodeState::SUCCESS				
				},
				None => TreeNodeState::FAILURE
			}
			None => TreeNodeState::FAILURE
		}
	}
}

impl TreeNode for TargetClosestPosition {
	
	fn describe(&self) -> String {
		String::from("Write the position of the closest node in a given group to the blackboard")
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct TargetClosestNode {
	#[property]
	group: String,
	#[property]
	name: String,
	#[property (default = 0.0)]
	range: f32
}

#[methods]
impl TargetClosestNode {

	fn new(_owner: &Node) -> Self {
		TargetClosestNode { group: String::new(), name: String::from("target"), range: 0.0 }
	}
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}

}

impl Tick for TargetClosestNode {
	unsafe fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		
		let manager = manager.assume_safe();

		let source = get_pos(manager.get_parent().unwrap()).unwrap();

		let r = owner.get_tree()
			.unwrap()
			.assume_safe()
			.get_nodes_in_group(self.group.clone());

		let r = get_closest_node(source, r.iter().map(|x| x.try_to_object::<Node>().unwrap()).collect::<Vec<Ref<Node, Shared>>>(), self.range);
			

		match r {
			Some(node) => {
				manager.call("insert", &[Variant::from_str(&self.name), Variant::from_object(node)]);
				TreeNodeState::SUCCESS
			}
			None => TreeNodeState::FAILURE
		}
	}
}

impl TreeNode for TargetClosestNode {
	
	fn describe(&self) -> String {
		String::from("Write the the closest node in a given group to the blackboard")
	}
}






	