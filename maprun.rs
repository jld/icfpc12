import io::{reader,reader_util,writer_util};
import state::{state, cont, died, aborted, won};

fn main(argv: ~[str]) {
    let in = io::stdin();
    let out = io::stdout();
    let mut lines = ~[];
    let fh = io::file_reader(argv[1]).get();
    for fh.each_line |line| { lines += ~[copy line]; }
    let mut state = state::parse(lines);
    loop {
        for state.print().each |line| { out.write_line(line); }
        out.write_line("-- ");
        let ch : char;
        let res = if in.eof() {
            aborted
        } else if {ch = in.read_char(); ch == 'A'} {
            aborted
        } else {
            alt state::cmd_opt_of_char(ch) {
              none { again }
              some(cmd) {
                let (res,nstate) = state.step(cmd);
                state = nstate;
                res
              }
            }
        };
        if (res != cont) {
            for state.print().each |line| { out.write_line(line); }
            let msg = alt res {
              cont { fail }
              died { "YOU DIED." }
              aborted { "Successfully aborted." }
              won { "You won!" }
            };
            out.write_line(#fmt("*** %s -- Score: %s", msg,
                                i64::to_str(state.score(res), 10)));
            break;
        }
    }
}
