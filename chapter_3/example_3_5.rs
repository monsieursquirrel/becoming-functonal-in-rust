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
