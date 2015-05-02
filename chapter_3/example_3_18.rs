impl Contract {
  pub fn set_begin_date(&mut self, begin_date: Date<Local>) -> &mut Self {
    self.begin_date = begin_date;
    self
  }

  pub fn set_end_date(&mut self, end_date: Date<Local>) -> &mut Self {
    self.end_date = end_date;
    self
  }

  pub fn set_enabled(&mut self, enabled: bool) -> &mut Self {
    self.enabled = enabled;
    self
  }
}
