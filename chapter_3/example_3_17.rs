// you can place these in the same impl block as Customer.new() etc or you
// can use a separate impl block, whichever you think looks better.

impl Customer {
  pub fn set_customer_id(&mut self, id: usize) -> &mut Self {
    self.id = id;
    self
  }

  pub fn set_name(&mut self, name: String) -> &mut Self {
    self.name = name;
    self
  }

  pub fn set_state(&mut self, state: String) -> &mut Self {
    self.state = state;
    self
  }

  // ... you get the idea ...
}
