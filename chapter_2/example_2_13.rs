pub fn get_enabled_customer_addresses(&self) -> Vec<&str> {
  self.get_enabled_customer_field(|customer| { &customer.address[..] })
}
