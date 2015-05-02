// The Rust equivalent to null involves using an Option type.
// This returns a borrow (a reference, more or less) as the customer object
// is still owned by the AllCustomers.all_csutomers list.
pub fn get_customer_by_id(&mut self, customer_id: usize) -> Option<&mut Customer> {
  for customer in &mut self.all_customers {
    if customer.id == customer_id {
      return Some(customer)
    }
  }
  None
}
