// There's probably a cleaner way to do this but it doesn't seem to work in beta.

pub fn get_enabled_customer_someone_email(&self, someone: &str) -> Vec<String> {
  self.get_enabled_customer_field(&|customer| {
    let mut email = String::new();
    email.push_str(someone);
    email.push_str("@");
    email.push_str(&customer.domain);
    email
  })
}
