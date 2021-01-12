tool
extends Node

var type = "Caller"

export var method_name = ""
export(NodePath) var path
export(Array) var values = []
export(String) var target = null
