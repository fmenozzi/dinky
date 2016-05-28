all: results
debug: results-debug

build:
	@cargo build --release

build-debug:
	@cargo build

run:
	@cargo run --release

run-debug:
	@cargo run

results: run
	@for file in results/ppm/*; do filename=$$(basename "$$file"); convert $$file results/png/"$${filename%.*}.png"; done

results-debug: run-debug
	@for file in results/ppm/*; do filename=$$(basename "$$file"); convert $$file results/png/"$${filename%.*}.png"; done
