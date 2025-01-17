#![crate_name = "foo"]

// @has foo/fn.f.html '//pre[@class="rust item-decl"]' '#[no_mangle]'
#[no_mangle]
pub extern "C" fn f() {}

// @has foo/fn.g.html '//pre[@class="rust item-decl"]' '#[export_name = "bar"]'
#[export_name = "bar"]
pub extern "C" fn g() {}

// @has foo/struct.Repr.html '//pre[@class="rust item-decl"]' '#[repr(C, align(8))]'
#[repr(C, align(8))]
pub struct Repr;

// checking for the presence of the "Default" chip on the variant
// @has foo/enum.Enum2.html '//section[@id="variant.Foo"]' 'Default'
#[derive(Default)]
pub enum Enum2 {
    #[default]
    Foo,
    Bar,
}
