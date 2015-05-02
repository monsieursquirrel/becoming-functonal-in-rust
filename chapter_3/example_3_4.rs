// Note: this function uses mutable references as it is called from functions
// which modify the returned items. The function has to take a mutable reference
// parameter in order to use a mutable return type, despite not modifying the
// data internally. This should shake out later in the chapter when side effects
// are removed.

pub fn filter<T>(&mut self, test: T) -> Vec<&mut Customer>
  where T: Fn(&Customer) -> bool {
  let mut outlist: Vec<&mut Customer> = vec!();
  for customer in &mut self.all_customers {
    if test(customer) {
      outlist.push(customer)
    }
  }
  outlist
}
