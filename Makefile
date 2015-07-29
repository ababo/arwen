ifeq ("$(wildcard .config.mk)", "")
$(error not configured (run configure.sh before make))
endif

include .config.mk

ROOT_DIR := $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))
SRC_DIR := $(ROOT_DIR)/src
SRC_KERNEL_DIR := $(SRC_DIR)/kernel
BUILD_DIR := $(ROOT_DIR)/build
BUILD_SRC_DIR := $(BUILD_DIR)/src
BUILD_KERNEL_DIR := $(BUILD_SRC_DIR)/kernel

BUILD_STAMP := $(BUILD_DIR)/.stamp

RUSTC_FLAGS := --crate-type rlib --target $(TARGET)-unknown-linux-gnu \
	-C opt-level=$(OPT_LEVEL) -C no-stack-check -Z no-landing-pads \
	--cfg arch_$(TARGET) --sysroot /dev/null -L $(BUILD_SRC_DIR)

define SRCS
$(patsubst %, $(SRC_DIR)/$(strip $(1))/%, $(2))
endef

define RLIBS
$(patsubst %, $(BUILD_SRC_DIR)/lib%.rlib, $(1))
endef

define ALIBS
$(patsubst %, $(BUILD_SRC_DIR)/lib%.a, $(1))
endef

define REL_PATHS
$(patsubst $(ROOT_DIR)/%, %, $(1))
endef

define BUILD_LIB
$(call RLIBS, $(1)): $(BUILD_STAMP) \
	$(call RLIBS, $(2)) $(call SRCS, $(1), $(3))
	@echo Compiling $(call REL_PATHS, $(call RLIBS, $(1)))
	@$(RUSTC) $(RUSTC_FLAGS) $(call SRCS, $(1), lib.rs) -o $$@
$(call ALIBS, $(1)): $(call RLIBS, $(1))
	@echo Creating $(call REL_PATHS, $(call ALIBS, $(1)))
	@${OBJCOPY} $$< $$@ 2> /dev/null
endef

.PHONY: all clean run

all: $(BUILD_SRC_DIR)/libkernel.a $(BUILD_SRC_DIR)/libcore.a

$(eval $(call BUILD_LIB, core, , *.rs))
$(eval $(call BUILD_LIB, kernel, core, *.rs $(TARGET)/*.rs))

$(BUILD_STAMP):
	@echo Creating $(call REL_PATHS, $(BUILD_KERNEL_DIR))
	@mkdir -p $(BUILD_KERNEL_DIR)
	@touch $(BUILD_STAMP)

clean:
	@echo Removing $(call REL_PATHS, $(BUILD_DIR))
	@rm -rf $(BUILD_DIR)
