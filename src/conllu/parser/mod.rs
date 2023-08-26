use self::line::parse_line;

pub mod line;

pub fn parse(lines: Vec<String>) {
    for line in lines {
        let line = parse_line(line);

        dbg!(line);
    }
}
