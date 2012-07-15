RANLIB=ranlib
RUSTC=rustc

STUFF=state.rs mine.rs geom.rs

sigtest: libcstuff.a
maprun: maprun.rs $(STUFF)

%: %.rc
	$(RUSTC) -L . $<

libcstuff.a: cstuff.o
	$(AR) cru libcstuff.a cstuff.o

cstuff.o: cstuff.c
	$(CC) -O -c cstuff.c

