.PHONY: trigger-plugin
trigger-plugin:
	cargo build --release 
	spin pluginify -i