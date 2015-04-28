// This function can panic, this is a copy of the book example.
// A better solution would be to return a Result or an Option.
// Or to use an enum for the field parameter.

pub fn get_enabled_customer_field(&self, field: &str) -> Vec<&str> {
  let mut outlist: Vec<&str> = vec!();
  for customer in self.all_customers.iter() {
    if customer.enabled {
      if field == "name" {
        outlist.push(&customer.name);
      }
      else if field == "state" {
        outlist.push(&customer.name);
      }
      else if field == "primary_contact" {
        outlist.push(&customer.name);
      }
      else if field == "domain" {
        outlist.push(&customer.name);
      }
      else if field == "address" {
        outlist.push(&customer.name);
      }
      else {
        panic!("Unknown field")
      }
    }
  }
  outlist
}
