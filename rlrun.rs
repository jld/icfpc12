import io::{reader,reader_util,writer_util};
import state::{state, cont, died, aborted, won, move, wait};
import geom::{left, down, up, right};

fn get_map(-fh: reader) -> state {
    let mut lines = ~[];
    for fh.each_line |line| { lines += ~[copy line]; }
    state::parse(lines)
}

fn show_map(out: io::writer, state: state) {
    out.write_str("\x1b[2J\x1b[H");
    for state.print().each |line| { out.write_line(line); }
}

fn main(argv: ~[str]) {
    termstuff::game_mode(true);
    let in = io::stdin();
    let out = io::stdout();
    let mut state = get_map(io::file_reader(argv[1]).get());
    loop {
        show_map(out, state);
        out.write_line(#fmt("Score: %?", state.score(none)));
        out.flush();
        let mut cmd;
        alt in.read_char() {
          'h' { cmd = move(left) }
          'j' { cmd = move(down) }
          'k' { cmd = move(up) }
          'l' { cmd = move(right) }
          '.' { cmd = wait }
          'q' { break }
          _ { out.write_char('\x07'); again }
        }
        let (res, nstate) = state.step(cmd);
        state = nstate;
        if (res != cont) {
            show_map(out, state);
            out.write_line("\n\x1b[1m"+alt res {
                died { "YOU DIED." } 
                won { "You won!" }
                cont { fail }
            });
            out.write_line(#fmt("Score: %?\x1b[0m", state.score(some(res))));
            out.flush();
            break;
        }
    }
    termstuff::game_mode(false);
}
