ifeq ("$(wildcard config.mk)", "")
$(error not configured (run configure.sh before make))
endif

include config.mk
