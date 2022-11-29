use gdnative::prelude::*;
use std::collections::HashMap;

use crate::state::StateMachine;
use std::cell::RefCell;

pub trait Tick {
	unsafe fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState;
}

impl dyn Tick {

}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TreeNodeState {
	SUCCESS = 1,
	FAILURE = 0,
	BUSY = 2
}

impl TreeNodeState {
	

	pub fn from_string<T: ToString>(s: T) -> Self {
		let s = s.to_string();
		let s = s.as_str();
		match s {
			"SUCCESS" => Self::SUCCESS,
			"FAILURE" => Self::FAILURE,
			"BUSY" => Self::BUSY,
			_ => Self::FAILURE
		}
	}

	pub fn from_i64(i: i64) -> Self {
		match i {
			0 => Self::FAILURE,
			1 => Self::SUCCESS,
			_ => Self::BUSY
		}
	}

	pub fn from_bool(b: bool) -> Self {
		match b {
			true => Self::SUCCESS,
			false => Self::FAILURE
		}
	}

	pub fn to_i64(&self) -> i64 {
		match self {
			Self::FAILURE => 0,
			Self::SUCCESS => 1,
			Self::BUSY => 2
		}
	}

	pub fn to_bool(&self) -> bool {
		match self {
			Self::SUCCESS => true,
			_ => false
		}
	}
}

pub trait ToTreeNodeState {
	fn to_tree_node_state(&self) -> TreeNodeState;
}

impl ToString for TreeNodeState {
	fn to_string(&self) -> String {
		match self {
			Self::SUCCESS => String::from("SUCCESS"),
			Self::FAILURE => String::from("FAILURE"),
			Self::BUSY => String::from("BUSY")
		}
	}
}

impl ToTreeNodeState for Variant {

	fn to_tree_node_state(&self) -> TreeNodeState {
		match self.get_type() {
			VariantType::Nil => TreeNodeState::FAILURE,
			VariantType::I64 => TreeNodeState::from_i64(self.to_i64()),
			VariantType::Bool => TreeNodeState::from_bool(self.to_bool()),
			VariantType::GodotString => TreeNodeState::from_string(self.to_string()),
			_ => TreeNodeState::SUCCESS
		}
	}

}

impl ToVariant for TreeNodeState {

	fn to_variant(&self) -> Variant {
		Variant::from_i64(self.to_i64())
	}

}

#[macro_export]
macro_rules! empty_signal {
	($b:ident, $e:expr) => ({
		$b.add_signal(Signal {
			name: $e,
			args: &[],
		});
	})
}

#[macro_export]
macro_rules! call_on_base {
	( $target:expr, $base:ty, $fun:expr $( , $x:expr )* ) => {{

		let f = move |x: &'_ $base, y: TRef<Node>| {
			let a = $fun(x, y, $($x,)*).clone();
			a
		};

		let r = $target.cast_instance::<$base>().unwrap().map(f).ok().unwrap().clone();

		r

	}}
}

#[macro_export]
macro_rules! call_on_base_mut {
	( $target:expr, $base:ty, $fun:expr $( , $x:expr )* ) => {{

		let f = move |x: &'_ mut $base, y: TRef<Node>| {
			let a = $fun(x, y, $($x,)*).clone();
			a
		};

		let r = $target.cast_instance::<$base>().unwrap().map_mut(f).ok().unwrap().clone();

		r

	}}
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct AIManager {
	pub data: RefCell<AIData>
}

#[methods]
impl AIManager {
	
	#[export]
	pub fn get(&self, owner: TRef<Node>, path: Variant) -> Variant {
		
		let path = &path.to_string();
		
		match self.data.borrow().get(path) {
			Some(data) => data.clone(),
			None => owner.get(path)
		}
	}

