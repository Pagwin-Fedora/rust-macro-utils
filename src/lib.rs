extern crate syn;
extern crate quote;
extern crate rand;
extern crate lazy_static;

use rand::{SeedableRng,Rng};
use std::str::FromStr;
use std::sync::Mutex;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Item};
use syn::{Token,parenthesized};

lazy_static::lazy_static!{
    static ref GENERATOR:Mutex<rand::rngs::SmallRng> = Mutex::from(rand::rngs::SmallRng::seed_from_u64(0x13378001));
}

#[proc_macro_attribute]
pub fn assert_func(_:TokenStream, content:TokenStream)->TokenStream{
    if let Item::Fn(fun) = parse_macro_input!(content as Item){
        fun.to_token_stream().into()
    }
    else{
        quote!(
            compile_error!("Must be a function");
        ).into()
    }
}

#[proc_macro_attribute]
pub fn prepend_exec(attr:TokenStream, mut content:TokenStream)->TokenStream{
    //asert_func doesn't care about what's given to attr
    content = assert_func(TokenStream::new(),content);
    let copy = content.clone();
    //redundant check but oh well
    if let Item::Fn(mut fun) = parse_macro_input!(content as Item){
        fun.block.stmts.insert(0,parse_macro_input!(attr as syn::Stmt));
        fun.into_token_stream().into()
    }
    else{
        copy
    }
}

#[proc_macro_attribute]
pub fn postpend_exec(attr:TokenStream, mut content:TokenStream)->TokenStream{
    //asert_func doesn't care about what's given to attr
    content = assert_func(TokenStream::new(),content);
    let copy = content.clone();
    //redundant check but oh well
    if let Item::Fn(mut fun) = parse_macro_input!(content as Item){
        fun.block.stmts.push(parse_macro_input!(attr as syn::Stmt));
        fun.into_token_stream().into()
    }
    else{
        copy
    }
}

trait UsableAttr {
    fn get_name(&self)->TokenStream;
}

impl UsableAttr for syn::Receiver {
    fn get_name(&self)->TokenStream{
        self.self_token.into_token_stream().into()
    }
}
impl UsableAttr for syn::PatType {
    fn get_name(&self)->TokenStream{
        self.pat.to_token_stream().into()
    }
}
impl UsableAttr for syn::FnArg {
    fn get_name(&self)->TokenStream{
        match self {
            syn::FnArg::Receiver(r)=>r.get_name(),
            syn::FnArg::Typed(t)=>t.get_name()
        }
    }
}

struct GimmeParen(syn::token::Paren);
impl syn::parse::Parse for GimmeParen {
    fn parse(input:syn::parse::ParseStream)->syn::parse::Result<Self>{
        let content;
        Ok(parenthesized!(content in input).into())
    }
}
impl From<syn::token::Paren> for GimmeParen {
    fn from(other:syn::token::Paren) -> Self{
        GimmeParen(other)
    }
}
#[proc_macro_attribute]
pub fn decorate(attr:TokenStream, content:TokenStream)->TokenStream{
    let ret = match parse_macro_input!(content as Item){
        Item::Fn(mut fun)=>{
            let og_name = fun.sig.ident.clone();
            let og_sig = fun.sig.clone();
            let err_msg = "A mutex problem has occurred in the decorate macro";
            let new_name:TokenStream = TokenStream::from_str(format!("__{}_{:x}",og_name,{GENERATOR.lock().expect(err_msg).gen::<u16>()}).as_str())
                .expect("Something bad happened when making the new name");
            let new_clone = new_name.clone();
            fun.sig.ident = parse_macro_input!(new_clone as syn::Ident);

            let s_token = "(self)"
                .parse::<TokenStream>().unwrap();
                //.parse::<GimmeParen>().unwrap();
            let s_token = parse_macro_input!(s_token as GimmeParen);
            if let syn::Visibility::Public(_) = fun.vis{
                fun.vis = syn::Visibility::Restricted(syn::VisRestricted{
                    pub_token: <Token![pub]>::default(),
                    paren_token: s_token.0,
                    in_token:None,
                    path:Box::from(syn::Path::from(<Token!(self)>::default()))
                });
            }

            // first format arg is function name which should be the original name
            // second format arg is the decorator function name
            // third format arg is the original function's new random name
            // fourth format arg decides if we need a leading comma before the arguments in the
            // case where a function doesn't take any arguments
            // fifth format arg is "arg1,arg2,arg3,..."
            let args:Vec<TokenStream> = og_sig.clone().inputs.iter()
                .map(UsableAttr::get_name)
                .collect();
            let new_fun = TokenStream::from_str(format!("{} {{ {}({}{}{}) }}",
                og_sig.to_token_stream().to_string(),
                attr,
                new_name.to_string(),
                    if args.len() >= 1 {
                        ","
                    }
                    else {
                        ""
                    },
                args.iter().map(ToString::to_string).collect::<Vec<String>>().join(",")
                ).as_str()).unwrap();
            let mut stream:TokenStream = fun.into_token_stream().into();
            stream.extend(new_fun);
            stream
        },
        _=>{
            quote!(compile_error!("Invalid input")).into_token_stream().into()
        }
    };
    ret
    //format!(r#"compile_error!("{}");"#,ret.to_string()).into_token_stream().into()
}
