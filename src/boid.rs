use std::iter::FromIterator;

use gdnative::prelude::*;
use crate::base::*;
use crate::utility::get_pos;
use crate::*;

#[derive(NativeClass)]
#[inherit(Node)]
// #[register_with(Self::register)]
pub struct Boid {
	#[property (path = "parameters/separation/base", default = 1.0)]
	separation: f32,
	#[property (path = "parameters/separation/range", default = 1.0)]
	separation_range: f32,

	#[property (path = "parameters/alignment/base", default = 1.0)]
	alignment: f32,
	#[property (path = "parameters/alignment/range", default = 1.0)]
	alignment_range: f32,

	#[property (path = "parameters/cohesion/base", default = 1.0)]
	cohesion: f32,
	#[property (path = "parameters/cohesion/range", default = 1.0)]
	cohesion_range: f32,

	#[property (default = 100.0)]
	range: f32,
	#[property]
	flock_name: String,
	#[property (path = "movement/velocity")]
	velocity: Vector3,
	#[property (path = "movement/orient", default = false)]
	orient: bool,
	#[property (path = "movement/speed", default = 100.0)]
	speed: f32,
}

unsafe fn separate(pos: Vector3, others: &Vec<Ref<Node, Shared>>, range: f32) -> Vector3 {
	others.iter()
		.map(|x| get_pos(*x).unwrap())
		.filter(|x| (*x - pos).square_length() < range)
		.fold(Vector3::zero(), |a, b| a - b + pos) / others.len().max(1) as f32

}

unsafe fn align(pos: Vector3, others: &Vec<Ref<Node, Shared>>, range: f32) -> Vector3 {
	others.iter()
		.filter(|x| (get_pos(**x).unwrap() - pos).square_length() < range)
		.map(|x| x.assume_safe().call("get", &[Variant::from_str("movement/velocity")]).to_vector3())
		.fold(Vector3::zero(), |a, b| a + b) / others.len().max(1) as f32
}

unsafe fn coalesce(pos: Vector3, others: &Vec<Ref<Node, Shared>>, range: f32) -> Vector3 {
	
	if others.len() == 0 {
		return Vector3::zero();
	}
	
	others.iter()
		.map(|x| get_pos(*x).unwrap())
		.filter(|x| (*x - pos).square_length() < range)
		.fold(Vector3::zero(), |a, b| a + b) / others.len().max(1) as f32 - pos
}

#[methods]
impl Boid {
	fn new(_owner: &Node) -> Self {
		Self { 
			separation: 1.0,
			separation_range: 1.0,

			alignment: 1.0, 
			alignment_range: 1.0,

			cohesion: 1.0,
			cohesion_range: 1.0,
			range: 200.0,
			flock_name: String::from("flock"),
			velocity: Vector3::zero(),
			orient: false,
			speed: 0.01,
		}
	}


	#[export]
	unsafe fn run(&self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		let pos = get_pos(manager.assume_safe().get_parent().unwrap()).unwrap();

		let range = self.range * self.range;
		let separation_range = range * self.separation_range * self.separation_range;
		let alignment_range = range * self.alignment_range * self.alignment_range;
		let cohesion_range = range * self.cohesion_range * self.cohesion_range;

		let others = owner.get_tree().unwrap().assume_safe().get_nodes_in_group(&self.flock_name).iter()
			.map(|x| x.try_to_object::<Node>().unwrap())
			.filter(|x| x.assume_safe().get_instance_id() != owner.get_instance_id() && {let p = get_pos(*x); p.is_some() && (p.unwrap() - pos).square_length() < range})
			.collect::<Vec<Ref<Node, Shared>>>();

		let mut velocity = manager.assume_safe().call("get", &[Variant::from_str("velocity")]).to_vector3() + 
			separate(pos, &others, separation_range) * self.separation +
			align(pos, &others, alignment_range) * self.alignment +
			coalesce(pos, &others, cohesion_range) * self.cohesion;

		let others_va = Variant::from_array(&VariantArray::from_iter(others.iter()).into_shared());

		for child in owner.get_children().iter().map(|x| x.try_to_object::<Node>().unwrap().assume_safe()) {
			let x = child.call("get_velocity_modifier", &[others_va.clone()]);
			match x.get_type() {
				VariantType::Vector3 => velocity += x.to_vector3(),
				_ => continue
			}
		}
		
		velocity = velocity.normalize() * self.speed;

		manager.assume_safe().call("insert", &[Variant::from_str("velocity"), Variant::from_vector3(&velocity)]);

		
		TreeNodeState::SUCCESS
	}
}

#[derive(NativeClass)]
#[inherit(KinematicBody)]
// #[register_with(Self::register)]
pub struct SimpleBoid {
	#[property (path = "parameters/separation/base", default = 1.0)]
	separation: f32,
	#[property (path = "parameters/separation/range", default = 1.0)]
	separation_range: f32,

	#[property (path = "parameters/alignment/base", default = 1.0)]
	alignment: f32,
	#[property (path = "parameters/alignment/range", default = 1.0)]
	alignment_range: f32,

	#[property (path = "parameters/cohesion/base", default = 1.0)]
	cohesion: f32,
	#[property (path = "parameters/cohesion/range", default = 1.0)]
	cohesion_range: f32,

