
RUST_VERSION ?= 1.73

.PHONY: builder
builder: Dockerfile
	@docker build -t phalanx-builder -f Dockerfile .

.PHONY: build
build: builder
	@docker run --rm -it \
		-v $(PWD):$(PWD) \
		-w $(PWD) \
		phalanx-builder \
		"cargo build --examples"


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
	mkdir cmake-build
	@docker run --rm -it \
		-v $(PWD):$(PWD) \
		-w $(PWD) \
		phalanx-builder \
		cmake -B cmake-build bpf/
