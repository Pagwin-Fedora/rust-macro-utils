# rust-macro-utils

This repository was made to respond to a joking comment that python decorators are better than rust macros by implementing most of python's decorator functionality into a rust macro. Warning this is a buggy mess that will have issues with async functions and will provide annoying(but working) output for functions that use any visibility other than fully public or fully private aka `pub` or `pub(self)`(I know how to fix the visibility issue but I'm done dealing with this forn now). Also adding the example I'm realizing I'm a dumbass because the decorator function should only take the input function as an arg and return an impl Fn with the same type sig but oh well. 

**PLEASE DON'T USE THIS IN PROD EVER.**

## Example
input:
```rs
#[rust_macro_utils::decorate(num_logger)]
fn num_doer(num:u64)->u64{
  println!("Doing {}", num);
  num*2
}
fn num_logger(input_fn:fn(u64)->u64,arg1:u64)->u64 {
  println!("logging that we received input {}", arg1);
  let ret_val = input_fn(arg1);
  println!("finished running the function");
  ret_val
}
```
output source code from macro:
```rs
fn num_doer(num:u64)->u64{
  num_logger(__num_doer_xxxx, num)
}
//note in real code the x's are 4 pseudo random hex digits, these digits will stay the same between compilations
fn __num_doer_xxxx(num:u64)->u64{
  println!("Doing {}", num);
  num*2
}
fn num_logger(input_fn:fn(u64)->u64,arg1:u64)->u64 {
  println!("logging that we received input {}", arg1);
  let ret_val = input_fn(arg1);
  println!("finished running the function");
  ret_val
}
```
