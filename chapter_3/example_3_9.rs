pub fn get_customer_by_id(inlist: &mut Vec<Customer>, customer_id: usize) -> Vec<&mut Customer> {
  return filter(inlist, |customer| { customer.id == customer_id })
}
