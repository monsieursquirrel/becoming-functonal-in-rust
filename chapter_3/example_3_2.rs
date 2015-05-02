// The Rust equivalent to null involves using an Option type.
// This returns a borrow (a reference, more or less) as the customer object
// is still owned by the AllCustomers.all_csutomers list.
//
// The return keyword is used here to return from the middle of the function,
// it hasn't been necessary until now as all the functions were returning from
// the end of a code block.
pub fn get_customer_by_id(&mut self, customer_id: usize) -> Option<&mut Customer> {
  for customer in &mut self.all_customers {
    if customer.id == customer_id {
      return Some(customer)
    }
  }
  None
}
