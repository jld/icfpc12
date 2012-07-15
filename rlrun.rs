import io::{reader,reader_util,writer_util};
//import state::{state, cont, died, aborted, won};

fn main(argv: ~[str]) {
    termstuff::game_mode(true);
    let in = io::stdin();
    let out = io::stdout();
    loop {
        let ch = in.read_char() as uint;
        if (ch < 32) { break }
        out.write_char((ch ^ 16) as char);
        out.flush();
    }
}
