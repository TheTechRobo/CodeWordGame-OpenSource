all:
	@make sanitydonotrundirectly
	cargo --verbose build
	@echo Done - binary in target/whatever

clean:
	cargo --verbose clean

sanitydonotrundirectly:
	@echo Sanity check
	@ls src/main.rs

wasm:
	@make sanitydonotrundirectly
	cargo --verbose build --target wasm32-unknown-unknown
	@echo Done - WASM in target/wasm32-unknown-unknown/debug
