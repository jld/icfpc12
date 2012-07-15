RANLIB=ranlib
RUSTC=rustc

STUFF=state.rs mine.rs geom.rs

sigtest: libsigwrap_c.a
maprun: maprun.rs $(STUFF)

%: %.rc
	$(RUSTC) -L . $<

libsigwrap_c.a: sigwrap_c.o
	$(AR) cru libsigwrap_c.a sigwrap_c.o

sigwrap_c.o: sigwrap_c.c
	$(CC) -O -c sigwrap_c.c

