struct Closure {
  foo: &'static str
}

impl Closure {
  fn new() -> Self {
    Closure {
      foo: ""
    }
  }

  fn process(&mut self) -> &Self {
    println!("{:p} = {}", self, self.foo);
    self.foo = "bar";

    (|| {
      println!("{:p} = {}", self, self.foo);
      self.foo = "baz";
    })();

    println!("{:p} = {}", self, self.foo);
    self
  }
}

fn main() {
  // something
  Closure::new().process();
}
