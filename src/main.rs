// mod tree;
// use crate::tree::{TreeNodeState, Tree};

fn main() {
	// let t = &mut Tree::from_json(String::from(r#"
	// {
	// 	"type": "sequence",
	// 	"children": [
	// 		{
	// 			"type": "BusyCounter",
	// 			"max": 10
	// 		}
	// 		,{
	// 			"type": "selector",
	// 			"children": [
	// 				{
	// 					"type": "if",
	// 					"name": "test"
	// 				}
	// 				,{
	// 					"type": "Set",
	// 					"name": "test",
	// 					"value": true
	// 				}
	// 			]
	// 		}
	// 		,{
	// 			"type": "if",
	// 			"name": "test"
	// 		}
	// 	]		
	// }"#));

	// // let t = &mut Tree::from_json(String::from(r#"{"type": "stub", "result": "SUCCESS"}"#));

	// println!("built tree");

	// loop {
	// 	let x = t.tick(None);
	// 	println!("{:?} {:?}", x, t.blackboard);
	// 	println!("");
	// 	if x == TreeNodeState::SUCCESS {
	// 		break;
	// 	}
	// }
}
