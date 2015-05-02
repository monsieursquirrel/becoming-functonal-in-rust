
pub fn map<T, A, B>(inlist: Vec<A>, func: T) -> Vec<B>
  where T: Fn(A) -> B {
  let mut outlist: Vec<B> = vec!();
  for obj in inlist {
    outlist.push(func(obj))
  }
  outlist
}


pub fn for_each<F, A>(inlist: Vec<A>, func: F)
  where F: Fn(A){
  for obj in inlist {
    func(obj);
  }
}

pub fn filter<T, A>(inlist: &mut Vec<A>, test: T) -> Vec<&mut A>
  where T: Fn(&A) -> bool {
  let mut outlist: Vec<&mut A> = vec!();
  for obj in inlist {
    if test(obj) {
      outlist.push(obj)
    }
  }
  outlist
}
