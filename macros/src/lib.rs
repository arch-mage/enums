use heck::ToKebabCase;
use heck::ToLowerCamelCase;
use heck::ToShoutyKebabCase;
use heck::ToShoutySnakeCase;
use heck::ToSnakeCase;
use heck::ToTitleCase;
use heck::ToTrainCase;
use heck::ToUpperCamelCase;

#[proc_macro_attribute]
pub fn name(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    name_(args.into(), input.into())
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn name_(
    args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> syn::Result<proc_macro2::TokenStream> {
    let case = syn::parse2::<Options>(args)?.case;

    let tokens = syn::parse2::<syn::ItemEnum>(input)?;

    let name = &tokens.ident;
    let vis = &tokens.vis;

    let variants = tokens.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let variant = match case {
            None => ident.to_string(),
            Some(case) => match case {
                Case::Snake => ident.to_string().to_snake_case(),
                Case::Kebab => ident.to_string().to_kebab_case(),
                Case::Title => ident.to_string().to_title_case(),
                Case::Train => ident.to_string().to_train_case(),
                Case::LowerCamel => ident.to_string().to_lower_camel_case(),
                Case::UpperCamel => ident.to_string().to_upper_camel_case(),
                Case::ShoutySnake => ident.to_string().to_shouty_snake_case(),
                Case::ShoutyKebab => ident.to_string().to_shouty_kebab_case(),
            },
        };
        let lit = syn::LitStr::new(variant.as_str(), ident.span());

        quote::quote! {
            #name::#ident => #lit,
        }
    });

    let variant_from = tokens.variants.iter().map(|variant| {
        let ident = &variant.ident;
        let variant = match case {
            None => ident.to_string(),
            Some(case) => match case {
                Case::Snake => ident.to_string().to_snake_case(),
                Case::Kebab => ident.to_string().to_kebab_case(),
                Case::Title => ident.to_string().to_title_case(),
                Case::Train => ident.to_string().to_train_case(),
                Case::LowerCamel => ident.to_string().to_lower_camel_case(),
                Case::UpperCamel => ident.to_string().to_upper_camel_case(),
                Case::ShoutySnake => ident.to_string().to_shouty_snake_case(),
                Case::ShoutyKebab => ident.to_string().to_shouty_kebab_case(),
            },
        };
        let lit = syn::LitStr::new(variant.as_str(), ident.span());

        quote::quote! {
            #lit => ::std::result::Result::Ok(#name::#ident),
        }
    });

    Ok(quote::quote! {
        #tokens

        impl #name {

            #vis const fn name(&self) -> &'static str {
                match self {
                    #(#variants)*
                }
            }
        }

        impl ::std::str::FromStr for #name {
            type Err = ();

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    #(#variant_from)*
                    _ => Err(())
                }
            }
        }
    })
}

#[derive(Clone, Copy)]
enum Case {
    Snake,
    Kebab,
    Title,
    Train,
    LowerCamel,
    UpperCamel,
    ShoutySnake,
    ShoutyKebab,
}

#[derive(Clone, Copy, Default)]
struct Options {
    case: Option<Case>,
}

impl syn::parse::Parse for Options {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Options::default());
        }

        let name = input.parse::<syn::Ident>()?;
        if name != "case" {
            return Err(syn::Error::new(
                input.span(),
                format!("unknown param: {name}"),
            ));
        }
        input.parse::<syn::Token![=]>()?;
        let lit = input.parse::<syn::LitStr>()?;
        let val = lit.value();
        let case = match val.as_str() {
            "snake" => Case::Snake,
            "kebab" => Case::Kebab,
            "title" => Case::Title,
            "train" => Case::Train,
            "lower-camel" => Case::LowerCamel,
            "upper-camel" => Case::UpperCamel,
            "shouty-kebab" => Case::ShoutyKebab,
            "shouty-snake" => Case::ShoutySnake,
            _ => {
                return Err(syn::Error::new(
                    lit.span(),
                    format!("invalid case: {val:?}"),
                ))
            }
        };
        Ok(Options { case: Some(case) })
    }
}
