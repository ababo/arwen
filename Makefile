ifeq ("$(wildcard .config.mk)", "")
$(error not configured (run configure.sh before make))
endif

include .config.mk

ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
SRC_DIR := $(ROOT_DIR)/src
BUILD_DIR := $(ROOT_DIR)/build
BUILD_SRC_DIR := $(BUILD_DIR)/src

BUILD_STAMP := $(BUILD_DIR)/.stamp

RUSTC := rustc
RUSTC_FLAGS := --crate-type rlib --target $(TARGET)-unknown-linux-gnu \
		-C opt-level=$(OPT_LEVEL) -C no-stack-check -Z no-landing-pads \
        --cfg arch_$(TARGET) --sysroot /dev/null -L $(BUILD_SRC_DIR)

LIBR_CORE := $(SRC_DIR)/core/lib.rs
SRCS_CORE := $(SRC_DIR)/core/*.rs
RLIB_CORE := $(BUILD_SRC_DIR)/libcore.rlib

LIBR_KERNEL := $(SRC_DIR)/kernel/lib.rs
SRCS_KERNEL := $(SRC_DIR)/kernel/*.rs
RLIB_KERNEL := $(BUILD_SRC_DIR)/libkernel.rlib

.PHONY: all clean run

$(BUILD_SRC_DIR)/kernel.rlib: $(BUILD_STAMP) $(RLIB_CORE) $(SRCS_KERNEL)
	@echo Compiling $(RLIB_KERNEL)
	@$(RUSTC) $(RUSTC_FLAGS) $(LIBR_KERNEL) -o $(RLIB_KERNEL)

$(BUILD_SRC_DIR)/libcore.rlib: $(BUILD_STAMP) $(SRCS_CORE)
	@echo Compiling $(RLIB_CORE)
	@$(RUSTC) $(RUSTC_FLAGS) $(LIBR_CORE) -o $(RLIB_CORE)

$(BUILD_STAMP):
	@echo Creating $(BUILD_SRC_DIR)
	@mkdir -p $(BUILD_SRC_DIR)
	@touch $(BUILD_STAMP)

clean:
	@echo Removing $(BUILD_DIR)
	@rm -rf $(BUILD_DIR)
