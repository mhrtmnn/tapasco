HOST = alarm@192.168.2.144
HOST = tapasco@192.168.2.137

CC = aarch64-linux-gnu-gcc  
CC = /home/marco/Downloads/gcc-linaro-7.5.0-2019.12-x86_64_aarch64-linux-gnu/bin/aarch64-linux-gnu-gcc 

INC = /home/marco/Documents/Projects/University/TUD/ESA/tapasco/runtime/build/libtapasco/
LIB = /home/marco/Documents/Projects/University/TUD/ESA/tapasco/runtime/libtapasco/target/aarch64-unknown-linux-gnu/debug/

compile:
	$(CC) arraysum-example.c -o myarraysum -I $(INC) -L $(LIB) -ltapasco -lpthread -ldl -lrt -lm


deploy:
	scp myarraysum $(HOST):~/myArraySum


