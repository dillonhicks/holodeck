SHELL:=/bin/bash
.PHONY: all bootstrap serve clean build


proj.root.dir:=$(PWD)
proj.tools:=tools
proj.tools.dir:=$(proj.root.dir)/$(proj.tools)
proj.tools:=
proj.tools+=nvm
proj.tools+=npm
proj.tools+=cargo
proj.tools+=rustc
proj.tools+=wasm-pack
proj.tools+=wasm-bindgen-cli

proj.nvm.dir:=$(proj.tools.dir)/nvm
proj.nvm.env:=$(proj.nvm.dir)/nvm.sh
proj.node.version:=v14.14.0
proj.node:=$(proj.nvm.dir)/versions/node/$(proj.node.version)

proj.cargo.dir:=$(HOME)/.cargo
proj.cargo:=$(proj.cargo.dir)/bin/cargo
proj.rustc:=$(proj.cargo.dir)/bin/rustc
proj.wasm-pack:=$(proj.cargo.dir)/bin/wasm-pack
proj.wasm-bindgen-cli:=$(proj.cargo.dir)/bin/wasm-bindgen


export NVM_DIR=$(proj.nvm.dir)
export NODE_VERSION=$(proj.node.version)


all: run


run: bootstrap build
	source $(proj.nvm.env) && npm run serve


build: $(proj.tools) node_modules
	cargo build --release --target=wasm32-unknown-unknown
	source $(proj.nvm.env)  && npm run build


node_modules: $(proj.nvm.env) $(proj.node)
	source $(proj.nvm.env) && npm install 

cargo: $(proj.cargo)

npm: node

node:  $(proj.node) 

wasm-pack: $(proj.wasm-pack.bin) 

rustc: $(proj.rustc)

wasm-bindgen-cli: $(proj.wasm-bindgen-cli)


rust: cargo
	rustup target add wasm32-unknown-unknown


$(proj.rustc): cargo


$(proj.wasm-pack.bin): cargo
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh;


$(proj.wasm-bindgen-cli): cargo
	cargo install wasm-bindgen-cli


$(proj.cargo):
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y


$(proj.node): nvm
	source $(proj.nvm.env) && nvm install $(proj.node.version) && nvm use $(proj.node.version)


nvm: $(proj.nvm.env)


$(proj.nvm.env): 
	mkdir -p $(proj.nvm.dir)
	wget -qO- https://raw.githubusercontent.com/nvm-sh/nvm/v0.36.0/install.sh | bash



clean:
	-cargo clean
	-rm -rfv \
		Cargo.lock \
		target/ \
		package-lock.json \
		dist/ \
		node_modules/ \
		pkg/* \
		$(proj.tools.dir)

