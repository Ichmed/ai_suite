use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;
use crate::utility::get_pos;

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct TargetClosest {
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
impl TargetClosest {

	fn new(_owner: &Node) -> Self {
		TargetClosest { group: String::new(), name: String::from("target"), is2d: true, range: 0.0 }
	}
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}

}

impl Tick for TargetClosest {
	unsafe fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		
		let manager = manager.assume_safe();

		let source = get_pos(manager.get_parent().unwrap()).unwrap();

		let range = self.range * self.range;

		let r = owner.get_tree()
			.unwrap()
			.assume_safe()
			.get_nodes_in_group(self.group.clone())
			.iter()
			.map(|x| get_pos(x.try_to_object().unwrap()))
			.filter(|x| x.is_some() && (range <= 0.0 || (x.unwrap() - source).square_length() < range))
			.fold(None, |acc, x| match x {
				Some(position) => match acc {
					None => Some(position),
					Some(acc_pos) => if (source - acc_pos).square_length() > (source - position).square_length() {
							Some(position)
						} else {
							Some(acc_pos)
						}
				},
				None => acc
			});

		match r {
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
	}
}

impl TreeNode for TargetClosest {
	
	fn describe(&self) -> String {
		String::from("Write the position of the closest node in a given group to the blackboard")
	}

	fn tree_node_type(&self) -> TreeNodeType {
		TreeNodeType::DECORATOR
	}
}




	