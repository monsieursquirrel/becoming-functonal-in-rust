pub fn get_customer_by_id(&mut self, customer_id: usize) -> Vec<&mut Customer> {
  return self.filter(|customer| { customer.id == customer_id })
}
