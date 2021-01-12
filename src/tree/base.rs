use crate::base::*;


// #[derive(NativeClass)]
// #[inherit(Node)]
// #[register_with(Self::register_signals)]
// pub struct Tree {
// 	head: Option<Box<dyn TreeNode>>,
// 	pub manager: Box<AIData>,
// }

// #[methods]
// impl Tree {
// 	#[export]
// 	fn _process(&mut self, owner: &Node, _delta: f64) {
		
// 		println!("{}", owner.get_class());

// 		let x = self.tick(Some(owner));

// 		match x {
// 			TreeNodeState::SUCCESS	=> owner.emit_signal("on_success",	&[]),
// 			TreeNodeState::FAILURE	=> owner.emit_signal("on_failure",	&[]),
// 			TreeNodeState::BUSY		=> owner.emit_signal("on_busy",		&[])
// 		};

// 		owner.emit_signal("tick", &[Variant::from_str(format!("{:?}", x))]);
// 	}

// 	#[export]
// 	fn write_to_blackboard(&mut self, owner: &Node, name: Variant, value: Variant) -> bool
// 	{
// 		self.manager.insert(name.to_string(), value.clone());

// 		owner.emit_signal("on_blackboard_update", &[name, value]);

// 		return true;
// 	}

// 	#[export]
// 	fn update_blackboard(&mut self, _owner: &Node, map: Variant) {

// 		match map.try_to_dictionary() {
// 			Some(map) => for (key, value) in map.iter() {
// 				self.manager.insert(key.to_string(), value);
// 			},
// 			None => ()
// 		}

// 	}
	
// 	#[export]
// 	pub fn tick(&mut self, owner: Option<&Node>) -> TreeNodeState {
// 		match &mut self.head {
// 			Some(head) => {
// 				head.tick(&mut self.manager, owner)
// 			}
// 			None => {
// 				godot_error!("tickning behaviour_tree without head (construction might have failed)");
// 				TreeNodeState::FAILURE
// 			}
// 		}
// 	}

// 	fn new(owner: &Node) -> Self {

// 		Self {
//             head: None,
// 			manager: Box::new(AIData::new()),
//         }
// 	}
	
// 	#[export]
// 	fn _ready(&mut self, owner: &Node) {
// 		self.head = unsafe { TreeNode::from_node(owner.get_child(0).unwrap()) };
// 	}
	
// 	pub fn godot_describe(&self, owner: TRef<Node, Shared>) -> &str {
// 		self.describe()
// 	}

// 	// pub fn from_json(json: String) -> Self {
// 	// 	let d: Value = serde_json::from_str(&json).unwrap();

// 	// 	Self {
// 	// 		head: Some(TreeNode::from_json(d.as_object().unwrap())),
// 	// 		blackboard: Box::new(HashMap::<String, Variant>::new()),
// 	// 	}
// 	// }

// 	fn register_signals(builder: &ClassBuilder<Self>) {
// 		empty_signal!(builder, "tick");
// 		empty_signal!(builder, "on_success");
// 		empty_signal!(builder, "on_busy");
// 		empty_signal!(builder, "on_failure");
		
// 		empty_signal!(builder, "on_blackboard_update");
// 	}
// }

// impl dyn TreeNode {
// 	pub unsafe fn from_node(node: Ref<Node, Shared>) -> Option<Box<dyn TreeNode>> {
		
// 		match node.assume_safe().get("type").try_to_string() {
// 			Some(t) => match t.as_str() {
// 				"Not"		=> Not::from_node(node),
// 				"Stub"		=> Stub::from_node(node),
// 				"Selector"	=> Selector::from_node(node),
// 				"Sequence"	=> Sequence::from_node(node),

// 				"Overrider"	=> Sequence::from_node(node),
// 				"Emitter"	=> Emitter::from_node(node),
// 				"BusyWaiter"=> BusyWaiter::from_node(node),
// 				"Caller"	=> Caller::from_node(node),
// 				"Setter"	=> Setter::from_node(node),
				
// 				"StatePicker"=> StatePicker::from_node(node),
// 				_ => None
// 			},
// 			None => None
// 		}
// 	}
// }

// impl Tick for Tree {
// 	fn tick(&mut self, manager: &mut Box<AIData>, owner: Option<&Node>) -> TreeNodeState {
		
// 		match &mut self.head {
// 			Some(head) => {
// 				head.tick(manager, owner)
// 			}
// 			None => {
// 				godot_error!("tickning behaviour_tree without head (construction might have failed)");
// 				TreeNodeState::FAILURE
// 			}
// 		}
// 	}
// }

