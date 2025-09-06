use syn::{parenthesized, parse::{Parse, ParseStream}, token::Paren};
use crate::{Permission, Permissions};

#[derive(Clone)]
pub enum Expr {
    Permission(Permissions),
    Not(Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Xor(Box<Expr>, Box<Expr>),
    Bracketed(Box<Expr>)
}


impl Parse for Expr {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        if input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            let parsed: Expr = content.parse()?;
            let expr = Box::new(parsed);
            if input.peek(syn::Token![&]) || input.peek(syn::Token![&&]) {
                if input.peek(syn::Token![&&]) {
                    input.parse::<syn::Token![&&]>()?;
                } else {
                    input.parse::<syn::Token![&]>()?;
                }
                let right: Expr = input.parse()?;
                return Ok(Self::And(expr, Box::new(right)));
            } else if input.peek(syn::Token![|]) || input.peek(syn::Token![||]) {
                if input.peek(syn::Token![||]) {
                    input.parse::<syn::Token![||]>()?;
                } else {
                    input.parse::<syn::Token![|]>()?;
                }
                let right: Expr = input.parse()?;
                return Ok(Self::Or(expr, Box::new(right)));
            } else if input.peek(syn::Token![^])  {
                input.parse::<syn::Token![^]>()?;
                let right: Expr = input.parse()?;
                return Ok(Self::Xor(expr, Box::new(right)));
            }
            return Ok(Self::Bracketed(expr));
        }
        if input.peek(syn::Token![!]) {
            input.parse::<syn::Token![!]>()?;
            let expr: Expr = input.parse()?;
            return Ok(Self::Not(Box::new(expr)));
        }
        if input.fork().parse::<Permissions>().is_ok() {
            let parsed: Permissions =  input.parse()?;
            let expr: Box<Self> = Box::new(
                Self::Permission(
                    parsed
                )
            );
            if input.peek(syn::Token![&]) || input.peek(syn::Token![&&]) {
                if input.peek(syn::Token![&&]) {
                    input.parse::<syn::Token![&&]>()?;
                } else {
                    input.parse::<syn::Token![&]>()?;
                }
                let right: Expr = input.parse()?;
                return Ok(Self::And(expr, Box::new(right)));
            } else if input.peek(syn::Token![|]) || input.peek(syn::Token![||]) {
                if input.peek(syn::Token![||]) {
                    input.parse::<syn::Token![||]>()?;
                } else {
                    input.parse::<syn::Token![|]>()?;
                }
                let right: Expr = input.parse()?;
                return Ok(Self::Or(expr, Box::new(right)));
            } else if input.peek(syn::Token![^])  {
                input.parse::<syn::Token![^]>()?;
                let right: Expr = input.parse()?;
                return Ok(Self::Xor(expr, Box::new(right)));
            }
            return Ok(*expr);
        }
        if input.fork().parse::<Permission>().is_ok() {
            input.parse::<Permissions>()?;
        } else {
            return Err(syn::Error::new(input.span(), "Invalid token"))
        }
        unreachable!()
    }
}