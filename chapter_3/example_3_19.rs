pub fn map<T, A, B>(inlist: Vec<A>, func: T) -> Vec<B>
  where T: Fn(A) -> B {
  let mut outlist: Vec<B> = vec!();
  for obj in inlist {
    outlist.push(func(obj))
  }
  outlist
}
