/*
MIT License

Copyright (c) 2017 David DeSimone

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

#![feature(proc_macro)]
extern crate mock_derive;

use mock_derive::mock;

mod generic_tests;

#[allow(dead_code)]
struct Foo {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
impl Foo {
    pub fn new() -> Foo {
        Foo { x: 0, y: 0 }
    }
}

#[mock]
trait HelloWorld {
    fn hello_world(&self);
    fn foo(&self) -> u32;
    fn bar(&self) -> Option<u32>;
    fn baz(&self, x: i32) -> Foo;
    fn default_method(&self, x: i32, y: i32) -> i32 {
        x + y
    }
}

impl HelloWorld for Foo {
    fn hello_world(&self) {
        println!("Hello World!");
    }

    fn foo(&self) -> u32 {
        1
    }

    fn bar(&self) -> Option<u32> {
        Some(12)
    }

    fn baz(&self, x: i32) -> Foo {
        Foo { x: x, y: x }
    }
}

/* Example of API
   let mut mock = MockHelloWorld::new();
   let method = mock.method_bar()
       .first_call()
       .set_result(Ok(13))
       .second_call()
       .set_result(None);
   mock.set_bar(method);
   mock.bar(); // Returns Ok(13)
   mock.bar(); // Returns None

   // Will fall back to Foo's implementation
   // if method is not mocked
   let foo = Foo::new(...);
   let mut mock = MockHelloWorld::new();
   mock.set_fallback(foo); 

   let method = mock.method_hello_world()
       .when(|| true) 
       .set_result((20));
   mock.set_hello_world(method); 
   mock.hello_world(); // Returns 20
   mock.other_method(); // Calls foo's version of other_method
 
 */

#[test]
fn it_works() {
    let foo = Foo::new();
    let mut mock = MockHelloWorld::new();
    mock.set_fallback(foo);
    let method = mock.method_hello_world()
        .first_call()
        .when(|| true)
        .set_result(());

    mock.set_hello_world(method);
    mock.hello_world();

    let foo_method = mock.method_foo()
        .second_call()
        .set_result(4)
        .first_call()
        .set_result(3);


    mock.set_foo(foo_method);
    let result = mock.foo();
    assert!(result == 3);
    let result2 = mock.foo();
    assert!(result2 == 4);

    // This is a fallback case
    let result3 = mock.foo();
    assert!(result3 == 1);
}

#[test]
fn parameter_type_test() {
    let mut mock = MockHelloWorld::new();
    let method = mock.method_bar()
        .first_call()
        .set_result(Some(11))
        .nth_call(2) // equiv to 'second_call'
        .set_result(None);

    mock.set_bar(method);

    let result = mock.bar();
    assert!(result == Some(11));
    assert!(mock.bar() == None);
}

#[test]
fn parameter_gen_test() {
    let mut mock = MockHelloWorld::new();
    let method = mock.method_baz().first_call().set_result(Foo::new());

    mock.set_baz(method);
    let result = mock.baz(32);
    assert!(result.x == 0 && result.y == 0);
}

#[test]
fn default_impl_test() {
    let mut mock = MockHelloWorld::new();
    let method = mock.method_default_method()
        .first_call()
        .set_result(5);

    mock.set_default_method(method);
    assert!(mock.default_method(1, 1) == 5);
}
