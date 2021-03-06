// A trait would be a more direct translation of an interface.
// However it would be very heavy in a situation where no actual struct data is
// required.
pub type ConversionFunction = fn(&Customer) -> &str;
