ifdef NPERFC
$(info "environment variable NPERFC is set, building with -DNPERFC")
CPPFLAGS+=-DNPERFC
endif

# ifndef LINUX_HOME
# LINUX_HOME:=/lib/modules/$(shell uname -r)/build
# endif

# ifeq ($(ARCH), arm)
# CROSS_COMPILE?=arm-linux-gnueabihf-
# export CROSS_COMPILE
# endif

LINUX_HOME    := ~/Desktop/ToyProjects/PetaLinuxKernel/linux-xlnx
ARCH          := arm64
CROSS_COMPILE := ~/Desktop/ToyProjects/PetaLinuxKernel/aarch64--glibc--stable-2020.08-1/bin/aarch64-buildroot-linux-gnu-
export ARCH
export CROSS_COMPILE

MODULE_DIR := $(PWD)

CPPFLAGS+=-Werror -Wno-missing-attributes -I$(MODULE_DIR) \
				  -I$(MODULE_DIR)/device \
				  -I$(MODULE_DIR)/tlkm \
				  -I$(MODULE_DIR)/user \
				  -I$(MODULE_DIR)/nanopb \
				  -DPB_SYSTEM_HEADER=\<pb_system.h\>

.PHONY:	all clean

all:
	$(MAKE) KCPPFLAGS="$(CPPFLAGS)" -C $(LINUX_HOME) M=$(MODULE_DIR) modules

release:
	$(MAKE) KCPPFLAGS+="$(CPPFLAGS) -DNDEBUG -O3" -C $(LINUX_HOME) M=$(MODULE_DIR) modules

deploy:
	scp tlkm.ko tapasco@192.168.2.137:~/tapasco/runtime/kernel/
	ssh root@192.168.2.137 "bash /home/tapasco/test.sh"

clean:
	$(MAKE) -C $(LINUX_HOME) M=$(MODULE_DIR) clean
