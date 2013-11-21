use util;

static NIL: char = '\xff';

pub struct Reader {
    priv src: ~str,
    priv len: uint,
    priv pos: Option<Position>,
    priv pos_n1: Option<Position>,
    priv pos_n2: Option<Position>,
}

#[deriving(Clone)]
struct Position {
    ch: char,
    idx: uint,
    row: uint,
    col: uint,
}

impl Reader {
    #[inline]
    pub fn new(src: ~str) -> Reader {
        let len = src.len();
        let pos = if len > 0 {
            Some(Position::new(src.char_range_at(0).ch, 0, 0, 0))
        } else {
            None
        };
        let pos_n1 = Reader::next_position(src, pos);
        let pos_n2 = Reader::next_position(src, pos_n1);

        Reader {
            src: src,
            len: len,
            pos: pos,
            pos_n1: pos_n1,
            pos_n2: pos_n2,
        }
    }

    #[inline]
    pub fn nil() -> char { NIL }

    #[inline]
    pub fn curr_pos(&self) -> Position {
        self.pos.clone().unwrap()
    }

    #[inline]
    pub fn curr_pos_idx(&self) -> uint {
        self.pos.map_default(self.len, |pos| pos.idx)
    }

    #[inline]
    pub fn curr(&self) -> char {
        self.pos.map_default(Reader::nil(), |pos| pos.ch)
    }
    #[inline]
    pub fn next(&self) -> char {
        self.pos_n1.map_default(Reader::nil(), |pos| pos.ch)
    }
    #[inline]
    pub fn next_next(&self) -> char {
        self.pos_n2.map_default(Reader::nil(), |pos| pos.ch)
    }

    #[inline]
    pub fn is_eof(&self) -> bool {
        self.pos.is_none()
    }

    #[inline]
    pub fn is_curr(&self, ch: char) -> bool {
        self.pos.map_default(false, |pos| pos.ch == ch)
    }
    #[inline]
    pub fn is_next(&self, ch: char) -> bool {
        self.pos_n1.map_default(false, |pos| pos.ch == ch)
    }
    #[inline]
    pub fn is_next_next(&self, ch: char) -> bool {
        self.pos_n2.map_default(false, |pos| pos.ch == ch)
    }

    #[inline]
    pub fn bump_curr(&mut self) -> char {
        let c = self.curr();
        self.bump();
        c
    }
    #[inline]
    pub fn bump_if(&mut self, ch: char) -> bool {
        if self.curr() == ch {
            self.bump();
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn bump(&mut self) {
        if self.pos.is_some() {
            self.pos = self.pos_n1;
            self.pos_n1 = self.pos_n2;
            self.pos_n2 = Reader::next_position(self.src, self.pos_n1);
        }
    }

    #[inline]
    pub fn with_str_from<R>(&self, start: uint, f: &fn(s: &str) -> R) -> R {
        self.with_str_from_to(start, self.curr_pos_idx(), f)
    }
    #[inline]
    pub fn with_str_from_to<R>(&self, start: uint, end: uint, f: &fn(s: &str) -> R) -> R {
        f(self.src.slice(start, end))
    }

    #[inline]
    fn next_position(src: &str, pos: Option<Position>) -> Option<Position> {
        let len = src.len();
        match pos {
            Some(ref pos) => {
                assert!(pos.idx < len);
                let next_idx = src.char_range_at(pos.idx).next;
                if next_idx < len {
                    let next_ch = src.char_range_at(next_idx).ch;
                    let mut next_pos = Position::new(next_ch, next_idx, pos.row, pos.col);
                    next_pos.set_next(next_idx, util::is_newline(pos.ch));
                    Some(next_pos)
                } else {
                    None
                }
            }
            None => None
        }
    }

    // Eat white spaces and comments.
    pub fn consume_whitespace_and_comments(&mut self) {
        while util::is_whitespace(self.curr()) {
            self.bump();
        }
        self.consume_comment();
    }
    // Eat comments.
    pub fn consume_comment(&mut self) {
        if self.is_curr('/') {
            if self.is_next('/') {
                self.bump();
                self.bump();
                while !self.is_eof() && !self.is_curr('\n') {
                    self.bump();
                }
                self.consume_whitespace_and_comments();
            } else if self.is_next('*') {
                self.bump();
                self.bump();
                while !self.is_eof() {
                    if self.is_curr('*') && self.is_next('/') {
                        self.bump();
                        self.bump();
                        break;
                    } else {
                        self.bump();
                    }
                }
                self.consume_whitespace_and_comments();
            }
        }
    }
}

impl Position {
    #[inline]
    pub fn new(ch: char, idx: uint, row: uint, col: uint) -> Position {
        Position {
            ch: ch,
            idx: idx,
            row: row,
            col: col,
        }
    }
    #[inline]
    pub fn set_next(&mut self, next_idx: uint, is_new_line: bool) {
        self.idx = next_idx;
        if is_new_line {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::Reader;

    #[test]
    fn t1() {
        let src = ~"var a = 1; var b=2; a + b;";
        let mut reader = Reader::new(src);

        assert_eq!(reader.curr(), 'v'); reader.bump();
        assert_eq!(reader.curr(), 'a'); reader.bump();
        assert_eq!(reader.curr(), 'r'); reader.bump();
        assert_eq!(reader.curr(), ' '); reader.bump();
        assert_eq!(reader.curr(), 'a'); reader.bump();
        assert_eq!(reader.curr(), ' '); reader.bump();
        assert_eq!(reader.curr(), '='); reader.bump();
        assert_eq!(reader.curr(), ' '); reader.bump();
        assert_eq!(reader.curr(), '1'); reader.bump();
        assert_eq!(reader.curr(), ';'); reader.bump();
        assert_eq!(reader.curr(), ' '); reader.bump();
        assert_eq!(reader.curr(), 'v'); reader.bump();
        assert_eq!(reader.curr(), 'a'); reader.bump();
        assert_eq!(reader.curr(), 'r'); reader.bump();
        assert_eq!(reader.curr(), ' '); reader.bump();
        assert_eq!(reader.curr(), 'b'); reader.bump();
        assert_eq!(reader.curr(), '='); reader.bump();
        assert_eq!(reader.curr(), '2'); reader.bump();
        assert_eq!(reader.curr(), ';'); reader.bump();
        assert_eq!(reader.curr(), ' '); reader.bump();
        assert_eq!(reader.curr(), 'a'); reader.bump();
        assert_eq!(reader.curr(), ' '); reader.bump();
        assert_eq!(reader.curr(), '+'); reader.bump();
        assert_eq!(reader.curr(), ' '); reader.bump();
        assert_eq!(reader.curr(), 'b'); reader.bump();
        assert_eq!(reader.curr(), ';'); reader.bump();
        assert_eq!(reader.curr(), Reader::nil());
    }
}
