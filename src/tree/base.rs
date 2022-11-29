use crate::base::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TreeNodeType {
	LEAF, //No children
	DECORATOR, //One child
	COLLECTION, //List of children
}

impl TreeNodeType {
	#[allow(dead_code)]
	pub fn has_children(&self) -> bool {
		match self {
			Self::LEAF => false,
			_ => true,
		}
	}
}

pub trait TreeNode: Tick {

	// fn tick(&mut self, manager: &mut Box<AIData>, owner: Option<&Node>) -> TreeNodeState;
	
	fn describe(&self) -> String {
		String::from("This TreeNode has no associated description, implement the describe method to display a TreeNode specific description")
	}

	fn tree_node_type(&self) -> TreeNodeType {
		TreeNodeType::LEAF
	}
	
	fn has_children(&self) -> bool {
		self.tree_node_type().has_children()
	}
}