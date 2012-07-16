import io::{reader,reader_util,writer_util};
import state::{state, cont, died, toolong, won};

fn main(argv: ~[str]) {
    let in = io::stdin();
    let out = io::stdout();
    let mut lines = ~[];
    let fh = io::file_reader(argv[1]).get();
    for fh.each_line |line| { lines += ~[copy line]; }
    let mut state = state::parse(lines);
    loop {
        for state.print().each |line| { out.write_line(line); }
        out.write_line(#fmt("Lambdas: %?/%?   Rolling: %?",
                            state.lgot, state.c.lamb, state.rolling));
        out.write_line("-- ");
        let ch : char;
        let res = if in.eof() {
            none
        } else if {ch = in.read_char(); ch == 'A'} {
            none
        } else {
            alt state::cmd_opt_of_char(ch) {
              none { again }
              some(cmd) {
                let (res,nstate) = state.step(cmd);
                state = nstate;
                some(res)
              }
            }
        };
        if (res != some(cont)) {
            for state.print().each |line| { out.write_line(line); }
            let msg = alt res {
              none { "Successfully aborted." }
              some(died) { "YOU DIED." }
              some(toolong) { "You took too long." }
              some(won) { "You won!" }
              some(cont) { fail }
            };
            out.write_line(#fmt("*** %s\nScore: %?", msg,
                                state.score(res)));
            break;
        }
    }
}
