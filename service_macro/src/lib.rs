use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as PM2TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, FnArg, ItemFn, Pat, PathArguments, ReturnType, Type};

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

    let deprecated = item.attrs.iter().any(|attr| {
        attr.path
            .get_ident()
            .filter(|ident| ident.to_string() == "deprecated")
            .is_some()
    });

    let vis = item.vis.clone();
    let (is_last_context, context_type) = item
        .sig
        .inputs
        .last_mut()
        .and_then(|arg| {
            let pat_type = if let FnArg::Typed(ref mut pat_type) = arg {
                pat_type
            } else {
                return None;
            };

            let index = if let Some((index, _)) =
                pat_type.attrs.iter().enumerate().find(|(_, attr)| {
                    attr.path
                        .get_ident()
                        .map_or(false, |v| v.to_string() == "context".to_owned())
                }) {
                index
            } else {
                return None;
            };
            pat_type.attrs.remove(index);

            let ty = pat_type.ty.clone();
            let path = if let Type::Path(path) = *ty {
                path
            } else {
                return None;
            };

            let segments = path.path.segments;
            let last_segment = if let Some(last_segment) = segments.last() {
                last_segment
            } else {
                return None;
            };

            if last_segment.ident.to_string() != "Arc" {
                return None;
            }

            let ty = if let PathArguments::AngleBracketed(arg) = &last_segment.arguments {
                arg.args.clone()
            } else {
                return None;
            };

            Some((true, quote! { #ty }))
        })
        .unwrap_or_else(|| (false, quote! { () }));
    let param_count = item.sig.inputs.len();
    let param_type: Vec<_> = item
        .sig
        .inputs
        .iter()
        .filter_map(|fnarg| match fnarg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_type) => Some(pat_type.ty.clone()),
        })
        .take(if is_last_context {
            param_count - 1
        } else {
            param_count
        })
        .collect();
    let param_name: Vec<_> = item
        .sig
        .inputs
        .iter()
        .filter_map(|fnarg| match fnarg {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat) => match *pat.pat.clone() {
                Pat::Ident(ident) => Some(ident.ident),
                _ => None,
            },
        })
        .enumerate()
        .map(|(i, name)| {
            if is_last_context && i + 1 == param_count {
                format_ident!("context")
            } else {
                name
            }
        })
        .collect();
    let destructure_name: Vec<_> = param_name
        .iter()
        .take(if is_last_context {
            param_count - 1
        } else {
            param_count
        })
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
        #vis struct #original_name;

        impl shaped::Service for #original_name {
            type Context = #context_type;
            type Param = (#(#param_type,)*);
            type Response = #response_type;

            fn path(&self) -> shaped::route_path::RoutePath {
                #route_path.parse().expect(&format!("#original_name set invalid path: {}", #route_path))
            }

            fn openapi_detail(&self) -> shaped::openapi::PathItem {
                use shaped::openapi::*;
                use shaped::schemars::{JsonSchema, schema::SchemaObject, schema_for};
                PathItem {
                    reference: None,
                    summary: Option::<String>::None,
                    description: Option::<String>::None,
                    get: Some(Operation {
                        tags: Vec::<String>::new(),
                        summary: Option::<String>::None,
                        description: Option::<String>::None,
                        operation_id: Some(#route_path.to_owned()),
                        parameters: Vec::<RefOr<Parameter>>::new(),
                        request_body: Option::<RefOr<RequestBody>>::None,
                        responses: Responses {
                            responses: vec![
                                (
                                    "200",
                                    Response {
                                        description: "".to_owned(),
                                        content: vec![
                                            (
                                                "application/json".to_owned(),
                                                MediaType {
                                                    schema: Some(SchemaObject::new_ref(<Self::Response as JsonSchema>::schema_name())),
                                                    ..Default::default()
                                                }
                                            )
                                        ].into_iter().collect(),
                                        ..Default::default()
                                    }
                                ),
                                (
                                    "404",
                                    Response {
                                        description: "".to_owned(),
                                        // content: Map<String, MediaType>,
                                        ..Default::default()
                                    }
                                ),
                            ]
                            .into_iter()
                            .map(|(status, response)| (status.to_owned(), RefOr::Object(response.clone())))
                            .collect(),
                            ..Default::default()
                        },
                        deprecated: #deprecated,
                        ..Default::default()
                    }),
                    parameters: vec![
                        #(
                            RefOr::Object(
                                Parameter {
                                    name: stringify!(#destructure_name).to_owned(),
                                    location: "query".to_owned(),
                                    description: Option::<String>::None,
                                    required: true,
                                    deprecated: false,
                                    allow_empty_value: false,
                                    value: ParameterValue::Schema {
                                        style: None,
                                        explode: None,
                                        allow_reserved: false,
                                        schema: schema_for!(#param_type).schema,
                                        example: None,
                                        examples: None,
                                    },
                                    extensions: Default::default(),
                                }
                            )
                        ),*
                    ],
                    ..Default::default()
                }
            }

            fn make_variables(&self, params: &Self::Param) -> std::collections::HashMap<String, String> {
                let mut vars = std::collections::HashMap::new();
                let (#(#destructure_name,)*) = params;
                #(
                    vars.insert(stringify!(#destructure_name).to_owned(), #param_name.into());
                )*
                vars
            }

            fn execute(&self, context: std::sync::Arc<Self::Context>, params: Self::Param) -> Self::Response {
                let (#(#destructure_name,)*) = params;
                #raw_function_name(#(#param_name),*)
            }
        }
    })
    .into()
}
