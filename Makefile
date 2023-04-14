.PHONY: all doc test test-lib test-tests test

all: doc test

doc:
	cargo +nightly rustdoc --all-features -- --cfg docsrs

TESTFLAGS = 

test:
	@cargo hack --optional-deps --feature-powerset test -- $(TESTFLAGS)

test-lib:
	@cargo hack --optional-deps --feature-powerset test --lib -- $(TESTFLAGS)

test-tests:
	@cargo hack --optional-deps --feature-powerset test --tests -- $(TESTFLAGS)

test-doc:
	@cargo hack --optional-deps --feature-powerset test --doc -- $(TESTFLAGS)
