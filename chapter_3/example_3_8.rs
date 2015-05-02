// This function isn't linked to AllCustomers in any way anymore so it's
// better to have it outside the impl AllCustomers block. 
pub fn filter<T>(inlist: &mut Vec<Customer>, test: T) -> Vec<&mut Customer>
  where T: Fn(&Customer) -> bool {
  let mut outlist: Vec<&mut Customer> = vec!();
  for customer in inlist {
    if test(customer) {
      outlist.push(customer)
    }
  }
  outlist
}

impl AllCustomers {
  pub fn get_field<'a, B, F>(&'a mut self,
                             test: fn(&Customer) -> bool,
                             func: F) -> Vec<B>
    where F: Fn(&'a Customer) -> B {

    let mut outlist: Vec<B> = vec!();
    for customer in filter(&mut self.all_customers, test) {
      outlist.push(func(customer));
    }
    outlist
  }

  pub fn get_customer_by_id(&mut self, customer_id: usize) -> Vec<&mut Customer> {
    return filter(&mut self.all_customers, |customer| { customer.id == customer_id })
  }
}
