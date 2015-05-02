// The Rust equivalent to null involves using an Option type
pub fn get_customer_by_id(&mut self, customer_id: usize) -> Option<&mut Customer> {
  for customer in &mut self.all_customers {
    if customer.id == customer_id {
      return Some(customer)
    }
  }
  None
}
