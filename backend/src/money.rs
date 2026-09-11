#[derive(Clone, Copy)]
pub struct Money(i64);

impl Money {
    pub fn as_cents(&self) -> i64 {
        self.0
    }

    pub fn from_cents(cents: i64) -> Self {
        Self(cents)
    }
}
