use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(VotodroidResponseObject)]
pub fn votodroid_response_object_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_votodroid_response_object(&ast)
}

fn impl_votodroid_response_object(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let first_field = (|| {
        if let syn::Data::Struct(s) = &ast.data {
            if let syn::Fields::Named(f) = &s.fields {
                return f.named.first().unwrap();
            }
        }
        panic!("VotodroidResponseObject must be a struct with at least one named field.")
    })();
    let first_field_name = first_field.ident.as_ref().unwrap();
    let object_name = (|| {
        if let syn::Type::Path(p) = &first_field.ty {
            if let syn::PathArguments::AngleBracketed(ref a) =
                p.path.segments.first().unwrap().arguments
            {
                if let syn::GenericArgument::Type(t) = a.args.first().unwrap() {
                    return t;
                }
            }
        }
        panic!("VotodroidResponseObject first field must be Option<T>");
    })();
    let from_name = syn::Ident::new(&format!("from_{}", first_field_name), name.span());
    let out = quote! {
        impl #name {
            pub fn #from_name(arg: #object_name) -> #name {
                #name {
                    #first_field_name: Some(arg),
                    errors: None,
                }
            }
            pub fn from_error(field: String, message: String) -> #name {
                #name {
                    #first_field_name: None,
                    errors: Some(vec![FieldError::new(field, message)]),
                }
            }
            pub fn from_errors(errors: Vec<FieldError>) -> #name {
                #name {
                    #first_field_name: None,
                    errors: Some(errors),
                }
            }
        }
    };

    out.into()
}
