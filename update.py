with open("nodes") as file:
	nodes = [x.strip() for x in file.read().splitlines() if not x.startswith('#') and not x == ""]

print(nodes)

plugin = open("plugin.gd.template").read()

register = ""
remove = ""


gdns = open("Node.gdns.template").read()

for node in nodes:
	register += f'add_custom_type("{node}", "Node", preload("res://addons/ai_suite/scripts/{node}.gdns"), preload("res://addons/ai_suite/icons/{node}.png"))\n\t'
	remove += f'remove_custom_type("{node}")\n\t'
	with open("scripts/" + node + ".gdns", "w") as file:
		file.write(gdns.replace("%NAME%", node))


open("plugin.gd", "w").write(plugin.replace("%REGISTER%", register).replace("%REMOVE%", remove))