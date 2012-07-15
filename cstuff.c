#include <stdint.h>
#include <signal.h>

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
