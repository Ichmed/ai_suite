tool
extends Node

var type = "Caller"

export var method_name = "f"
export(NodePath) var path = "./Dummy"
export(Array) var values = []
export(String) var target = null


func _ready():
	if Engine.editor_hint:
		var child = Node.new()
		child.name = "Dummy"
		self.add_child(child)
		child.set_owner(get_tree().get_edited_scene_root())
