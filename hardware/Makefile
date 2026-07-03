TEST_FILES := $(shell find tests -name '*.v')
TESTS := $(TEST_FILES:.v=.test)
VVP := $(TEST_FILES:.v=.vvp)
DEPS_TMP := $(TEST_FILES:.v=.d.tmp)
DEPS := $(TEST_FILES:.v=.d)

.PHONY: all
all: $(TESTS)
	@echo "Tests: ${TEST_FILES}"

.PHONY: $(TESTS)
$(TESTS): %.test: %.vvp
	vvp $<

$(VVP): %.vvp: %.v
	iverilog -M $*.d.tmp -o $@ src/strict.v $<
	@# Reformat the file list into a make rule
	@echo "$@: $$(tr '\n' ' ' < $*.d.tmp)" > $*.d
	@rm $*.d.tmp

.PHONY: clean
clean:
	rm -f $(VVP) $(DEPS) $(DEPS_TMP)

-include $(DEPS)