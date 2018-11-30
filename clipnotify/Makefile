x11_bsd_flags = -I/usr/X11R6/include -L/usr/X11R6/lib

all:
	${CC} ${CFLAGS} ${LDFLAGS} clipnotify.c -o clipnotify $(x11_bsd_flags) -lX11 -lXfixes

lib:
	${CC} ${CFLAGS} -c ${LDFLAGS} clipnotifylib.c -o clipnotifylib $(x11_bsd_flags) -lX11 -lXfixes
