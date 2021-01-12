tool
extends EditorPlugin

func _enter_tree():
	add_custom_type("AIManager", "Node", preload("res://addons/ai_suite/scripts/AIManager.gdns"), preload("res://addons/ai_suite/icons/AIManager.png"))
	add_custom_type("StateMachine", "Node", preload("res://addons/ai_suite/scripts/StateMachine.gdns"), preload("res://addons/ai_suite/icons/StateMachine.png"))
	add_custom_type("State", "Node", preload("res://addons/ai_suite/scripts/State.gdns"), preload("res://addons/ai_suite/icons/State.png"))
	add_custom_type("Not", "Node", preload("res://addons/ai_suite/scripts/Not.gdns"), preload("res://addons/ai_suite/icons/Not.png"))
	add_custom_type("Stub", "Node", preload("res://addons/ai_suite/scripts/Stub.gdns"), preload("res://addons/ai_suite/icons/Stub.png"))
	add_custom_type("Emitter", "Node", preload("res://addons/ai_suite/scripts/Emitter.gdns"), preload("res://addons/ai_suite/icons/Emitter.png"))
	add_custom_type("Selector", "Node", preload("res://addons/ai_suite/scripts/Selector.gdns"), preload("res://addons/ai_suite/icons/Selector.png"))
	add_custom_type("Sequence", "Node", preload("res://addons/ai_suite/scripts/Sequence.gdns"), preload("res://addons/ai_suite/icons/Sequence.png"))
	add_custom_type("Setter", "Node", preload("res://addons/ai_suite/scripts/Setter.gdns"), preload("res://addons/ai_suite/icons/Setter.png"))
	add_custom_type("Caller", "Node", preload("res://addons/ai_suite/scripts/Caller.gdns"), preload("res://addons/ai_suite/icons/Caller.png"))
	add_custom_type("UtilityMaximizer", "Node", preload("res://addons/ai_suite/scripts/UtilityMaximizer.gdns"), preload("res://addons/ai_suite/icons/UtilityMaximizer.png"))
	add_custom_type("UtilityFunction", "Node", preload("res://addons/ai_suite/scripts/UtilityFunction.gdns"), preload("res://addons/ai_suite/icons/UtilityFunction.png"))
	add_custom_type("UtilityAdder", "Node", preload("res://addons/ai_suite/scripts/UtilityAdder.gdns"), preload("res://addons/ai_suite/icons/UtilityAdder.png"))
	add_custom_type("UtilityMultiplier", "Node", preload("res://addons/ai_suite/scripts/UtilityMultiplier.gdns"), preload("res://addons/ai_suite/icons/UtilityMultiplier.png"))
	add_custom_type("UtilityMask", "Node", preload("res://addons/ai_suite/scripts/UtilityMask.gdns"), preload("res://addons/ai_suite/icons/UtilityMask.png"))
	


func _exit_tree():
	remove_custom_type("AIManager")
	remove_custom_type("StateMachine")
	remove_custom_type("State")
	remove_custom_type("Not")
	remove_custom_type("Stub")
	remove_custom_type("Emitter")
	remove_custom_type("Selector")
	remove_custom_type("Sequence")
	remove_custom_type("Setter")
	remove_custom_type("Caller")
	remove_custom_type("UtilityMaximizer")
	remove_custom_type("UtilityFunction")
	remove_custom_type("UtilityAdder")
	remove_custom_type("UtilityMultiplier")
	remove_custom_type("UtilityMask")
	
