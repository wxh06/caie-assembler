use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn;

#[proc_macro_derive(Parser)]
pub fn parser_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_parser(&ast)
}

fn impl_parser(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let mut patterns = quote!();
    match &ast.data {
        syn::Data::Enum(data) => {
            for variant in &data.variants {
                let opcode = &variant.ident;
                let mut fields = quote!();
                for field in &variant.fields {
                    match &field.ty {
                        syn::Type::Path(p) => {
                            let method = proc_macro2::Ident::new(
                                &format!("parse_{}", &p.path.segments.last().unwrap().ident)
                                    .to_lowercase(),
                                proc_macro2::Span::call_site(),
                            );
                            let msg = format!("Too few operands to {}", opcode);
                            fields.extend(quote!(#method(operand.next().expect(#msg),)))
                        }
                        _ => panic!(),
                    }
                }
                patterns.extend(if fields.is_empty() {
                    quote!(stringify!(#opcode) => #name::#opcode,)
                } else {
                    quote!(stringify!(#opcode) => #name::#opcode(#fields),)
                });
            }
        }
        _ => panic!(),
    }
    let gen = quote! {
        impl #name {
            pub fn new<'a>(opcode: &str, operand: &mut impl Iterator<Item = &'a str>) -> #name {
                match opcode {
                    #patterns
                    _ => panic!("Unknown opcode"),
                }
            }
        }
    };
    gen.into()
}
