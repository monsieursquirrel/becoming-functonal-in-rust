// I'm using &str for the parameter as this is most convenient for the caller.
pub fn get_enabled_customer_field(&self, field: &str) -> Vec<String> {
  let mut outlist: Vec<String> = vec!();
  for customer in self.all_customers.iter() {
    if customer.enabled {
      if field == "name" {
        outlist.push(customer.name.clone());
      }
      else if field == "state" {
        outlist.push(customer.name.clone());
      }
      else if field == "primary_contact" {
        outlist.push(customer.name.clone());
      }
      else if field == "domain" {
        outlist.push(customer.name.clone());
      }
      else if field == "address" {
        outlist.push(customer.name.clone());
      }
      else {
        panic!("Unknown field")
      }
    }
  }
  outlist
}
