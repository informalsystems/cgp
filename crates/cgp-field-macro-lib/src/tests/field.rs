use quote::quote;

use crate::field::derive_fields;
use crate::tests::helper::equal::equal_token_stream;
use crate::tests::helper::format::format_token_stream;

#[test]
fn test_basic_derive_fields() {
    let derived = derive_fields(quote! {
        pub struct Foo {
            pub bar: Bar,
            pub baz: Baz,
        }
    });

    let expected = quote! {
        pub struct Foo {
            pub bar: Bar,
            pub baz: Baz,
        }

        impl HasField<(Char<'b'>, Char<'a'>, Char<'r'>)> for Foo {
            type Field = Bar;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<(Char<'b'>, Char<'a'>, Char<'r'>)>,
            ) -> &Self::Field {
                &self.bar
            }
        }

        impl HasField<(Char<'b'>, Char<'a'>, Char<'z'>)> for Foo {
            type Field = Baz;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<(Char<'b'>, Char<'a'>, Char<'z'>)>,
            ) -> &Self::Field {
                &self.baz
            }
        }
    };

    let expected = quote! {
        pub struct Foo<FooParamA, FooParamB: Clone>
        where
            FooParamA: Eq,
        {
            pub bar: Bar<FooParamA>,
            pub baz: Baz<String>,
        }

        impl<FooParamA, FooParamB: Clone> HasField<(Char<'b'>, Char<'a'>, Char<'r'>)>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            type Field = Bar<FooParamA>;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<(Char<'b'>, Char<'a'>, Char<'r'>)>,
            ) -> &Self::Field {
                &self.bar
            }
        }

        impl<FooParamA, FooParamB: Clone> HasField<(Char<'b'>, Char<'a'>, Char<'z'>)>
            for Foo<FooParamA, FooParamB>
        where
            FooParamA: Eq,
        {
            type Field = Baz<String>;

            fn get_field(
                &self,
                key: ::core::marker::PhantomData<(Char<'b'>, Char<'a'>, Char<'z'>)>,
            ) -> &Self::Field {
                &self.baz
            }
        }
    };

    assert!(equal_token_stream(&derived, &expected));
}

#[test]
fn test_generic_derive_fields() {
    let derived = derive_fields(quote! {
        pub struct Foo<FooParamA, FooParamB: Clone>
        where
            FooParamA: Eq,
        {
            pub bar: Bar<FooParamA>,
            pub baz: Baz<String>,
        }
    });

    println!("derived:\n{}", format_token_stream(&derived));
}
