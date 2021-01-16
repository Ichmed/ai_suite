use gdnative::prelude::*;

#[macro_export]
macro_rules! blackboard{
	($values:expr, $manager:expr) => (
		$values.iter().map(|x| match x.try_to_string() {
			Some(s) => match s.strip_prefix("#") {
				Some(np) => match np.strip_prefix("!") {
					Some(np) => {
						let mut s = np.split(".");
						let dict_entry = s.next();
						let property = s.next();

						let node: Option<Ref<Node, Shared>> = $manager.call("get", &[dict_entry.to_variant()]).try_to_object();
						match node {
							Some(node) => match node.assume_safe_if_sane()
							{
								Some(x) => x.call("get", &[property.to_variant()]),
								None => Variant::new()
							},
							None => Variant::new()
						}
					},
					None => $manager.call("get", &[np.to_string().to_variant()]),
				}
				None => x.clone()
			},
			None => x.clone(),
		}).collect::<Vec<Variant>>()
	)
}

#[macro_export]
macro_rules! vec2tovec3{
	($v2:expr) => (
		Vector3::new($v2.x, $v2.y, 0.0)
	)
}

pub unsafe fn get_pos(node: Ref<Node, Shared>) -> Option<Vector3> {
	let node = node.assume_safe();
	match node.cast::<Spatial>() {
		Some(spatial) => Some(spatial.translation()),
		None => match node.cast::<Node2D>() {
			Some(node2d) => {
				let v = node2d.position();
				Some(Vector3::new(v.x, v.y, 0.0))
			},
			None => None
		}
	}
}