use gdnative::prelude::*;

#[macro_export]
macro_rules! blackboard{
	($values:expr, $manager:expr) => ({
		$values.iter().map(|x| match x.try_to_string() {
			Some(s) => match s.strip_prefix("#") {
				Some(np) => match np.strip_prefix("!") {
					Some(np) => {
						let mut s = np.split(".");
						let dict_entry = s.next();
						let property = s.next();

						let node: Option<Ref<Node, Shared>> = $manager.call("get", &[dict_entry.to_variant()]).try_to_object();
						match node {
							Some(node) => match node.assume_safe_if_sane() {
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
	})
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

pub unsafe fn get_closest_node(position: Vector3, nodes: Vec<Ref<Node, Shared>>, range: f32) -> Option<Ref<Node, Shared>> {
	
	let range = range * range;
	
	let tmp = nodes.iter()
		.map(|x| (x, get_pos(*x))) //Get all positions (node, position)
		.filter(|x| x.1.is_some()) //Filter out nodes without positions
		.map(|x| (x.0, (x.1.unwrap() - position).square_length())) //(node, distance)
		.filter(|x| range <= 0.0 || x.1 <= range) //cull results that are to far away
		.fold(None, |acc, x| match acc {
			None => Some(x),
			Some(acc) => if acc.1 > x.1 { Some(x) } else { Some(acc) }
		});

	match tmp {
		None => None,
		Some(x) => Some(*x.0)
	}
}

pub unsafe fn get_closest_node_in_group(pos: Vector3, any: TRef<Node>, group: String, range: f32) -> Option<Ref<Node, Shared>> {
	let r = any.get_tree()
			.unwrap()
			.assume_safe()
			.get_nodes_in_group(group.clone());

	get_closest_node(pos, r.iter().map(|x| x.try_to_object::<Node>().unwrap()).collect::<Vec<Ref<Node, Shared>>>(), range)
}