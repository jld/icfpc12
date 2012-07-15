RANLIB=ranlib
RUSTC=rustc
RUSTCFLAGS=-O

STUFF=state.rs mine.rs geom.rs

randbot: libcstuff.a botshell.rs $(STUFF)
rlrun: libcstuff.a termstuff.rs $(STUFF)
maprun: $(STUFF)
sigtest: libcstuff.a

%: %.rc %.rs
	$(RUSTC) $(RUSTCFLAGS) -L . $<

libcstuff.a: cstuff.o
	$(AR) cru libcstuff.a cstuff.o

cstuff.o: cstuff.c
	$(CC) -O -Wall -Werror -c cstuff.c

