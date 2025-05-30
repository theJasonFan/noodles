use std::io::{self, BufRead};

use crate::Line;

pub(super) fn read_line<R>(reader: &mut R, line: &mut Line) -> io::Result<usize>
where
    R: BufRead,
{
    let dst = &mut line.0;

    loop {
        dst.clear();

        let n = super::read_line(reader, dst)?;

        // § 4 "The Canonical Gene" (2020-08-18): "Blank lines should be ignored by parsers..."
        if n == 0 || !is_blank(dst) {
            return Ok(n);
        }
    }
}

pub(crate) fn is_blank(src: &[u8]) -> bool {
    src.iter().all(u8::is_ascii_whitespace)
}

#[cfg(test)]
mod tests {
    use bstr::BString;

    use super::*;

    #[test]
    fn test_read_line() -> io::Result<()> {
        const DATA: &[u8] = b"\n#comment\n\t\n";

        let mut line = Line::default();
        let mut lines: Vec<BString> = Vec::new();

        let mut src = DATA;

        while read_line(&mut src, &mut line)? != 0 {
            lines.push(line.as_ref().into());
        }

        assert_eq!(lines, [BString::from("#comment")]);

        Ok(())
    }
}
