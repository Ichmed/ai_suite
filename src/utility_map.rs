use gdnative::prelude::*;
use gdnative::nativescript::init::property::hint::*;
use std::ops::{Add, AddAssign, Mul, MulAssign};
use crate::base::*;
use crate::utility::*;
use crate::*;

use euclid::{Vector3D, UnknownUnit};

macro_rules! acces{
	($buffer:expr, $x:ident, $y:ident, $z:ident, $size:expr ) => (
		$buffer[($x * $size.y * $size.z + $y * $size.z + $z) as usize]
	)
}


#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct UtilityMaximizer {
	#[property]
	target: String,
	latest_cloud: Option<UtilityCloud>,
	#[property (default = 100)]
	size: i64,
}

#[methods]
impl UtilityMaximizer {
	fn new(_owner: &Node) -> Self {
		UtilityMaximizer {
			target: String::new(),
			latest_cloud: None,
			size: 100
		}
	}

	fn set_target(&mut self, _owner: TRef<Node>, target: String) {
		self.target = target;
	}

	#[export]
	fn get_target(&self, _owner: TRef<Node>) -> String {
		self.target.clone()
	}
	
	fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("target")
			.with_default(String::new())
			.with_getter(UtilityMaximizer::get_target)
			.with_setter(UtilityMaximizer::set_target)
			.done();
	}

	#[export]
	fn add_functions(&mut self, _owner: &Node) {

	}

	#[export]
	unsafe fn run(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		self.tick(owner, manager)
	}

	#[export]
	fn make_image(&self, _owner: TRef<Node>) -> Ref<Image, Shared> {
		
		match self.latest_cloud.as_ref() {
			Some(cloud) => {
				let img = Image::new();
				
				// godot_error!("{:?}", cloud.data.len());

				// godot_print!("{:?}", cloud.as_normalized().data);
				
				img.create_from_data(self.size, self.size, false, 4, TypedArray::from_vec(cloud.as_normalized().data.iter().map(|x| vec![(255.0 - (*x * 255.0)) as u8, (*x * 255.0) as u8, 0u8]).flatten().collect::<Vec<u8>>()));
				img.into_shared()
			}
			None => {
				let img = Image::new();
				img.create(self.size, self.size, false, 0);
				img.into_shared()
			}
		}
	}
}

impl Tick for UtilityMaximizer {

	unsafe fn tick(&mut self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> TreeNodeState {
		

		if owner.get_child_count() == 0 {
			return TreeNodeState::FAILURE;
		}

		for child in owner.get_children().iter() {
			let child = child.try_to_object::<Node>().unwrap().assume_safe();

			let mut cloud = match get_cloud(child, manager) {
				Some(cloud) => cloud,
				None => return TreeNodeState::FAILURE
			};
			
			let manager = manager.assume_safe();

			let pos = get_pos(manager.get_parent().unwrap()).unwrap();

			manager.call("insert", &[Variant::from_str(&self.target), Variant::from_vector3(&(cloud.get_max_position() + pos))]);
			// godot_print!("{:?}", cloud.get_max_position());

			self.latest_cloud = Some(cloud);

		}
		
		TreeNodeState::SUCCESS
	}

}

unsafe fn get_cloud(node: TRef<Node>, manager: Ref<Node, Shared>) -> Option<UtilityCloud> {
	if node.cast_instance::<UtilityFunction>().is_some() {
		return Some(call_on_base_mut!(node, UtilityFunction, UtilityFunction::get_cloud, manager).unwrap())
	} else if node.cast_instance::<UtilityAdder>().is_some() {
		return Some(call_on_base_mut!(node, UtilityAdder, UtilityAdder::get_cloud, manager).unwrap())
	} else if node.cast_instance::<UtilityMultiplier>().is_some() {
		return Some(call_on_base_mut!(node, UtilityMultiplier, UtilityMultiplier::get_cloud, manager).unwrap())
	} else if node.cast_instance::<UtilityMask>().is_some() {
		return Some(call_on_base_mut!(node, UtilityMask, UtilityMask::get_cloud, manager).unwrap())
	} else {
		None
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register)]
pub struct UtilityFunction {
	#[property]
	arguments: VariantArray,
	#[property (default = 100)]
	size: i64,
	#[property (default = true)]
	is_2d: bool,
	#[property (default = 1.0)]
	grain: f32,
	function: String,
	#[property (default = false)]
	invert: bool,
	#[property (default = true)]
	normalize: bool
}

#[methods]
impl UtilityFunction {
	fn new(_owner: &Node) -> Self {
		Self {
			function: String::from("DISTANCE_TO_POINT"),
			size: 100,
			grain: 1.0,
			is_2d: true,
			arguments: VariantArray::default(),
			invert: false,
			normalize: true,
		}
	}

