pub fn get_customer_by_id(&mut self, customer_id: usize) -> Vec<&mut Customer> {
  let mut outlist: Vec<&mut Customer> = vec!();
  for customer in &mut self.all_customers {
    if customer.id == customer_id {
      outlist.push(customer)
    }
  }
  outlist
}
