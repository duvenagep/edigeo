pub struct NamePrefix([char; 3]);

pub enum Nature {
    T,
    S,
    C,
}

pub enum Format {
    /// [`A`] for a string of characters ;
    A,
    /// [`C`] for coordinates ;
    C,
    /// [`C`] for a date ;
    D,
    /// for a real number with exponent ;
    E,
    ///  for a signed integer ;
    I,
    ///  for an integer with no sign ;
    N,
    /// for the descriptor reference ;
    P,
    ///  for a real number with no exponent,
    R,
    ///  for text,
    T,
    /// [`SPACE`] a space for a reserved logical record.
    SPACE,
}

pub struct Length([usize; 2]);

pub enum Delimiter {
    Colon,
    SemiColon,
}

pub struct Value;
