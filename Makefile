.PHONY: all run docs show-docs diagrams clean

GENERATED_DIR = "./generated"

all: docs diagrams

run:
	@cargo run --example $(EXAMPLE_NAME)

docs:
	@cargo doc --examples

show-docs: docs
	@cargo doc --examples --open

diagrams: clean
	@EXPORT_FOLDER="$(GENERATED_DIR)" cargo build --examples --features export-diagrams
	@for f in "$(GENERATED_DIR)"/*.dot ; do \
		dot -Tpng "$$f" > "$$f".png ; \
	done

clean:
	@cargo clean
	@rm -rf "$(GENERATED_DIR)"