	#[export]
	pub fn insert(&self, owner: TRef<Node>, path: Variant, value: Variant) -> Option<Variant> {
		owner.emit_signal("on_blackboard_update", &[path.clone(), value.clone()]);
		self.data.borrow_mut().insert(path.to_string(), value.clone())
	}

	#[export]
	pub fn has_state_machine(&self, _owner: TRef<Node>) -> bool {
		self.data.borrow().state_machine.is_some()
	}

	#[export]
	pub fn has(&self, owner: TRef<Node>, module: String) -> bool {
		match module.to_lowercase().as_str() {
			"state_machine" => self.has_state_machine(owner),
			_ => false
		}
	}

	pub fn new(_owner: &Node) -> Self {
		AIManager { data: RefCell::new(AIData::new()) }
	}

	#[export]
	unsafe fn _process(&self, owner: TRef<Node>, _delta: Variant) {
		
		let x = owner.get_child(0).unwrap().assume_safe();

		#[allow(unused_variables)]
		let result = x.call("run", &[Variant::from_object(owner.clone())]);

		// godot_print!("{:?}", result);
	}
	
	fn register(builder: &ClassBuilder<Self>) {
		empty_signal!(builder, "on_blackboard_update");
	}
}

pub struct AIData {
	pub blackboard: HashMap<String, Variant>,
	pub state_machine: Option<StateMachine>,
}

impl AIData {

	pub fn new() -> AIData {
		Self { blackboard: HashMap::new(), state_machine: None}
	}
	
	pub fn insert(&mut self, path: String, value: Variant) -> Option<Variant> {
		self.blackboard.insert(path, value)
	}
	
	pub fn get<T: ToString>(&self, path: T) -> Option<Variant> {
		
		let path = path.to_string();
		
		if path == "state" {
			self.get_state()
		}
		else if path.starts_with("/"){
			None
		}
		else {
			match self.blackboard.get(&path) {
				Some(variant) => Some(variant.clone()),
				None => None
			}
		}
	}
	
	pub fn get_state(&self) -> Option<Variant> {
		match &self.state_machine {
			Some(state_machine) => match state_machine.get_state() {
				Some(state) => Some(Variant::from_str(state)),
				None => None
			}
			None => None,
		}		
	}
}


fn init(handle: InitHandle) {
	handle.add_class::<AIManager>();
	// handle.add_class::<crate::tree::Tree>();
	handle.add_class::<crate::state::StateMachine>();
	handle.add_class::<crate::state::State>();

	handle.add_class::<crate::tree::stub::Stub>();
	handle.add_class::<crate::tree::not::Not>();
	handle.add_class::<crate::tree::selector::Selector>();
	handle.add_class::<crate::tree::sequence::Sequence>();
	handle.add_class::<crate::tree::setter::Setter>();
	handle.add_class::<crate::tree::caller::Caller>();
	handle.add_class::<crate::tree::emitter::Emitter>();
	handle.add_class::<crate::tree::target::TargetClosestPosition>();
	handle.add_class::<crate::tree::target::TargetClosestNode>();

	handle.add_class::<crate::tree::check::Has>();
	handle.add_class::<crate::tree::check::IsTrue>();
	handle.add_class::<crate::tree::check::CompareInt>();
	handle.add_class::<crate::tree::check::CompareFloat>();
	handle.add_class::<crate::tree::check::CompareString>();
	handle.add_class::<crate::tree::check::CompareDistance>();
	
	handle.add_class::<crate::utility_map::UtilityFunction>();
	handle.add_class::<crate::utility_map::UtilityMaximizer>();
	handle.add_class::<crate::utility_map::UtilityAdder>();
	handle.add_class::<crate::utility_map::UtilityMultiplier>();
	handle.add_class::<crate::utility_map::UtilityMask>();

	handle.add_class::<crate::boid::Boid>();
	handle.add_class::<crate::boid::SimpleBoid>();
	handle.add_class::<crate::boid::SimpleBoid2D>();
}

godot_init!(init);
