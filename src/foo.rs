pub use bindgen::foo::Foo;

#[no_mangle]
pub unsafe extern fn foo_get(foo: *const Foo) -> std::os::raw::c_int {
    let foo = &*foo;
    foo.x
}

#[no_mangle]
pub unsafe extern fn foo_set(foo: *mut Foo, value: std::os::raw::c_int) {
    let foo = &mut *foo;
    foo.x = value
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
}
