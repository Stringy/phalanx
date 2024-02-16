
RUST_VERSION ?= 1.73

BUILD_TYPE ?= debug

.PHONY: build
build: Dockerfile $(shell find src -type f -name '*.rs')
	@docker build -t phalanx -f Dockerfile --build-arg BUILD_TYPE=$(BUILD_TYPE) .

.PHONY: run-falco
run-falco: build
	@docker run --rm -it \
		-v $(PWD):$(PWD) \
		-w $(PWD) \
		-e RUST_BACKTRACE=full \
		--privileged \
		phalanx-builder \
		"./target/debug/examples/falco-libbpf bpf_probe.o"

.PHONY: debug
debug: builder
	@docker run --rm -it \
		-v $(PWD):$(PWD) \
		-w $(PWD) \
		-v /sys:/sys \
		phalanx-builder

.PHONY: bpf
bpf:
	mkdir -p cmake-build
	@docker run --rm -it \
		-v /usr/src:/usr/src \
		-v $(PWD):$(PWD):rw \
		-w $(PWD) \
		-u 1000:1000 \
		phalanx-builder \
		./scripts/build-bpf.s

.PHONY: clean
clean:
	rm -rf cmake-build
