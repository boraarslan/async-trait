use proc_macro2::Span;
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::{Attribute, ItemImpl, ItemTrait, Token};

pub struct Nothing;

impl Parse for Nothing {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(Nothing)
    }
}

pub enum Item {
    Trait(ItemTrait),
    Impl(ItemImpl),
}

impl Parse for Item {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let first_lookahead = input.lookahead1();
        let lookahead = if first_lookahead.peek(Token![unsafe]) {
            let ahead = input.fork();
            ahead.parse::<Token![unsafe]>()?;
            ahead.lookahead1()
        } else {
            first_lookahead
        };
        if lookahead.peek(Token![pub]) || lookahead.peek(Token![trait]) {
            let mut item: ItemTrait = input.parse()?;
            item.attrs = attrs;
            Ok(Item::Trait(item))
        } else if lookahead.peek(Token![impl]) {
            let mut item: ItemImpl = input.parse()?;
            if item.trait_.is_none() {
                return Err(Error::new(Span::call_site(), "expected a trait impl"));
            }
            item.attrs = attrs;
            Ok(Item::Impl(item))
        } else {
            Err(lookahead.error())
        }
    }
}
