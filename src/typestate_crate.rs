//! Implemented with the help of the [`typestate`](https://crates.io/crates/typestate) crate.

#[typestate::typestate]
pub mod entity {
    /// An entity in a specific state.
    #[automaton]
    pub struct Entity {
        pub(super) id: u32,
    }

    /// The initial state of a new entity.
    #[state]
    pub struct Initial;

    /// The state of an entity after the addition of `a`.
    #[state]
    pub struct WithA {
        pub(super) a: f32,
    }

    /// The state of an entity after the addition of `b`.
    #[state]
    pub struct WithB {
        pub(super) b: bool,
    }

    /// The state of an entity after the addition of `c`.
    #[state]
    pub struct WithC {
        pub(super) a_or_b: super::AorB,
        pub(super) c: char,
    }

    pub trait Initial {
        fn new(id: u32) -> Initial;

        fn add_a(self, a: f32) -> WithA;

        fn add_b(self, b: bool) -> WithB;
    }

    pub trait WithA {
        fn add_c(self, c: char) -> WithC;
    }

    pub trait WithB {
        fn add_c(self, c: char) -> WithC;
    }

    pub trait WithC {
        fn format(&self) -> String;
        fn drop(self);
    }
}

pub use entity::*;

pub use entity::{Entity, Initial, WithA, WithB, WithC};

enum AorB {
    A(f32),
    B(bool),
}

impl InitialState for Entity<Initial> {
    fn new(id: u32) -> Entity<Initial> {
        Entity { id, state: Initial }
    }

    fn add_a(self, a: f32) -> Entity<WithA> {
        Entity {
            id: self.id,
            state: WithA { a },
        }
    }

    fn add_b(self, b: bool) -> Entity<WithB> {
        Entity {
            id: self.id,
            state: WithB { b },
        }
    }
}

impl WithAState for Entity<WithA> {
    fn add_c(self, c: char) -> Entity<WithC> {
        Entity {
            id: self.id,
            state: WithC {
                a_or_b: AorB::A(self.state.a),
                c,
            },
        }
    }
}

impl WithAState for Entity<WithB> {
    fn add_c(self, c: char) -> Entity<WithC> {
        Entity {
            id: self.id,
            state: WithC {
                a_or_b: AorB::B(self.state.b),
                c,
            },
        }
    }
}

impl WithCState for Entity<WithC> {
    fn format(&self) -> String {
        match self.state.a_or_b {
            AorB::A(a) => format!("id:{id} a:{a} c:{c}", id = self.id, c = self.state.c),
            AorB::B(b) => format!("id:{id} b:{b} c:{c}", id = self.id, c = self.state.c),
        }
    }

    fn drop(self) {}
}

#[cfg(test)]
mod tests {
    #![allow(clippy::approx_constant)]

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
