SHELL := /bin/bash
ARGS ?=

CARGO 					:= cargo
CARGO_CHANNEL 			:= +nightly

RF_TARGET_CPU 			:= -Ctarget-cpu=native
RF_REMAP_PATH_PREFIX 	:= --remap-path-prefix $(HOME)=~
RF_SHARED 				:= $(RF_TARGET_CPU) $(RF_REMAP_PATH_PREFIX)
RF_LOCATION_DETAIL 		:= -Zlocation-detail=none
RF_FMT_DEBUG 			:= -Zfmt-debug=none

OFS := optimize_for_size
PU := panic_unwind
PIA := panic_immediate_abort


.PHONY: dev unwind build check unpretty ctln clean-unpretty all

dev:
	RUSTFLAGS="$(RF_SHARED)" \
		$(CARGO) $(CARGO_CHANNEL) run --profile=dev -Z build-std=core,alloc-- $(ARGS)

unwind:
	RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL)" \
		$(CARGO) $(CARGO_CHANNEL) run --release -Z build-std=std,core,alloc -Z build-std-features=$(OFS)-- $(ARGS)
run:
	RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL) $(RF_FMT_DEBUG)" \
		$(CARGO) $(CARGO_CHANNEL) run --release -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA),$(OFS) -- $(ARGS)

build:
	RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL) $(RF_FMT_DEBUG)" \
		$(CARGO) $(CARGO_CHANNEL) build --release -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA),$(OFS) -- $(ARGS)

bloat: 
	RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL) $(RF_FMT_DEBUG)" \
		$(CARGO) $(CARGO_CHANNEL) build --release -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA),$(OFS) -- $(ARGS)
		$(CARGO) $(CARGO_CHANNEL) bloat --release -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA),$(OFS) --crates $(ARGS)

clean:
	$(CARGO) $(CARGO_CHANNEL) clean

check:
	RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL) $(RF_FMT_DEBUG)" \
		$(CARGO) $(CARGO_CHANNEL) check -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA),$(OFS) --

unpretty:
	rm -rf unpretty
	mkdir -p unpretty
	set -e; \
	for mode in \
	  normal \
	  identified \
	  expanded \
	  'expanded,identified' \
	  'expanded,hygiene' \
	  ast-tree \
	  'ast-tree,expanded' \
	  hir \
	  'hir,identified' \
	  'hir,typed' \
	  hir-tree \
	  thir-tree \
	  'thir-flat' \
	  mir \
	  stable-mir \
	  'mir-cfg'; do \
	    echo "Generating unpretty output for $$mode"; \
		RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL) $(RF_FMT_DEBUG)" ; \
	    	$(CARGO) $(CARGO_CHANNEL) rustc -q --release -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA) -- -Z unpretty=$$mode > unpretty/$$mode.txt; \
	  done

	# MIR
	RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL) $(RF_FMT_DEBUG)" ; \
		$(CARGO) $(CARGO_CHANNEL) rustc -q --release -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA) -- --emit=mir || true
	mv target/release/deps/*.mir unpretty/ 2>/dev/null || true

	# ASM
	RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL) $(RF_FMT_DEBUG)" ; \
		$(CARGO) $(CARGO_CHANNEL) rustc -q --release -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA) -- --emit=asm || true
	mv target/release/deps/*.s unpretty/ 2>/dev/null || true

	# LLVM IR
	RUSTFLAGS="$(RF_SHARED) $(RF_LOCATION_DETAIL) $(RF_FMT_DEBUG)" ; \
		$(CARGO) $(CARGO_CHANNEL) rustc -q --release -Z build-std=std,panic_abort,core,alloc -Z build-std-features=$(PIA) -- --emit=llvm-ir || true
	mv target/release/deps/*.ll unpretty/ 2>/dev/null || true

ctln:
	@test -d unpretty || { echo "unpretty directory not found; run 'make unpretty' first" >&2; exit 1; }
	@echo "["; \
	i=0; \
	for entry in $$(for f in unpretty/*; do [ -f "$$f" ] || continue; nl=$$(wc -l < "$$f"); bn=$$(basename "$$f"); printf "%s:%s\n" "$$nl" "$$bn"; done | sort -t: -k1,1nr); do \
	  nl=$${entry%%:*}; bn=$${entry#*:}; \
	  if [ $$i -gt 0 ]; then echo ","; fi; \
	  printf "  { file: '%s', newlines: %s }" "$$bn" "$$nl"; \
	  i=$$((i+1)); \
	done; \
	echo; echo "]";

clean-unpretty:
	rm -rf unpretty

all: build

