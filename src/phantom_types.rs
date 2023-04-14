//! Every state is implemented as a type parameter of a generic type.

/// An entity in a specific state.
pub struct Entity<State> {
    id: u32,
    a_or_b: Option<AorB>,
    c: Option<char>,
    _state: std::marker::PhantomData<State>,
}

/// The initial state of a new entity.
pub enum Initial {}

/// The state of an entity after the addition of `a`.
pub enum WithA {}

/// The state of an entity after the addition of `b`.
pub enum WithB {}

/// The state of an entity after the addition of `c`.
pub enum WithC {}

enum AorB {
    A(f32),
    B(bool),
}

impl Entity<Initial> {
    pub fn new(id: u32) -> Entity<Initial> {
        Entity {
            id,
            a_or_b: None,
            c: None,
            _state: std::marker::PhantomData,
        }
    }

    pub fn add_a(self, a: f32) -> Entity<WithA> {
        Entity {
            id: self.id,
            a_or_b: Some(AorB::A(a)),
            c: self.c,
            _state: std::marker::PhantomData,
        }
    }

    pub fn add_b(self, b: bool) -> Entity<WithB> {
        Entity {
            id: self.id,
            a_or_b: Some(AorB::B(b)),
            c: self.c,
            _state: std::marker::PhantomData,
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
            a_or_b: self.a_or_b,
            c: Some(c),
            _state: std::marker::PhantomData,
        }
    }
}

impl Entity<WithB> {
    pub fn add_c(self, c: char) -> Entity<WithC> {
        Entity {
            id: self.id,
            a_or_b: self.a_or_b,
            c: Some(c),
            _state: std::marker::PhantomData,
        }
    }
}

impl Entity<WithC> {
    pub fn format(&self) -> String {
        match self.a_or_b.as_ref().unwrap() {
            AorB::A(a) => format!("id:{id} a:{a} c:{c}", id = self.id, c = self.c.unwrap()),
            AorB::B(b) => format!("id:{id} b:{b} c:{c}", id = self.id, c = self.c.unwrap()),
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
