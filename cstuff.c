#include <stdint.h>
#include <signal.h>
#include <termios.h>
#include <unistd.h>

// sigwrap
static volatile sig_atomic_t poked = 0;

static void
sigwrap_handle(int sig)
{
	poked = 1;
}

void
sigwrap_enable(int sig)
{
	signal(sig, sigwrap_handle);
}

void
sigwrap_disable(int sig)
{
	signal(sig, SIG_DFL);
}

unsigned
sigwrap_get(void)
{
	return poked;
}

void
sigwrap_reset(void)
{
	poked = 0;
}

//termstuff
int
tty_game_mode(int fd, int on)
{
	static const tcflag_t lbits = ICANON | ECHO;
	struct termios tio;

	if (tcgetattr(fd, &tio) != 0) {
		return -1;
	}
	if (on != 0) {
		tio.c_lflag &= ~lbits;
	} else {
		tio.c_lflag |= lbits;
	}
	if (tcsetattr(fd, TCSAFLUSH, &tio) != 0) {
		return -1;
	}

	return 0;
}