// impl TreeNode for Tree {
	
// 	fn describe(&self) -> &str {
// 		&"Encapsulates a Behaviour tree. Behaviour trees may be used as Leaf nodes in other trees. The returned state is the returned state of this trees head Node. If Behaviour Tree is used as a node it will use its parent's blackboard."
// 	}
// }


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



// pub struct Executor<T> {
// 	target: fn(&Box<AIData>) -> Option<T>,
// 	name: String,
// 	write_to_blackboard: bool,
// }

// #[allow(dead_code)]
// impl<T> Executor<T> {
// 	pub fn make_void(function: fn(&Box<AIData>) -> Option<T>) -> Executor<T> {
// 		Executor {
// 			target: function,
// 			name: String::new(),
// 			write_to_blackboard: false,
// 		}
// 	}

// 	pub fn make(function: fn(&Box<AIData>) -> Option<T>, name: String) -> Executor<T> {
// 		Executor {
// 			target: function,
// 			name: name,
// 			write_to_blackboard: true,
// 		}
// 	}
// }

// impl Tick for Executor 

// impl<T: Serialize> TreeNode for Executor<T> {
// 	fn tick(&mut self, manager: &mut Box<AIData>, _owner: Option<&Node>) -> TreeNodeState {
// 		let x = self.target;
// 		let x: T = match x(manager) {
// 			Some(x) => x,
// 			None => return TreeNodeState::FAILURE
// 		};
		
// 		// if self.write_to_manager {
// 		// 	let b = manager;
// 		// 	b.insert(self.name.clone(), value_to_variant(Value::from(x).unwrap()).unwrap());
// 		// }

// 		TreeNodeState::SUCCESS
// 	}
	
// 	fn describe(&self) -> &str {
// 		&"Call a rust function that takes a HashMap(the blaackboard) as an argument and write the result to the blackboard. returns SUCCESS if there was a result and FAILURE otherwise"
// 	}
// }



// macro_rules! if_node {
// 	($n:ident, $p:path, $f:path) => (
// 		struct $n {
// 			name: String,
// 			value: Variant
// 		}
		
// 		#[allow(dead_code)]
// 		impl $n {
// 			pub fn new(name: String, value: Variant) -> Self {
// 				Self { name, value }
// 			}
// 		}

// 		impl Tick for $n {
// 			fn tick(&mut self, manager: &mut Box<AIData>, _owner: Option<&Node>) -> TreeNodeState
// 			{
// 				match manager.get(self.name.clone()) {
// 					Some(x) => {
// 							let a = match $p(&x) {
// 								Some(a) => a,
// 								None => return TreeNodeState::FAILURE
// 							};

// 							let b = match $p(&self.value) {
// 								Some(a) => a,
// 								None => return TreeNodeState::FAILURE
// 							};


						
// 							if $f(&a, &b) {
// 							TreeNodeState::SUCCESS
// 							} else {
// 							TreeNodeState::FAILURE
// 						}}
// 					_ => TreeNodeState::FAILURE
// 				}
// 			}
// 		}
// 	)
// }

// // String operators
// if_node!(StringEqual, Variant::try_to_string, String::eq);
// if_node!(StringUnEqual, Variant::try_to_string, String::ne);
// // if_node!(StringStartsWith, Variant::to_string, String::starts_with);

// // Int operators
// if_node!(IntEqual, Variant::try_to_i64, i64::eq);

// struct If {
// 	name: String,
// }

// impl If {
// 	fn new(name: String) -> Self {
// 		If { name: name }
// 	}
// }

// impl Tick for If {
// 	fn tick(&mut self, manager: &mut Box<AIData>, _owner: Option<&Node>) -> TreeNodeState {
// 		match manager.get(self.name.clone()) {
// 			Some(x) => match x.try_to_bool() {
// 				Some(b) => match b {
// 					true => TreeNodeState::SUCCESS,
// 					false => TreeNodeState::FAILURE
// 				}
// 				_ => TreeNodeState::FAILURE
// 			}
// 			_ => TreeNodeState::FAILURE
// 		}
// 	}
// }



// struct Overrider {
// 	result: TreeNodeState,
// 	child: Box<dyn TreeNode>
// }

// impl Overrider {
// 	pub fn new(child: Box<dyn TreeNode>, result: TreeNodeState) -> Option<Self> {
// 		match result {
// 			TreeNodeState::BUSY => {
// 				println!("Overriders should never return BUSY as this will cause the tree to freeze");
// 				None
// 			},
// 			_ => Some(Self { child, result })
// 		}
// 	}
// }

