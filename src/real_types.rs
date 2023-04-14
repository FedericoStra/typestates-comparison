//! Every state is implemented as a type parameter of a generic type.

/// An entity in a specific state.
pub struct Entity<State> {
    id: u32,
    state: State,
}

/// The initial state of a new entity.
pub struct Initial;

/// The state of an entity after the addition of `a`.
pub struct WithA {
    a: f32,
}

/// The state of an entity after the addition of `b`.
pub struct WithB {
    b: bool,
}

/// The state of an entity after the addition of `c`.
pub struct WithC {
    a_or_b: AorB,
    c: char,
}

enum AorB {
    A(f32),
    B(bool),
}

impl Entity<Initial> {
    pub fn new(id: u32) -> Entity<Initial> {
        Entity { id, state: Initial }
    }

    pub fn add_a(self, a: f32) -> Entity<WithA> {
        Entity {
            id: self.id,
            state: WithA { a },
        }
    }

    pub fn add_b(self, b: bool) -> Entity<WithB> {
        Entity {
            id: self.id,
            state: WithB { b },
        }
    }
}

/// Implemented by entities in states [`WithA`] or [`WithB`].
pub trait EntityWithAorB {
    fn add_c(self, c: char) -> Entity<WithC>;
}

impl EntityWithAorB for Entity<WithA> {
    fn add_c(self, c: char) -> Entity<WithC> {
        self.add_c(c)
    }
}

impl EntityWithAorB for Entity<WithB> {
    fn add_c(self, c: char) -> Entity<WithC> {
        self.add_c(c)
    }
}

impl Entity<WithA> {
    pub fn add_c(self, c: char) -> Entity<WithC> {
        Entity {
            id: self.id,
            state: WithC {
                a_or_b: AorB::A(self.state.a),
                c,
            },
        }
    }
}

impl Entity<WithB> {
    pub fn add_c(self, c: char) -> Entity<WithC> {
        Entity {
            id: self.id,
            state: WithC {
                a_or_b: AorB::B(self.state.b),
                c,
            },
        }
    }
}

impl Entity<WithC> {
    pub fn format(&self) -> String {
        match self.state.a_or_b {
            AorB::A(a) => format!("id:{id} a:{a} c:{c}", id = self.id, c = self.state.c),
            AorB::B(b) => format!("id:{id} b:{b} c:{c}", id = self.id, c = self.state.c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_a() {
        let e = Entity::new(42);
        let e = e.add_a(3.14);
        let e = e.add_c('a');
        assert_eq!(e.format(), format!("id:42 a:3.14 c:a"));
    }

    #[test]
    fn it_works_with_b() {
        let e = Entity::new(42);
        let e = e.add_b(true);
        let e = e.add_c('b');
        assert_eq!(e.format(), format!("id:42 b:true c:b"));
    }
}
