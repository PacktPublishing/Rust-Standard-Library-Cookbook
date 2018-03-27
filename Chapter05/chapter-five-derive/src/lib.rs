extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;

// HelloWorld is the name for the derive
// hello_world_name is the name of our optional attribute
#[proc_macro_derive(HelloWorld, attributes(hello_world_name))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    // Parse the string representation into an abstract syntax tree
    let ast = syn::parse_derive_input(&s).expect("Failed to parse the source into an AST");

    // Build the implementation
    let gen = impl_hello_world(&ast);

    // Return the generated implementation
    gen.parse()
        .expect("Failed to parse the AST generated from deriving from HelloWorld")
}

fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let identifier = &ast.ident;
    // Use the name provided by the attribute
    // If there is no attribute, use the identifier
    let hello_world_name = get_name_attribute(ast).unwrap_or_else(|| identifier.as_ref());
    quote! {
        // Insert an implementation for our trait
        impl HelloWorld for #identifier {
            fn hello_world() {
                println!(
                    "The struct or enum {} says: \"Hello world from {}!\"",
                    stringify!(#identifier),
                    #hello_world_name
                );
            }
        }
    }
}

fn get_name_attribute(ast: &syn::DeriveInput) -> Option<&str> {
    const ATTR_NAME: &str = "hello_world_name";

    // Go through all attributes and find one with our name
    if let Some(attr) = ast.attrs.iter().find(|a| a.name() == ATTR_NAME) {
        // Check if it's in the form of a name-value pair
        if let syn::MetaItem::NameValue(_, ref value) = attr.value {
            // Check if the value is a string
            if let syn::Lit::Str(ref value_as_str, _) = *value {
                Some(value_as_str)
            } else {
                panic!(
                    "Expected a string as the value of {}, found {:?} instead",
                    ATTR_NAME, value
                );
            }
        } else {
            panic!(
                "Expected an attribute in the form #[{} = \"Some value\"]",
                ATTR_NAME
            );
        }
    } else {
        None
    }
}
