// In order to accept the output of our filter, this needs to accept a list of
// references rather than a list of customers.
pub fn for_each<F>(inlist: Vec<&mut Customer>, func: F)
  where F: Fn(&mut Customer){
  for customer in inlist {
    func(customer);
  }
}
