// The examples at this point in the book seem to be working around Java syntax.
// It is possible to express them in Rust but I don't necessarily recommend
// doing this. The use of template parameters A and B actually obscures the
// fact that B is a return type, which is explicit in the function type syntax.

pub type Function1<A, B> = fn(&A) -> B;
pub type Function2<A1, A2, B> = fn(&A1, &A2) -> B;
pub type Function4<A1, A2, A3, A4, B> = fn(&A1, &A2, &A3, &A4) -> B;
