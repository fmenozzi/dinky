build:
	@cargo build

run:
	@cargo run

results: run
	@for file in results/ppm/*; do filename=$$(basename "$$file"); convert $$file results/png/"$${filename%.*}.png"; done
