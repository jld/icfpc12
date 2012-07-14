import io::{file_reader,reader_util,writer_util,stdout};

fn main(argv: ~[str]) {
    let mut lines = ~[];
    let fh = file_reader(argv[1]).get();
    for fh.each_line |line| { lines += ~[copy line]; }
    let state = state::parse(lines);
    let xlines = state::print(state);
    for xlines.each |line| { stdout().write_line(line); }
}
