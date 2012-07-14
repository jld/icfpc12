RANLIB=ranlib
RUSTC=rustc

sigtest: libsigwrap_c.a

%: %.rc
	$(RUSTC) -L . $<

libsigwrap_c.a: sigwrap_c.o
	$(AR) cru libsigwrap_c.a sigwrap_c.o

sigwrap_c.o: sigwrap_c.c
	$(CC) -O -c sigwrap_c.c

