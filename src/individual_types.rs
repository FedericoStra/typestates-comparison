//! Every state is implemented as a separate type.

/// The initial state of a new entity.
pub struct Initial {
    id: u32,
}

/// The state of an entity after the addition of `a`.
pub struct WithA {
    id: u32,
    a: f32,
}

/// The state of an entity after the addition of `b`.
pub struct WithB {
    id: u32,
    b: bool,
}

/// The state of an entity after the addition of `c`.
pub struct WithC {
    id: u32,
    a_or_b: AorB,
    c: char,
}

enum AorB {
    A(f32),
    B(bool),
}

impl Initial {
    pub fn new(id: u32) -> Initial {
        Initial { id }
    }

    pub fn add_a(self, a: f32) -> WithA {
        WithA { id: self.id, a }
    }

    pub fn add_b(self, b: bool) -> WithB {
        WithB { id: self.id, b }
    }
}

/// Implemented by entities in states [`WithA`] or [`WithB`].
pub trait WithAorB {
    fn add_c(self, c: char) -> WithC;
}

impl WithAorB for WithA {
    fn add_c(self, c: char) -> WithC {
        WithC {
            id: self.id,
            a_or_b: AorB::A(self.a),
            c,
        }
    }
}

impl WithAorB for WithB {
    fn add_c(self, c: char) -> WithC {
        WithC {
            id: self.id,
            a_or_b: AorB::B(self.b),
            c,
        }
    }
}

impl WithC {
    pub fn format(&self) -> String {
        match self.a_or_b {
            AorB::A(a) => format!("id:{id} a:{a} c:{c}", id = self.id, c = self.c),
            AorB::B(b) => format!("id:{id} b:{b} c:{c}", id = self.id, c = self.c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_a() {
        let e = Initial::new(42);
        let e = e.add_a(3.14);
        let e = e.add_c('a');
        assert_eq!(e.format(), format!("id:42 a:3.14 c:a"));
    }

    #[test]
    fn it_works_with_b() {
        let e = Initial::new(42);
        let e = e.add_b(true);
        let e = e.add_c('b');
        assert_eq!(e.format(), format!("id:42 b:true c:b"));
    }
}
