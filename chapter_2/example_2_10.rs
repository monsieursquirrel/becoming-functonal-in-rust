// I've used a reference for the output type (B) here since we're grabbing items
// from inside Customer structures. Rust will enforce that the output of this
// function does no live longer than the &self parameter that was used for
// input. This guarantees that all the references will be valid.
// The <B: ?Sized> syntax means that the compiler doesn't need to determine the
// size of B at compile time. This works because references have fixed size so
// they can be placed into a contiguous vector.

pub fn get_enabled_customer_field<B: ?Sized>(&self, func: fn(&Customer) -> &B) -> Vec<&B> {
  let mut outlist: Vec<&B> = vec!();
  for customer in self.all_customers.iter() {
    if customer.enabled {
      outlist.push(func(customer));
    }
  }
  outlist
}