	#[property (default = 100.0)]
	range: f32,
	#[property]
	flock_name: String,
	#[property (path = "movement/velocity")]
	velocity: Vector3,
	#[property (path = "movement/orient", default = false)]
	orient: bool,
	#[property (path = "movement/speed", default = 100.0)]
	speed: f32,
}

#[methods]
impl SimpleBoid {
	fn new(_owner: &KinematicBody) -> Self {
		Self { 
			separation: 1.0,
			separation_range: 1.0,

			alignment: 1.0, 
			alignment_range: 1.0,

			cohesion: 1.0,
			cohesion_range: 1.0,
			range: 200.0,
			flock_name: String::from("flock"),
			velocity: Vector3::zero(),
			orient: false,
			speed: 0.01,
		}
	}


	#[export]
	unsafe fn _physics_process(&mut self, owner: TRef<KinematicBody>, delta: Variant) {
		let range = self.range * self.range;
		let separation_range = range * self.separation_range * self.separation_range;
		let alignment_range = range * self.alignment_range * self.alignment_range;
		let cohesion_range = range * self.cohesion_range * self.cohesion_range;
		
		let pos = owner.translation();

		let others = owner.get_tree().unwrap().assume_safe().get_nodes_in_group(&self.flock_name).iter()
			.map(|x| x.try_to_object::<Node>().unwrap())
			.filter(|x| x.assume_safe().get_instance_id() != owner.get_instance_id() && {let p = get_pos(*x); p.is_some() && (p.unwrap() - owner.translation()).square_length() < range})
			.collect::<Vec<Ref<Node, Shared>>>();

		self.velocity += 
			separate(pos, &others, separation_range) * self.separation +
			align(pos, &others, alignment_range) * self.alignment +
			coalesce(pos, &others, cohesion_range) * self.cohesion;
			
		self.velocity = self.velocity.normalize() * self.speed;

		if self.orient {
			owner.look_at(owner.translation() + self.velocity, Vector3::new(0.0, 1.0, 0.0));
		}
		
		owner.translate(self.velocity.normalize() * delta.to_f64() as f32);

	}
}

#[derive(NativeClass)]
#[inherit(KinematicBody2D)]
// #[register_with(Self::register)]
pub struct SimpleBoid2D {
	#[property (path = "parameters/separation/base", default = 1.0)]
	separation: f32,
	#[property (path = "parameters/separation/range", default = 1.0)]
	separation_range: f32,

	#[property (path = "parameters/alignment/base", default = 1.0)]
	alignment: f32,
	#[property (path = "parameters/alignment/range", default = 1.0)]
	alignment_range: f32,

	#[property (path = "parameters/cohesion/base", default = 1.0)]
	cohesion: f32,
	#[property (path = "parameters/cohesion/range", default = 1.0)]
	cohesion_range: f32,

	#[property (default = 100.0)]
	range: f32,
	#[property]
	flock_name: String,
	#[property (path = "movement/velocity")]
	velocity: Vector3,
	#[property (path = "movement/orient", default = false)]
	orient: bool,
	#[property (path = "movement/speed", default = 100.0)]
	speed: f32,
}

#[methods]
impl SimpleBoid2D {
	fn new(_owner: &KinematicBody2D) -> Self {
		Self { 
			separation: 1.0,
			separation_range: 1.0,

			alignment: 1.0, 
			alignment_range: 1.0,

			cohesion: 1.0,
			cohesion_range: 1.0,
			range: 200.0,
			flock_name: String::from("flock"),
			velocity: Vector3::zero(),
			orient: false,
			speed: 0.01,
		}
	}


	#[export]
	unsafe fn _physics_process(&mut self, owner: TRef<KinematicBody2D>, delta: Variant) {
		let range = self.range * self.range;
		let separation_range = range * self.separation_range * self.separation_range;
		let alignment_range = range * self.alignment_range * self.alignment_range;
		let cohesion_range = range * self.cohesion_range * self.cohesion_range;

		let pos = vec2tovec3!(owner.position());

		let others = owner.get_tree().unwrap().assume_safe().get_nodes_in_group(&self.flock_name).iter()
			.map(|x| x.try_to_object::<Node>().unwrap())
			.filter(|x| x.assume_safe().get_instance_id() != owner.get_instance_id() && {let p = get_pos(*x); p.is_some() && (p.unwrap() - pos).square_length() < range})
			.collect::<Vec<Ref<Node, Shared>>>();


		self.velocity += 
			separate(pos, &others, separation_range) * self.separation +
			align(pos, &others, alignment_range) * self.alignment +
			coalesce(pos, &others, cohesion_range) * self.cohesion;

			
		// println!("separate {:?}", separate(pos, &others, separation_range) * self.separation);
		// println!("align {:?}", align(pos, &others, alignment_range) * self.alignment);
		// println!("coalesce {:?}", coalesce(pos, &others, cohesion_range) * self.cohesion);
		// println!("pos {:?}", pos);
		
		if self.velocity == Vector3::zero() {
			return;
		}

		if self.velocity.square_length() > self.speed * self.speed {
			self.velocity = self.velocity.normalize() * self.speed;
		}


		if self.orient {
			owner.set_rotation(f32::atan2(self.velocity.y, self.velocity.x).into());
		}
		
		owner.translate(self.velocity.xy() * delta.to_f64() as f32);


	}
}