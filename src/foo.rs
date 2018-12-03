pub use bindgen::foo::Foo;
pub use bindgen::foo::foo_get;
pub use bindgen::foo::foo_set;

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
