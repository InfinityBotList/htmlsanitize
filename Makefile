RUSTFLAGS_LOCAL="-C target-cpu=native $(RUSTFLAGS) -C link-arg=-fuse-ld=lld"
CARGO_TARGET_GNU_LINKER="x86_64-unknown-linux-gnu-gcc"

# Some sensible defaults, should be overrided per-project
BINS ?= htmlsanitize
PROJ_NAME ?= htmlsanitize
HOST ?= 100.71.175.17

all: 
	@make cross
dev:
	RUSTFLAGS=$(RUSTFLAGS_LOCAL) cargo build
devrun:
	RUSTFLAGS=$(RUSTFLAGS_LOCAL) cargo run
cross:
	CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER=$(CARGO_TARGET_GNU_LINKER) cargo build --target=x86_64-unknown-linux-gnu --release ${ARGS}
push:
	# Kill arcadia
	ssh root@$(HOST) "systemctl stop htmlsanitize"

	@for bin in $(BINS) ; do \
		echo "Pushing $$bin to $(HOST):${PROJ_NAME}/$$bin"; \
		scp -C target/x86_64-unknown-linux-gnu/release/$$bin root@$(HOST):${PROJ_NAME}/$$bin; \
	done

	# Start it up
	ssh root@$(HOST) "systemctl start htmlsanitize"

remote:
	ssh root@$(HOST)
up:
	git submodule foreach git pull
run:
	-mv -vf htmlsanitize.new htmlsanitize # If it exists
	./htmlsanitize
