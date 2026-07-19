
use proc_macro::TokenStream as OrigTokenStream;
use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, punctuated::Punctuated, Ident, Token, Expr, parse, Error, braced, Path, FieldValue, parse2, parse_macro_input, ExprParen, TypePath, Type};
use syn::parse::discouraged::Speculative;
use syn::spanned::Spanned;
use syn::token::{Brace};

trait ErrorWithBacktrace<T> {
    fn with_backtrace(self) -> syn::Result<T>;
}

impl<T> ErrorWithBacktrace<T> for syn::Result<T> {
    fn with_backtrace(self) -> syn::Result<T> {
        match self {
            Ok(val) => Ok(val),
            Err(e) => Err(e) // Err(Error::new(e.span(), format!("Target Type: {}\nbacktrace:\n{}", std::any::type_name::<T>(), Backtrace::capture())))
        }
    }
}

fn forked_parse<T: Parse>(input: &ParseStream) -> Option<T> {
    let ahead = input.fork();
    let result = ahead.parse::<T>().ok();

    if result.is_some() {
        input.advance_to(&ahead);
    }

    result
}

mod kw {
    use syn::custom_keyword;

    custom_keyword!(push);
    custom_keyword!(into);
}

struct Assignment {
    has_into: Option<kw::into>,
    has_clone: Option<Token![&]>,
    has_mut: Option<Token![mut]>,
    ident: Ident,
    _assign: Token![=],
    has_addr: Option<Token![@]>,
    has_unwrap: Option<Token![-]>
}

impl Parse for Assignment {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let assignment = Self {
            has_into: input.parse().with_backtrace()?,
            has_clone: input.parse().with_backtrace()?,
            has_mut: input.parse().with_backtrace()?,
            ident: input.parse().with_backtrace()?,
            _assign: input.parse().with_backtrace()?,
            has_addr: input.parse().with_backtrace()?,
            has_unwrap: input.parse().with_backtrace()?,
        };

        if assignment.has_addr.is_some() && assignment.has_unwrap.is_some() {
            Err(Error::new(input.span(), "invalid syntax: may not combine addr and unwrap syntax."))
        } else {
            Ok(assignment)
        }
    }
}

impl ToTokens for Assignment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { has_mut, ident, .. } = self;
        let expanded = quote! {
            #has_mut
            #ident
        };

        tokens.extend(expanded);
    }
}

enum AssignmentType {
    None,
    Assign(Assignment),
    Let(Token![let], Assignment),
    Push(kw::push, Assignment),
}

impl Parse for AssignmentType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Some(t) = input.parse().ok() {
            Ok(AssignmentType::Let(t, input.parse()?))
        } else if let Some(t) = input.parse().ok() {
            Ok(AssignmentType::Push(t, input.parse()?))
        } else {
            let ahead = input.fork();
            if let Some(assign) = ahead.parse().ok() {
                input.advance_to(&ahead);
                Ok(AssignmentType::Assign(assign))
            } else {
                Ok(AssignmentType::None)
            }
        }
    }
}

impl ToTokens for AssignmentType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let emitted = match self {
            AssignmentType::None => quote! {},
            AssignmentType::Assign(a) => quote! {#a},
            AssignmentType::Let(_, a) => quote! {let #a},
            AssignmentType::Push(_, a) => quote! {#a},
        };
        tokens.extend(emitted);
    }
}

struct StructConstructBody {
    _brace_token: Brace,
    fields: Punctuated<FieldValue, Token![,]>,
    dot2_token: Option<Token![..]>,
    rest: Option<Expr>,
}

impl Parse for StructConstructBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _brace_token: braced!(content in input),
            fields: content.parse_terminated(FieldValue::parse, Token![,]).with_backtrace()?,
            dot2_token: content.parse().with_backtrace()?,
            rest: if content.is_empty() {
                None
            } else {
                Some(content.parse().with_backtrace()?)
            }
        })
    }
}

impl ToTokens for StructConstructBody {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { fields, dot2_token, rest, .. } = self;
        let expanded = quote! { #fields #dot2_token #rest };

        expanded.to_tokens(tokens);
    }
}

struct Instruction {
    assignment: AssignmentType,
    instruction: Ident,
    body: Option<StructConstructBody>
}

impl Parse for Instruction {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            assignment: input.parse().with_backtrace()?,
            instruction: input.parse().with_backtrace()?,
            body: forked_parse(&input)
        })
    }
}

struct EmittingInfo {
    emitter: Expr,
    _semi: Token![;],
    instructions: Punctuated<Instruction, Token![,]>
}

impl Parse for EmittingInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            emitter: input.parse().with_backtrace()?,
            _semi: input.parse().with_backtrace()?,
            instructions: input.parse_terminated(Instruction::parse, Token![,]).with_backtrace()?
        })
    }
}

impl ToTokens for EmittingInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let emitter = &self.emitter;
        let assignments: Vec<&AssignmentType> =  self.instructions.iter().by_ref()
            .map(|i| &i.assignment)
            .collect();

        let line = tokens.span().start();
        let temporaries: Vec<Ident> = (0..assignments.len())
            .map(|i| Ident::new(format!("_L{}_temp_{}", line.line, i).as_str(), tokens.span()))
            .collect();

        let assignment_stmts: Vec<TokenStream> = (0..assignments.len()).map(|i| {
            let temp = &temporaries[i];
            let inst = &self.instructions[i].instruction;
            match assignments[i] {
                AssignmentType::None => quote! { drop(#temp); },
                AssignmentType::Assign(a) |
                AssignmentType::Let(_, a) |
                AssignmentType::Push(_, a) => {
                    let (value, drop) =
                        if a.has_clone.is_some() {
                            if a.has_into.is_some() {
                                (quote! { #temp.clone() }, quote! { drop(#temp); })
                            } else {
                                (quote! { #temp.clone() }, quote! { drop(#temp); })
                            }
                        } else if a.has_addr.is_some() {
                            (quote! { #temp.address() }, quote! { drop(#temp); })
                        } else if a.has_unwrap.is_some() {
                            let unwrap_ex = quote! { <&mut Instruction as TryInto<&mut #inst>>::try_into(#temp.get_mut()).unwrap() };
                            (unwrap_ex, quote! { })
                        } else {
                            (quote! { #temp }, quote! { })
                        };
                    
                    let value = if a.has_into.is_some() {
                        quote! { #value.into() }
                    } else {
                        value
                    };

                    match assignments[i] {
                        AssignmentType::Let(..) => quote! { let #a = #value; #drop },
                        AssignmentType::Push(..) => quote! { #a.push(#value); #drop },
                        _ => quote! { #a = #value; #drop },
                    }
                }
            }
        }).collect();

        let instructions: Vec<TokenStream> = self.instructions.iter().map(|inst| {
            let instruction = &inst.instruction;
            if let Some(body) = &inst.body {
                quote! { Instruction::from(#instruction { #body }) }
            } else {
                quote! { Instruction::from(#instruction::default()) }
            }
        }).collect();

        let count = instructions.len();

        let expanded = quote! {
            let [
                #(#temporaries),*
            ] = #emitter.append_instruction_static_mut([
                #(#instructions),*
            ]);

            #(#assignment_stmts)*;
            #count as usize
        };

        expanded.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn emit(input: OrigTokenStream) -> OrigTokenStream {
    // let input = input.into();
    let info = parse_macro_input!(input as EmittingInfo);
    info.to_token_stream().into()
}
