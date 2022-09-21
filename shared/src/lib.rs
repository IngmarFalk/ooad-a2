extern crate proc_macro;

use proc_macro::{Ident, Span, TokenStream};
use quote::quote;
use syn::{
    parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed,
    Path, Type, TypePath,
};

// #[proc_macro_derive(PFrom, attributes(pfrom))]
// pub fn derive_from(_inp: TokenStream) -> TokenStream {
//     let DeriveInput { ident, data, .. } = parse_macro_input!(_inp as DeriveInput);
//     let fields = match data {
//         Struct(DataStruct {
//             fields: Named(FieldsNamed { ref named, .. }),
//             ..
//         }) => named,
//         _ => panic!("Not supported"),
//     };
//     let data = FromData {
//         name: format!("{}", ident),
//         fields: fields
//             .iter()
//             .filter_map(|field| get_data_field(field))
//             .collect(),
//     };

//     let fields: Vec<String> = data.fields.iter().map(|f| f.name.to_string()).collect();
//     let builder_name = format!("{}Builder", ident);
//     let builder_ident = syn::Ident::new(&builder_name, ident.span());

//     quote! {

//         impl FromMap for #ident {
//             pub fn from_partial_map(data: ::std::collections::HashMap<String, String>) -> Self {

//             }

//             fn from_complete_map(data: ::std::collections::HashMap<String, String>) -> Self {

//             }

//             pub fn copy_with(&self, data: ::std::collections::HashMap<String, String>) -> Self {

//             }
//         }

//         impl ToMap for #ident {
//             fn to_map(&self) -> ::std::collections::HashMap<String, String> {
//                 todo!()
//             }

//             fn to_allowed_mutable_map(&self) -> ::std::collections::HashMap<String, String> {
//                 todo!()
//             }

//             fn to_buffers_map(&self) -> ::std::collections::HashMap<String, String> {
//                 todo!()
//             }
//         }
//     }
//     .into()
// }

#[proc_macro_derive(Builder)]
pub fn derive_builder(inp: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(inp as DeriveInput);
    let fields = match data {
        Struct(DataStruct {
            fields: Named(FieldsNamed { ref named, .. }),
            ..
        }) => named,
        _ => panic!("Not supported"),
    };

    let builders = fields.iter().map(|f| {
        let name = &f.ident;
        let newname = Some(Ident::new("with", Span::call_site()));
        let fname = format!("with_{}", name.clone().unwrap());
        let ty = &f.ty;
        quote! {
            pub fn #name(&mut self, inp: #ty) -> &mut Self {
                self.#name = inp;
                self
                // ::std::string::String::new();
            }
        }
    });

    // let builder_ident = syn::Ident::new(&builder_name, name.span());

    let from_partial_map_args = fields.iter().map(|f| {
        let name = &f.ident;
        let key = format!("{}", name.clone().unwrap());
        quote! {
            #name: if data.contains_key(#key) { data.get(#key).unwrap() } else { Default::default() },
        }
    });

    let from_map_args = fields.iter().map(|f| {
        let name = &f.ident;
        let key = format!("{}", name.clone().unwrap());
        quote! {
            #name: data.get(#key).unwrap()
        }
    });

    let copy_with_args = fields.iter().map(|f| {
        let name = &f.ident;
        let key = format!("{}", name.clone().unwrap());
        quote! {
            match data.get(#key) {
                Some(d) => self.#name = d,
                None => {}
            }
        }
    });

    let from_map = quote! {
    impl FromMap for #ident {
        fn from_partial_map(data: StringMap) -> Self {
            Self {
                #(#from_partial_map_args)*
            }
        }

        fn from_complete_map(data: StringMap) -> Self {
            Self {
                #(#from_map_args,)*
            }
        }

        fn copy_with(&self, data: StringMap) -> Self {
            #(#copy_with_args)*
            self
        }
    }
    };

    let to_map_args = fields.iter().map(|f| {
        let name = &f.ident;
        let key = format!("{}", name.clone().unwrap());
        quote! {
                (#key, self.#name.to_string())
        }
    });

    let to_buffers_map_args = fields.iter().map(|f| {
        let name = &f.ident;
        let key = format!("{}", name.clone().unwrap());
        quote! {
                (#key, ::std::string::String::new())
        }
    });

    let to_map = quote! {
    impl ToMap for #ident {
        fn to_map(&self) -> StringMap {
            ::std::collections::HashMap::from([
                #(#to_map_args,)*
            ])
        }

        fn to_allowed_mutable_map(&self) -> StringMap {
            ::std::collections::HashMap::from([
            ])
        }

        fn to_buffers_map(&self) -> StringMap {
            ::std::collections::HashMap::from([
                #(#to_buffers_map_args,)*
            ])
        }
    }
    };

    let res = quote! {

        impl #ident {
            #(#builders)*
        }

        #from_map

        #to_map
    };

    res.into()
}
