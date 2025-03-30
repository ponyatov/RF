.PHONY: install update
install: rust
	$(MAKE) update
update:
	sudo apt update
	sudo apt install -uy `cat apt.$(shell lsb_release -si)` $(APT)
rust:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
