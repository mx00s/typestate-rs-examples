.PHONY: all run docs show-docs diagrams clean

GENERATED_DIR = "./generated"

all: docs diagrams

run:
	@cargo +nightly run --example $(EXAMPLE_NAME)

docs:
	@cargo +nightly doc --examples

show-docs:
	@cargo +nightly doc --examples --open

diagrams: clean
	@EXPORT_FOLDER="$(GENERATED_DIR)" cargo +nightly build --examples --features export-diagrams
	@for f in "$(GENERATED_DIR)"/*.dot ; do \
		dot -Tpng "$$f" > "$$f".png ; \
	done

clean:
	@cargo +nightly clean
	@rm -rf "$(GENERATED_DIR)"
