pub use bindgen::foo::Foo;

impl Foo {
    pub fn new() -> Foo {
        return Foo { x: 0 }
    }

    pub fn get(&self) -> i32 {
        self.x
    }

    pub fn set(&mut self, value: i32) {
        self.x = value;
    }
}

#[no_mangle]
pub unsafe extern fn foo_get(foo: *const Foo) -> std::os::raw::c_int {
    let foo = &*foo;
    foo.get()
}

#[no_mangle]
pub unsafe extern fn foo_set(foo: *mut Foo, value: std::os::raw::c_int) {
    let foo = &mut *foo;
    foo.set(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bindgen::foo::foo_print;

    #[test]
    fn c_api() {
        let mut foo = Foo { x: 0 };
        unsafe {
            foo_set(&mut foo, 5);
            foo_print(&foo);
            assert_eq!(foo_get(&foo), 5);
        }
    }

    #[test]
    fn rust_api() {
        let mut foo = Foo::new();
        foo.set(5);
        assert_eq!(foo.get(), 5);
    }
}
