use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as PM2TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, FnArg, ItemFn, ReturnType};

#[derive(Debug, FromMeta)]
struct ServiceItem {
    path: String,
}

#[proc_macro_attribute]
pub fn service(args: TokenStream, item: TokenStream) -> TokenStream {
    let args_pm2: PM2TokenStream = args.into();
    let args_hack: TokenStream = (quote! { path=#args_pm2 }).into();
    let attr_args = parse_macro_input!(args_hack as AttributeArgs);

    let attr_args = match ServiceItem::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };
    let route_path = attr_args.path;

    let mut item = parse_macro_input!(item as ItemFn);
    let param_type: Vec<_> = item
        .sig
        .inputs
        .iter()
        .filter_map(|fnarg| match fnarg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_type) => Some(pat_type.ty.clone()),
        })
        .collect();
    let param_name: Vec<_> = item
        .sig
        .inputs
        .iter()
        .filter(|fnarg| match fnarg {
            FnArg::Receiver(_) => false,
            FnArg::Typed(_) => true,
        })
        .enumerate()
        .map(|(i, _)| format_ident!("param_{}", i))
        .collect();
    let response_type = match item.sig.output.clone() {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };
    let original_name = item.sig.ident.clone();
    let raw_function_name = format_ident!("__service_macro_{}", original_name.clone());

    item.sig.ident = raw_function_name.clone();
    (quote! {
        #item

        #[allow(non_camel_case_types)]
        struct #original_name;

        impl shaped::Service for #original_name {
            type Param = (#(#param_type,)*);
            type Response = #response_type;
            fn path(&self) -> shaped::route_path::RoutePath {
                #route_path.parse().expect(&format!("#original_name set invalid path: {}", #route_path))
            }
            fn execute(&self, params: Self::Param) -> Self::Response {
                let (#(#param_name,)*) = params;
                #raw_function_name(#(#param_name),*)
            }
        }
    })
    .into()
}
