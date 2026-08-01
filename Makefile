# =============================================================================
#  Metal Slug 1 - Matching Decompilation
#  Top-level Makefile
# =============================================================================
#
#  Usage:
#    make setup   Process rom/201-p1.bin into build/mslug_prom.bin (needs ROM)
#    make match   Compile, link, and byte-compare every registered function
#    make scan    Show the top 30 unmatched callees ordered by popularity
#    make clean   Remove build artefacts
#    make help    Print this help
#
#  The 'match' target is the source of truth for project progress.
# =============================================================================

PYTHON     := python3
ROM_DIR    := rom
BUILD_DIR  := build
TOOLS_DIR  := tools

ROM_INPUT  := $(ROM_DIR)/201-p1.bin
ROM_PROC   := $(BUILD_DIR)/mslug_prom.bin
EXPECT_MD5 := 816b3f74c76b3373993407615f1850fe

.PHONY: all match setup scan clean help verify

all: match

# --- Baserom processing ------------------------------------------------------
$(ROM_PROC): $(ROM_INPUT) scripts/setup.sh
	@./scripts/setup.sh

setup: $(ROM_PROC)
	@echo "Processed P ROM ready at $(ROM_PROC)"

# --- Matcher -----------------------------------------------------------------
match: $(ROM_PROC)
	@$(PYTHON) $(TOOLS_DIR)/match_batch.py

# --- Scanner (priority queue for next targets) -------------------------------
scan: $(ROM_PROC)
	@$(PYTHON) $(TOOLS_DIR)/scan_unmatched_callees.py --top 30

# --- Verify baserom MD5 without processing -----------------------------------
verify:
	@$(PYTHON) -c "import hashlib,sys; \
	  h=hashlib.md5(open('$(ROM_PROC)','rb').read()).hexdigest(); \
	  ok = h=='$(EXPECT_MD5)'; \
	  print(('OK  ' if ok else 'FAIL') + '  ' + h); \
	  sys.exit(0 if ok else 1)"

# --- Cleanup -----------------------------------------------------------------
clean:
	@rm -rf $(BUILD_DIR)/*.bin $(BUILD_DIR)/*.elf $(BUILD_DIR)/*.o \
	        $(BUILD_DIR)/match_report_c.json
	@find $(TOOLS_DIR) -type d -name __pycache__ -exec rm -rf {} +
	@echo "build/ cleaned."

help:
	@sed -n '3,15p' Makefile
