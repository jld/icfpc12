RANLIB=ranlib
RUSTC=rustc

STUFF=state.rs mine.rs geom.rs

rlrun: libcstuff.a termstuff.rs $(STUFF)
maprun: $(STUFF)
sigtest: libcstuff.a

%: %.rc %.rs
	$(RUSTC) -L . $<

libcstuff.a: cstuff.o
	$(AR) cru libcstuff.a cstuff.o

cstuff.o: cstuff.c
	$(CC) -O -Wall -Werror -c cstuff.c