	pub fn get_function(&self, _owner: TRef<Node>) -> String {
		self.function.clone()
	}

	pub fn set_function(&mut self, _owner: TRef<Node>, function: String) {
		self.function = function;
	}

	unsafe fn get_cloud(&self, _owner: TRef<Node>, manager: Ref<Node, Shared>) -> Option<UtilityCloud> {

		let manager = manager.assume_safe();

		let x = blackboard!(self.arguments, manager);

		let params = x.as_slice();

		let mut func: Box<dyn UtilityFunc> = match self.function.as_str() {
			"DISTANCE_TO_POINT" => SquaredDistanceToPoint::new(params[0].to_vector3()),
			"DISTANCE_TO_GROUP" => SquaredDistanceToGroup::new(params[0].to_string()),
			_ => return None
		};

		let mut result = if self.is_2d {
			func.make_2d(manager.get_parent().unwrap(), self.size, self.grain) 
		} else { 
			func.make(manager.get_parent().unwrap(), self.size, self.grain) 
		};

		if self.invert {
			result.invert();
		}

		if self.normalize {
			result.normalize();
		}

		Some(result)

	}

	fn register(builder: &ClassBuilder<Self>) {
		builder.add_property("function")
			.with_default(String::from("DISTANCE_TO_POINT"))
			.with_getter(Self::get_function)
			.with_setter(Self::set_function)
			.with_hint(StringHint::Enum(EnumHint::new(vec![
				"DISTANCE_TO_POINT".into(),
				"DISTANCE_TO_POINTS".into(),
				"DISTANCE_TO_GROUP".into(),
			])))
			.done();
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct UtilityAdder {
	#[property (default = true)]
	normalize: bool
}

#[methods]
impl UtilityAdder {

	fn new(_owner: &Node) -> UtilityAdder {
		UtilityAdder {normalize: false}
	}

	unsafe fn get_cloud(&self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> Option<UtilityCloud> {
		if owner.get_child_count() == 0 { 
			None
		} else {
			let mut cloud = match get_cloud(owner.get_child(0).unwrap().assume_safe(), manager) {
				Some(cloud) => cloud,
				None => return None
			};

			for x in 1..owner.get_child_count() {
				cloud += match get_cloud(owner.get_child(x).unwrap().assume_safe(), manager) {
					Some(cloud) => cloud,
					None => continue
				}
			}
			cloud.calc();
			
			if self.normalize {
				cloud.normalize();
			}
			Some(cloud)
		}
	}
}

#[derive(NativeClass)]
#[inherit(Node)]
pub struct UtilityMultiplier {
	#[property (default = true)]
	normalize: bool
}

#[methods]
impl UtilityMultiplier {

	fn new(_owner: &Node) -> UtilityMultiplier {
		UtilityMultiplier {normalize: false}
	}

	unsafe fn get_cloud(&self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> Option<UtilityCloud> {
		if owner.get_child_count() == 0 { 
			None
		} else {
			let mut cloud = match get_cloud(owner.get_child(0).unwrap().assume_safe(), manager) {
				Some(cloud) => cloud,
				None => return None
			};

			for x in 1..owner.get_child_count() {
				cloud *= match get_cloud(owner.get_child(x).unwrap().assume_safe(), manager) {
					Some(cloud) => cloud,
					None => continue
				}
			}
			cloud.calc();
			
			if self.normalize {
				cloud.normalize();
			}
			Some(cloud)
		}
	}
}


#[derive(NativeClass)]
#[inherit(Node)]
pub struct UtilityMask {
	#[property (default = 0.0)]
	threshold: f32
}

#[methods]
impl UtilityMask {
	fn new(_owner: &Node) -> Self {
		Self { threshold: 0.0}
	}

	unsafe fn get_cloud(&self, owner: TRef<Node>, manager: Ref<Node, Shared>) -> Option<UtilityCloud> {
		if owner.get_child_count() == 0 { 
			None
		} else {
			match get_cloud(owner.get_child(0).unwrap().assume_safe(), manager) {
				Some(cloud) => {
					let mut cloud = cloud;
					cloud.to_mask(self.threshold);
					Some(cloud)
				},
				None => None
			}
		}
	}

}

#[derive(Clone, Debug, PartialEq)]
struct UtilityCloud {
	data: Vec<f32>,
	size: Vector3D<i64, UnknownUnit>,
	grain: f32,
	
	min_position: Option<Vector3D<i64, UnknownUnit>>,
	min: f32,
	
	max_position: Option<Vector3D<i64, UnknownUnit>>,
	max: f32,

}

impl UtilityCloud {
	fn new(size: i64, grain: f32) -> Self {
		// let size = if size % 2 == 0 {
		// 	size + 1
		// }
		// else {
		// 	size
		// };

		let size = Vector3D::new(size, size, size);

		Self {size, data: vec![0.0; (size.x * size.y * size.z) as usize], grain, min_position: None, min: f32::INFINITY, max_position: None, max: -f32::INFINITY}
	}

	fn new_2d(size: i64, grain: f32) -> Self {
		// let size = if size % 2 == 0 {
		// 	size + 1
		// }
		// else {
		// 	size
		// };

		let size = Vector3D::new(size, size, 1);

		Self {size, data: vec![0.0; (size.x * size.y) as usize], grain, min_position: None, min: f32::INFINITY, max_position: None, max: -f32::INFINITY}
	}

	fn calc(&mut self) {

		let mut min = f32::INFINITY;
		let mut max = -f32::INFINITY;

		let mut min_coords = Vector3D::new(0, 0, 0);
		let mut max_coords = Vector3D::new(0, 0, 0);

		for x in 0..self.size.x {
			for y in 0..self.size.y {
				for z in 0..self.size.z {
					let v = acces!(self.data, x, y, z, self.size);
					// print!("{:.2}\t", v);
					if v <= min {
						min = v;
						min_coords = Vector3D::new(x, y, z);
						// println!("New maximum at: {:?} = {}", coords, v);
					}
					if v >= max {
						max = v;
						max_coords = Vector3D::new(x, y, z);
						// println!("New maximum at: {:?} = {}", max_coords, v);
					}
				}
			}
			// println!("");
		}

		self.min = min;
		self.min_position = Some(min_coords);

		self.max = max;
		self.max_position = Some(max_coords);
	}

	fn get_max_position(&mut self) -> Vector3 {
		self.validate();
		
		let max = self.max_position.unwrap() - self.size / 2;
		
		Vector3::new(max.x as f32 * self.grain, max.y as f32 * self.grain, max.z as f32 * self.grain) 
	}

	fn validate(&mut self) -> bool {
		if !self.min_position.is_some() {
			self.calc();
			true
		} else {
			false
		}
	}
	
	fn invert(&mut self) {
		for x in self.data.iter_mut() {
			*x = -*x;
		}
	}

	fn make_positive(&mut self) {
		self.validate();
		if self.min < 0.0 {
			self.floor_to_zero();
		}
	}

	fn floor_to_zero(&mut self) {
		self.validate();
		for x in self.data.iter_mut() {
			*x = *x - self.min;
		}
		self.min = 0.0;
	}

	fn normalize(&mut self) {
		self.validate();

		if self.min == self.max {
			for x in self.data.iter_mut() {
				*x = 1.0;
			}
		} else {
			self.floor_to_zero();
			self.calc();
			for x in self.data.iter_mut() {
				*x = *x / self.max;
			}
			self.calc();
		}

		self.min = 0.0;
		self.max = 1.0;

	}

	fn clamp(&mut self, min: f32, max: f32) {
		self.clamp_min(min);
		self.clamp_max(max);

	}

	fn clamp_min(&mut self, min: f32) {
		for x in self.data.iter_mut() {
			*x = x.max(min);
		}

		self.min = min;
	}

	fn clamp_max(&mut self, max: f32) {
		for x in self.data.iter_mut() {
			*x = x.min(max);
		}
		self.max = max;
	}

	fn to_mask(&mut self, threshold: f32) {
		for x in self.data.iter_mut() {
			*x = if *x <= threshold {
				0.0
			} else {
				1.0
			}
		}
		self.min = 0.0;
		self.max = 1.0;
	}

	fn shift(&mut self, amount: f32) {
		for x in self.data.iter_mut() {
			*x += amount;
		}
		self.min += amount;
		self.max += amount;
	}

	fn as_normalized(&self) -> Self {
		let mut copy = self.clone();
		copy.normalize();
		copy
	}

	fn rescale(&mut self, new_size: Vector3D<i64, UnknownUnit>) {
	}
}


impl Add for UtilityCloud {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		let mut other = other.clone();
		other.rescale(self.size);
		
		let mut d = Vec::new();

		for i in 0..self.size.x * self.size.y * self.size.z {
			d.push(self.data[i as usize] + other.data[i as usize]);
		}
		
		Self {
			data: d,
			size: self.size,
			grain: self.grain, 
			
			min_position: None,
			min: f32::INFINITY, 
			
			max_position: None, 
			max: -f32::INFINITY
		}

	}
}

impl AddAssign for UtilityCloud {
	fn add_assign(&mut self, other: Self) {
		for i in 0..other.data.len() {
			self.data[i] += other.data[i]
		}
	}
}

impl MulAssign for UtilityCloud {
	fn mul_assign(&mut self, other: Self) {
		for i in 0..other.data.len() {
			self.data[i] *= other.data[i]
		}
	}
}

trait UtilityFunc {
	unsafe fn f(&self, position: Vector3) -> f32;
	unsafe fn prepare(&mut self, _owner: TRef<Node>) {}

	unsafe fn make(&mut self, invoker: Ref<Node, Shared>, size: i64, grain: f32) -> UtilityCloud {
		let mut data = UtilityCloud::new(size, grain);
		self.calc(invoker, &mut data);
		data
	}

	unsafe fn make_2d(&mut self, invoker: Ref<Node, Shared>, size: i64, grain: f32) -> UtilityCloud {
		let mut data = UtilityCloud::new_2d(size, grain);
		self.calc(invoker, &mut data);
		data
	}

	unsafe fn calc(&mut self, invoker: Ref<Node, Shared>, cloud: &mut UtilityCloud) {
		
		self.prepare(invoker.assume_safe());

		let center = match get_pos(invoker) {
			Some(pos) => pos,
			None => {
				godot_error!("Invoker has no position");
				return;
			}
		};
		
		for x in 0..cloud.size.x {
			for y in 0..cloud.size.y {
				for z in 0..cloud.size.z {
					let v = Vector3::new(
						(x as f32 - cloud.size.x as f32 / 2.0) * cloud.grain,
						(y as f32 - cloud.size.y as f32 / 2.0) * cloud.grain,
						(z as f32 - cloud.size.z as f32 / 2.0) * cloud.grain);

					// godot_print!("{:?} {:?}", v, v + center);
					acces!(cloud.data, x, y, z, cloud.size) = self.f(v + center);
				}
			}
		}

		cloud.min_position = None;
		cloud.min = f32::INFINITY;
		cloud.max_position = None;
		cloud.max = -f32::INFINITY;
	}
}

struct SquaredDistanceToPoint {
	point: Vector3
}

impl SquaredDistanceToPoint {
	fn new(point: Vector3) -> Box<dyn UtilityFunc> {
		Box::new(SquaredDistanceToPoint { point })
	}
}

impl UtilityFunc for SquaredDistanceToPoint {
	
	
	unsafe fn f(&self, position: Vector3) -> f32 {
		(position - self.point).square_length()
	}
}

struct SquaredDistanceToPoints {
	points: Vec<Vector3>
}

impl SquaredDistanceToPoints {
	fn new(points: Vec<Vector3>) -> Box<dyn UtilityFunc> {
		Box::new(SquaredDistanceToPoints { points })
	}
}

impl UtilityFunc for SquaredDistanceToPoints {
	unsafe fn f(&self, position: Vector3) -> f32 {
		self.points.iter().map(|point| (position - *point).square_length()).fold(f32::MAX, |a, b| a.min(b))
	}
}

struct SquaredDistanceToGroup {
	group: String,
	points: Vec<Vector3>
}

impl SquaredDistanceToGroup {
	fn new(group: String) -> Box<dyn UtilityFunc> {
		Box::new(SquaredDistanceToGroup { group, points: Vec::new()})
	}
}

impl UtilityFunc for SquaredDistanceToGroup {
	
	unsafe fn prepare(&mut self, owner: TRef<Node>) {
		self.points = owner.get_tree()
			.unwrap()
			.assume_safe()
			.get_nodes_in_group(self.group.clone())
			.iter()
			.map(|x| {
				let o = x.try_to_object::<Node>().unwrap();
				match o.assume_safe().cast::<Spatial>() {
					Some(spatial) => spatial.translation(),
					None => match o.assume_safe().cast::<Node2D>() {
						Some(node2d) => {
							let v = node2d.position();
							Vector3::new(v.x, v.y, 0.0)
						},
						None => Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY)
					}
				}
			})
			.collect();

		// godot_print!("{:?}", self.points);
	}
	
	unsafe fn f(&self, position: Vector3) -> f32 {
		self.points.iter().map(|point| (position - *point).square_length()).fold(f32::MAX, |a, b| a.min(b))
	}
}
