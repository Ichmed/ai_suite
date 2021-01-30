use gdnative::prelude::*;
use gdnative::nativescript::init::property::hint::*;

use crate::base::*;
use crate::blackboard;
use crate::utility::get_pos;

use crate::tree::base::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Has {
	#[property]
	key: String,
}

#[methods]
impl Has {
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
	
	pub fn new(_owner: &Node) -> Self {
		Self {key: String::from("key")}
	}
}

impl Tick for Has {
	unsafe fn tick(&mut self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		let x = manager.assume_safe().call("get", &[self.key.to_variant()]);
		TreeNodeState::from_bool(!x.is_nil())
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct IsTrue {
	#[property]
	key: String,
}

#[methods]
impl IsTrue {
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
	
	pub fn new(_owner: &Node) -> Self {
		Self {key: String::from("key")}
	}
}

impl Tick for IsTrue {
	unsafe fn tick(&mut self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		blackboard!(vec![self.key.to_variant()], manager.assume_safe())[0].to_tree_node_state()
	}
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CompareModeNumber {
	LESS,
	LESS_EQUAL,
	EQUAL,
	GREATER_EQUAL,
	GREATER
}

impl CompareModeNumber {
	fn from_string<T: ToString>(s: T) -> Self {
		match s.to_string().as_str() {
			"LESS" => Self::LESS,
			"LESS_EQUAL" => Self::LESS_EQUAL,
			"EQUAL" => Self::EQUAL,
			"GREATER_EQUAL" => Self::GREATER_EQUAL,
			"GREATER" => Self::GREATER,
			_ => panic!("unsuported string")
		}
	}

	fn compare<T: PartialOrd>(self, a: T, b: T) -> bool {
		match self {
			CompareModeNumber::LESS =>			a < b,
			CompareModeNumber::LESS_EQUAL =>	a <= b,
			CompareModeNumber::EQUAL =>			a == b,
			CompareModeNumber::GREATER_EQUAL =>	a >= b,
			CompareModeNumber::GREATER =>		a > b
		}
	}
}

impl ToString for CompareModeNumber {
	fn to_string(&self) -> String {
		match self {
			Self::LESS =>			String::from("LESS"),
			Self::LESS_EQUAL =>		String::from("LESS_EQUAL"),
			Self::EQUAL =>			String::from("EQUAL"),
			Self::GREATER_EQUAL =>	String::from("GREATER_EQUAL"),
			Self::GREATER =>		String::from("GREATER")
		}
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct CompareInt {
	#[property]
	value: i64,
	#[property]
	key: String,
	compare_mode: CompareModeNumber
}

#[methods]
impl CompareInt {
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
	
	pub fn new(_owner: &Node) -> Self {
		Self {value: 0, key: String::from("key"), compare_mode: CompareModeNumber::EQUAL}
	}

	fn get_mode(&self, _owner: TRef<Node>) -> String {
		self.compare_mode.to_string()
	}

	fn set_mode(&mut self, _owner: TRef<Node>, s: String) {
		self.compare_mode = CompareModeNumber::from_string(s)
	}

	pub fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("mode")
			.with_default(String::from("EQUAL"))
			.with_getter(Self::get_mode)
			.with_setter(Self::set_mode)
			.with_hint(StringHint::Enum(EnumHint::new(vec![
				"LESS".into(),
				"LESS_EQUAL".into(),
				"EQUAL".into(),
				"GREATER_EQUAL".into(),
				"GREATER".into(),
			])))
			.done();
	}
}

impl Tick for CompareInt {
	unsafe fn tick(&mut self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		let x = blackboard!(vec![self.key.to_variant()], manager.assume_safe())[0].clone();
		if !(x.get_type() == VariantType::I64) { return TreeNodeState::FAILURE; }

		let x = x.to_i64();

		TreeNodeState::from_bool(self.compare_mode.compare(x, self.value))

	}
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct CompareFloat {
	#[property]
	value: f64,
	#[property]
	key: String,
	compare_mode: CompareModeNumber
}

#[methods]
impl CompareFloat {
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
	
	pub fn new(_owner: &Node) -> Self {
		Self {value: 0.0, key: String::from("key"), compare_mode: CompareModeNumber::EQUAL}
	}

	fn get_mode(&self, _owner: TRef<Node>) -> String {
		self.compare_mode.to_string()
	}

	fn set_mode(&mut self, _owner: TRef<Node>, s: String) {
		self.compare_mode = CompareModeNumber::from_string(s)
	}

	pub fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("mode")
			.with_default(String::from("EQUAL"))
			.with_getter(Self::get_mode)
			.with_setter(Self::set_mode)
			.with_hint(StringHint::Enum(EnumHint::new(vec![
				"LESS".into(),
				"LESS_EQUAL".into(),
				"EQUAL".into(),
				"GREATER_EQUAL".into(),
				"GREATER".into(),
			])))
			.done();
	}
}

impl Tick for CompareFloat {
	unsafe fn tick(&mut self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		let x = blackboard!(vec![self.key.to_variant()], manager.assume_safe())[0].clone();
		if !(x.get_type() == VariantType::F64) { return TreeNodeState::FAILURE; }

		let x = x.to_f64();

		TreeNodeState::from_bool(self.compare_mode.compare(x, self.value))

	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum CompareModeString {
	EQUAL,
	PREFIX,
	POSTFIX,
	CONTAINS
}

impl CompareModeString {
	fn from_string<T: ToString>(s: T) -> Self {
		match s.to_string().as_str() {
			"EQUAL" =>		Self::EQUAL,
			"PREFIX" =>		Self::PREFIX,
			"POSTFIX" =>	Self::POSTFIX,
			"CONTAINS" =>	Self::CONTAINS,
			_ => panic!("unsuported string")
		}
	}

	fn compare<T1: ToString, T2: ToString>(&self, a: T1, b: T2) -> bool {
		match self {
			Self::EQUAL => a.to_string() == b.to_string(),
			Self::PREFIX => a.to_string().starts_with(&b.to_string()),
			Self::POSTFIX => a.to_string().ends_with(&b.to_string()),
			Self::CONTAINS => a.to_string().contains(&b.to_string()),
		}
	}
}

impl ToString for CompareModeString {
	fn to_string(&self) -> String {
		match self {
			Self::EQUAL =>		String::from("EQUAL"),
			Self::PREFIX =>		String::from("PREFIX"),
			Self::POSTFIX =>	String::from("POSTFIX"),
			Self::CONTAINS =>	String::from("CONTAINS")
		}
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct CompareString {
	#[property]
	value: f64,
	#[property]
	key: String,
	compare_mode: CompareModeString
}

#[methods]
impl CompareString {
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
	
	pub fn new(_owner: &Node) -> Self {
		Self {value: 0.0, key: String::from("key"), compare_mode: CompareModeString::EQUAL}
	}

	fn get_mode(&self, _owner: TRef<Node>) -> String {
		self.compare_mode.to_string()
	}

	fn set_mode(&mut self, _owner: TRef<Node>, s: String) {
		self.compare_mode = CompareModeString::from_string(s)
	}

	pub fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("mode")
			.with_default(String::from("EQUAL"))
			.with_getter(Self::get_mode)
			.with_setter(Self::set_mode)
			.with_hint(StringHint::Enum(EnumHint::new(vec![
				"LESS".into(),
				"LESS_EQUAL".into(),
				"EQUAL".into(),
				"GREATER_EQUAL".into(),
				"GREATER".into(),
			])))
			.done();
	}
}

impl Tick for CompareString {
	unsafe fn tick(&mut self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		let x = blackboard!(vec![self.key.to_variant()], manager.assume_safe())[0].clone();
		if !(x.get_type() == VariantType::F64) { return TreeNodeState::FAILURE; }

		let x = x.to_f64();

		TreeNodeState::from_bool(self.compare_mode.compare(x, self.value))
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct CompareDistance {
	#[property]
	value: f64,
	#[property]
	key: String,
	compare_mode: CompareModeNumber
}

#[methods]
impl CompareDistance {
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}
	
	pub fn new(_owner: &Node) -> Self {
		Self {value: 0.0, key: String::from("key"), compare_mode: CompareModeNumber::EQUAL}
	}

	fn get_mode(&self, _owner: TRef<Node>) -> String {
		self.compare_mode.to_string()
	}

	fn set_mode(&mut self, _owner: TRef<Node>, s: String) {
		self.compare_mode = CompareModeNumber::from_string(s)
	}

	pub fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("mode")
			.with_default(String::from("EQUAL"))
			.with_getter(Self::get_mode)
			.with_setter(Self::set_mode)
			.with_hint(StringHint::Enum(EnumHint::new(vec![
				"LESS".into(),
				"LESS_EQUAL".into(),
				"EQUAL".into(),
				"GREATER_EQUAL".into(),
				"GREATER".into(),
			])))
			.done();
	}
}

use crate::utility::get_closest_node_in_group;

impl Tick for CompareDistance {
	unsafe fn tick(&mut self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		let x = blackboard!(vec![self.key.to_variant()], manager.assume_safe())[0].clone();
		let v;
		
		if x.get_type() == VariantType::Vector2 { v = Some(Vector3::new(x.to_vector2().x, x.to_vector2().y, 0.0)) }
		else if x.get_type() == VariantType::Vector3 { v = Some(x.to_vector3().clone()) }
		else if x.get_type() == VariantType::GodotString {
			let source = get_pos(manager.assume_safe().get_parent().unwrap()).unwrap();
			let target = get_closest_node_in_group(source, manager.assume_safe(), x.to_string(), self.value as f32);
			v = match target {
				Some(t) => get_pos(t),
				None => {
					if self.compare_mode == CompareModeNumber::GREATER || self.compare_mode == CompareModeNumber::GREATER_EQUAL {
						return TreeNodeState::SUCCESS;
					} else {
						None
					}
				}
			}
		}	
		else { 
			v = match x.try_to_object::<Node>() {
				Some(n) => get_pos(n),
				None => None
			}
		}
		
		match v {
			Some(position) => {
				let source = get_pos(manager.assume_safe().get_parent().unwrap()).unwrap();
				TreeNodeState::from_bool(self.compare_mode.compare((source - position).square_length(), (self.value * self.value) as f32))
			},
			None => TreeNodeState::FAILURE
		}

	}
}

impl TreeNode for CompareDistance {
	
	fn describe(&self) -> String {
		String::from("Decide on the distance to a point, node or the closest node in a group")
	}
}