// impl Tick for Overrider {
// 	fn tick(&mut self, manager: &mut Box<AIData>, owner: Option<&Node>) -> TreeNodeState {
// 		self.child.tick(manager, owner);
// 		self.result
// 	}
// }

// impl TreeNode for Overrider {
// 	fn describe(&self) -> &str {
// 		&"Replace the result of the child node with a static result. Can not return BUSY because that would cause the tree to freeze"
// 	}
	
// 	fn tree_node_type(&self) -> TreeNodeType {
// 		TreeNodeType::DECORATOR
// 	}
// }

// struct BusyWaiter {
// 	counter: i64,
// 	max: i64,
// 	child: Box<dyn TreeNode>
// }

// impl BusyWaiter {
// 	pub fn new(child: Box<dyn TreeNode>, max: i64) -> Self {
// 		Self { counter: 0, max, child}
// 	}

// 	pub unsafe fn from_node(node: Ref<Node, Shared>) -> Option<Box<dyn TreeNode>> {
// 		let node = node.assume_safe();
// 		match node.get_child_count() {
// 			0 => None,
// 			1 => match TreeNode::from_node(node.get_child(0).unwrap()) {
// 				Some(child) => Some(Box::new(BusyWaiter::new(child, node.get("max").to_i64()))),
// 				None => None,
// 			}
// 			_ => None
// 		}
// 	}
// }

// impl Tick for BusyWaiter {
// 	fn tick(&mut self, manager: &mut Box<AIData>, owner: Option<&Node>) -> TreeNodeState {
// 		match self.child.tick(manager, owner) {
// 			TreeNodeState::SUCCESS => {
// 				self.counter = 0;
// 				TreeNodeState::SUCCESS
// 			},
// 			TreeNodeState::BUSY => {
// 				self.counter = 0;
// 				TreeNodeState::BUSY
// 			},
// 			TreeNodeState::FAILURE => {
// 				if self.max == 0 {
// 					TreeNodeState::BUSY
// 				}
// 				else {
// 					if self.counter == self.max {
// 						self.counter = 0;
// 						TreeNodeState::FAILURE
// 					}
// 					else {
// 						self.counter += 1;
// 						TreeNodeState::BUSY
// 					}
// 				}
// 			}
// 		}
// 	}
// }

// impl TreeNode for BusyWaiter {
// 	fn describe(&self) -> &str {
// 		&"Return BUSY if the child returns FAILURE (used to wait on a node that can not return BUSY). A maximum of times BUSY can be returned may be set, after which FAILURE will be returned an the counter resets. SUCCESS and BUSY are forwarded and reset the counter"
// 	}
	
// 	fn tree_node_type(&self) -> TreeNodeType {
// 		TreeNodeType::DECORATOR
// 	}
// }



// struct StatePicker {
// 	paths: HashMap<String, Box<dyn TreeNode>>
// }

// impl StatePicker {
// 	pub unsafe fn from_node(node: Ref<Node, Shared>) -> Option<Box<dyn TreeNode>> {
// 		let node = node.assume_safe();

// 		let mut m = HashMap::new();

// 		for child in node.get_children().iter() {
// 			let name = node.get("name").to_string();
// 			match TreeNode::from_node(child.try_to_object().unwrap()) {
// 				Some(n) => {
// 					m.insert(name, n);
// 				},
// 				None => return None
// 			}
// 		}

// 		Some(Box::new(StatePicker {paths: m}))
// 	}
// }

// impl Tick for StatePicker {
// 	fn tick(&mut self, manager: &mut Box<AIData>, owner: Option<&Node>) -> TreeNodeState {

// 		match &manager.state_machine {
// 			Some(state_machine) => 	match state_machine.get_state() {
// 				Some(state) => match self.paths.get_mut(&state) {
// 					Some(path) => path.tick(manager, owner),
// 					None => {
// 						match self.paths.get_mut("_") {
// 							Some(path) => path.tick(manager, owner),
// 							None => return TreeNodeState::FAILURE
// 						}
// 					}
// 				},
// 				None => TreeNodeState::FAILURE
// 			},
// 			None => return TreeNodeState::FAILURE
// 		}
// 	}
// }

// impl TreeNode for StatePicker {
	

// 	fn describe(&self) -> &str {
// 		&"Runs the child node with the same node as the AIDatas current state. A child called '_' acts as the default if no fitting child is found. If no matching child is found and there is no default child, the manager has no StateMachine or the StateMAchine is empty FAILURE is returned"
// 	}

// 	fn tree_node_type(&self) -> TreeNodeType {
// 		TreeNodeType::COLLECTION
// 	}
// }