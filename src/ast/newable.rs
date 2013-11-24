use super::Node;
use super::SourceLocation;
use super::Position;
use super::Program;

pub trait Newable {
    fn new() -> Self;
}

impl<T: Newable> Newable for Node<T> {
    fn new() -> Node<T> {
        Node {
            loc: Newable::new(),
            body: Newable::new(),
        }
    }
}

impl Newable for SourceLocation {
    fn new() -> SourceLocation {
        SourceLocation {
            start: Newable::new(),
            end: Newable::new(),
        }
    }
}

impl Newable for Position {
    fn new() -> Position {
        Position {
            line: 0,
            column: 0,
        }
    }
}

impl Newable for Program {
    fn new() -> Program {
        Program {
            body: ~[],
        }
    }
}