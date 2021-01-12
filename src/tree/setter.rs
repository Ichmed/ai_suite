use gdnative::prelude::*;
use crate::base::*;
use crate::tree::base::*;


#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]

pub struct Setter {
	name: String,
	value: VariantArray<Shared>
}

#[methods]
impl Setter {
	fn new(_owner: &Node) -> Self {
		Self { name: String::new(), value: VariantArray::default() }
	}

	fn get_name(&self, _owner: TRef<Node>) -> String {
		self.name.clone()
	}

	fn set_name(&mut self, _owner: TRef<Node>, name: String) {
		self.name = name;
	}

	fn get_value(&self, _owner: TRef<Node>) -> VariantArray {
		self.value.duplicate().into_shared()
	}

	fn set_value(&mut self, _owner: TRef<Node>, value: VariantArray) {
		self.value = value;
	}

	fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("name")
			.with_default(String::new())
			.with_getter(Self::get_name)
			.with_setter(Self::set_name)
			.done();

		builder.add_property("value")
			.with_default(VariantArray::default())
			.with_getter(Self::get_value)
			.with_setter(Self::set_value)
			.done();
	}
	
	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Variant) -> TreeNodeState {
		self.tick(owner, manager.try_to_object().unwrap())
	}

	#[export]
	unsafe fn _get_configuration_warning(&mut self, _owner: TRef<Node>) -> String {
		if self.name == "" {
			String::from("Invalid name")
		} else if self.value.len() != 1 {
			String::from("Must have exactly one entry in value")
		} else {
			String::from("")
		}
	}

	
}

impl Tick for Setter {
	unsafe fn tick(&mut self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		let manager = manager.assume_safe();

		// call_on_base!(manager, AIManager, AIManager::insert, manager, self.name.clone(), self.value.get(0));

		let mut x = self.value.get(0);

		x = match x.try_to_string() {
			Some(s) => match s.strip_prefix("#") {
				Some(px) => manager.call("get", &[Variant::from_str(px)]),
				None => x
			},
			None => x
		};

		manager.call("insert", &[self.name.clone().to_variant(), x]);
		TreeNodeState::SUCCESS
	}
	
}

impl TreeNode for Setter {
	
	fn describe(&self) -> String {
		String::from("Set a Value in the blackboard and return SUCCESS")
	}
}