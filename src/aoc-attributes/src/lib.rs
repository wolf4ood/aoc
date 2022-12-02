use proc_macro::TokenStream;

use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn, Meta};

fn find_attribute(name: &str, attributes: &AttributeArgs) -> Option<String> {
    attributes
        .iter()
        .filter_map(|attr| match attr {
            syn::NestedMeta::Meta(Meta::NameValue(named)) => {
                if named.path.segments.last().expect("Panic").ident == name {
                    match &named.lit {
                        syn::Lit::Str(string) => Some(string.value()),
                        syn::Lit::Int(int) => Some(int.base10_digits().to_string()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        })
        .next()
}

#[proc_macro_attribute]
pub fn aoc_main(attributes: TokenStream, item: TokenStream) -> TokenStream {
    let function = parse_macro_input!(item as ItemFn);
    let args: AttributeArgs = parse_macro_input!(attributes as AttributeArgs);
    generate_aoc_main(function, args).into()
}

fn generate_aoc_main(function: ItemFn, attributes: AttributeArgs) -> proc_macro2::TokenStream {
    let fn_body = function.block;
    let sig = function.sig;
    let vis = function.vis;
    let fn_name = sig.ident;
    let fn_args = sig.inputs;
    let fn_return_type = sig.output;

    let year = find_attribute("year", &attributes).unwrap_or_default();
    let day = find_attribute("day", &attributes).unwrap_or_default();
    let part1 = find_attribute("part1", &attributes)
        .map(|p1| syn::Ident::new(&p1, proc_macro2::Span::call_site()))
        .map(|ident| {
            quote! {
             let now = start("1");
             let result = #ident();
             stop_and_log(now,"1",&result);
            }
        })
        .unwrap_or_else(|| quote! {});

    let part2 = find_attribute("part2", &attributes)
        .map(|p2| syn::Ident::new(&p2, proc_macro2::Span::call_site()))
        .map(|ident| {
            quote! {
             let now = start("2");
             let result = #ident();
             stop_and_log(now,"2",&result);
            }
        })
        .unwrap_or_else(|| quote! {});

    quote! {
        #vis fn #fn_name(#fn_args) #fn_return_type {

            use std::time::Instant;


            fn start(part :&str) -> Instant {
                let year = #year;
                let day = #day;

                println!("Running AoC {} day {}, part {}",year,day,part);

                Instant::now()
            }


            fn stop_and_log(now : std::time::Instant, part:&str, result : impl std::fmt::Debug) {

                let duration = now.elapsed();

                if duration.as_micros() < 1000 {
                    println!("=> Part {} execution time: {}µs with result : {:?}", part, duration.as_micros(),result);
                } else if duration.as_millis() < 1000  {
                    println!("=> Part {} execution time: {}ms with result : {:?}", part , duration.as_millis(),result);
                }else {
                    println!("=> Part {} execution time: {}s with result : {:?}", part, duration.as_secs(),result);
                }
            }

            #part1

            #part2

            #fn_body

        }

    }
}
