use std::fmt::Debug;

use crate::{
    parse_tree::{
        decl::{
            ClauseKind, DeclLvl1, DeclLvl1Kind, DeclLvl2, DeclLvl2Kind, DeclModifier, EnumDecl,
            EnumDeclKind, FuncBody, FuncBodyKind, FuncParam, FunctionDecl, GenericType,
            GenericsDecl, ImplDecl, ImportDecl, IntEnumBody, IntEnumParam, IntEnumType,
            NamespaceDecl, StructBody, StructDecl, StructDeclKind, StructParam, ThisParam,
            TraitBody, TraitDecl, TypeClause, UnionDecl, VariableDecl,
        },
        statement::{FunctionModifier, VariableModifier, VariableName},
        types::{RefKind, Type},
        IdentPath,
    },
    string::StringSlice,
    tokenizer::{
        token::{Keyword, Symbol, TokenKind},
        Tokenizer,
    },
};

use super::{error::ParserError, expr::parse_expr, statement::parse_block, types::parse_type};

pub fn parse_lvl_1_decl(
    tokenizer: &mut Tokenizer,
) -> Result<Option<DeclModifier<DeclLvl1>>, ParserError> {
    return parse_modifiers(tokenizer, parse_lvl_1_decl_raw, |it| it.slice.clone());
}

pub fn parse_lvl_2_decl(
    tokenizer: &mut Tokenizer,
) -> Result<Option<DeclModifier<DeclLvl2>>, ParserError> {
    return parse_modifiers(tokenizer, parse_lvl_2_decl_raw, |it| it.slice.clone());
}

fn parse_lvl_1_decl_raw(tokenizer: &mut Tokenizer) -> Result<Option<DeclLvl1>, ParserError> {
    if let Some(lvl2) = parse_lvl_2_decl_raw(tokenizer)? {
        return Ok(Some(DeclLvl1 {
            slice: lvl2.slice.clone(),
            kind: DeclLvl1Kind::Lvl2(lvl2),
        }));
    }

    if let Some(st) = parse_struct_decl(tokenizer)? {
        return Ok(Some(DeclLvl1 {
            slice: st.slice.clone(),
            kind: DeclLvl1Kind::Struct(st),
        }));
    }

    if let Some(en) = parse_enum_decl(tokenizer)? {
        return Ok(Some(DeclLvl1 {
            slice: en.slice.clone(),
            kind: DeclLvl1Kind::Enum(en),
        }));
    }

    if let Some(un) = parse_union_decl(tokenizer)? {
        return Ok(Some(DeclLvl1 {
            slice: un.slice.clone(),
            kind: DeclLvl1Kind::Union(un),
        }));
    }

    if let Some(tr) = parse_trait_decl(tokenizer)? {
        return Ok(Some(DeclLvl1 {
            slice: tr.slice.clone(),
            kind: DeclLvl1Kind::Trait(tr),
        }));
    }

    if let Some(im) = parse_impl_decl(tokenizer)? {
        return Ok(Some(DeclLvl1 {
            slice: im.slice.clone(),
            kind: DeclLvl1Kind::Impl(im),
        }));
    }

    return Ok(None);
}

fn parse_lvl_2_decl_raw(tokenizer: &mut Tokenizer) -> Result<Option<DeclLvl2>, ParserError> {
    if let Some(var) = parse_var_decl(tokenizer)? {
        return Ok(Some(DeclLvl2 {
            slice: var.slice.clone(),
            kind: DeclLvl2Kind::Variable(var),
        }));
    }

    if let Some(func) = parse_func_decl(tokenizer)? {
        return Ok(Some(DeclLvl2 {
            slice: func.slice.clone(),
            kind: DeclLvl2Kind::Function(func),
        }));
    }

    return Ok(None);
}

fn parse_modifiers<FGet, FSlice, T>(
    tokenizer: &mut Tokenizer,
    get: FGet,
    slice: FSlice,
) -> Result<Option<DeclModifier<T>>, ParserError>
where
    FGet: FnOnce(&mut Tokenizer) -> Result<Option<T>, ParserError>,
    FSlice: Fn(&T) -> StringSlice,
    T: Debug + Clone + PartialEq,
{
    let peek = tokenizer.peek(0)?;
    let start = peek.slice;

    let generics = parse_generics_decl(tokenizer)?;

    let peek = tokenizer.peek(0)?;
    let is_pub = match peek.kind {
        TokenKind::Keyword(Keyword::Pub) => {
            tokenizer.next()?;
            true
        }
        _ => false,
    };

    let peek = tokenizer.peek(0)?;
    let Some(value) = get(tokenizer)? else {
        if generics.is_some() || is_pub {
            return Err(ParserError::unexpected_token(peek));
        }
        return Ok(None);
    };

    return Ok(Some(DeclModifier {
        slice: start.merge(&slice(&value)),
        generics,
        is_pub,
        value,
    }));
}

pub fn parse_import(tokenizer: &mut Tokenizer) -> Result<Option<ImportDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Import) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice.clone();

    let next = tokenizer.next()?;
    let TokenKind::String(path) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    return Ok(Some(ImportDecl {
        slice: start.merge(&next.slice),
        path,
    }));
}

pub fn parse_using(tokenizer: &mut Tokenizer) -> Result<Option<NamespaceDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Using) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let peek = tokenizer.peek(0)?;
    let Some(path) = IdentPath::try_parse(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek));
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    return Ok(Some(NamespaceDecl {
        slice: start.merge(&next.slice),
        path,
    }));
}

pub fn parse_namespace(tokenizer: &mut Tokenizer) -> Result<Option<NamespaceDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Namespace) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let peek = tokenizer.peek(0)?;
    let Some(path) = IdentPath::try_parse(tokenizer)? else {
        return Err(ParserError::unexpected_token(peek));
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    return Ok(Some(NamespaceDecl {
        slice: start.merge(&next.slice),
        path,
    }));
}

pub fn parse_impl_decl(tokenizer: &mut Tokenizer) -> Result<Option<ImplDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Impl) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let tr = parse_type(tokenizer)?;

    let next = tokenizer.next()?;
    let TokenKind::Keyword(Keyword::For) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let ty = parse_type(tokenizer)?;

    let body = parse_trait_body(tokenizer)?;

    return Ok(Some(ImplDecl {
        slice: start.merge(&body.slice),
        tr,
        ty,
        body,
    }));
}

pub fn parse_trait_decl(tokenizer: &mut Tokenizer) -> Result<Option<TraitDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Trait) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let next = tokenizer.next()?;
    let TokenKind::Identifier(name) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let body = parse_trait_body(tokenizer)?;

    return Ok(Some(TraitDecl {
        slice: start.merge(&body.slice),
        name,
        body,
    }));
}

pub fn parse_union_decl(tokenizer: &mut Tokenizer) -> Result<Option<UnionDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Enum) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let next = tokenizer.next()?;
    let TokenKind::Identifier(name) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let body = parse_struct_body(tokenizer)?;

    return Ok(Some(UnionDecl {
        slice: start.merge(&body.slice),
        name,
        body,
    }));
}

pub fn parse_enum_decl(tokenizer: &mut Tokenizer) -> Result<Option<EnumDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Enum) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let next = tokenizer.next()?;
    let TokenKind::Identifier(name) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let peek = tokenizer.peek(0)?;

    if let TokenKind::Symbol(Symbol::Colon) = peek.kind {
        tokenizer.next()?;
        let next = tokenizer.next()?;
        let Some(ty) = IntEnumType::from(next.kind.clone()) else {
            return Err(ParserError::unexpected_token(next));
        };

        let body = parse_int_enum_body(tokenizer)?;

        return Ok(Some(EnumDecl {
            slice: start.merge(&body.slice),
            name,
            kind: EnumDeclKind::Int { ty, body },
        }));
    }

    let body = parse_struct_body(tokenizer)?;

    return Ok(Some(EnumDecl {
        slice: start.merge(&body.slice),
        name,
        kind: EnumDeclKind::Value(body),
    }));
}

pub fn parse_struct_decl(tokenizer: &mut Tokenizer) -> Result<Option<StructDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Struct) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let next = tokenizer.next()?;
    let TokenKind::Identifier(name) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    if let Some(ty) = parse_type_annotation(tokenizer)? {
        let next = tokenizer.next()?;
        let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };
        return Ok(Some(StructDecl {
            slice: start.merge(&next.slice),
            name,
            kind: StructDeclKind::Wrapper(ty),
        }));
    }

    let body = parse_struct_body(tokenizer)?;

    return Ok(Some(StructDecl {
        slice: start.merge(&body.slice),
        name,
        kind: StructDeclKind::Value(body),
    }));
}

pub fn parse_func_decl(tokenizer: &mut Tokenizer) -> Result<Option<FunctionDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;

    let modifier = match peek.kind {
        TokenKind::Keyword(Keyword::Const) => {
            let peek = tokenizer.peek(1)?;
            let TokenKind::Keyword(Keyword::Func) = peek.kind else {
                return Ok(None);
            };
            tokenizer.next()?;
            FunctionModifier::ConstFunc
        }
        TokenKind::Keyword(Keyword::Func) => FunctionModifier::Func,
        _ => return Ok(None),
    };
    tokenizer.next()?;

    let start = peek.slice;

    let ident = tokenizer.next()?;

    let TokenKind::Identifier(name) = ident.kind else {
        return Err(ParserError::unexpected_token(ident));
    };

    let next = tokenizer.next()?;
    let TokenKind::Symbol(Symbol::ParenOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let this_param = parse_this_param(tokenizer)?;

    if this_param.is_some()
        && let TokenKind::Symbol(Symbol::Comma) = tokenizer.peek(0)?.kind
    {
        tokenizer.next()?;
    }

    let mut params = vec![];

    let peek = tokenizer.peek(0)?;

    match peek.kind {
        TokenKind::Symbol(Symbol::ParenClose) => {
            tokenizer.next()?;
        }
        _ => loop {
            let peek = tokenizer.peek(0)?;

            let start = peek.slice;

            let is_mut = if let TokenKind::Keyword(Keyword::Mut) = peek.kind {
                tokenizer.next()?;
                true
            } else {
                false
            };

            let next = tokenizer.next()?;

            let TokenKind::Identifier(name) = next.kind else {
                return Err(ParserError::unexpected_token(next));
            };

            let next = tokenizer.next()?;

            let TokenKind::Symbol(Symbol::Colon) = next.kind else {
                return Err(ParserError::unexpected_token(next));
            };

            let ty = parse_type(tokenizer)?;

            params.push(FuncParam {
                slice: start.merge(&ty.slice),
                is_mut,
                name,
                ty,
            });

            let next = tokenizer.next()?;

            match next.kind {
                TokenKind::Symbol(Symbol::Comma) => {
                    if let TokenKind::Symbol(Symbol::ParenClose) = tokenizer.peek(0)?.kind {
                        tokenizer.next()?;
                        break;
                    }
                }
                TokenKind::Symbol(Symbol::ParenClose) => break,
                _ => return Err(ParserError::unexpected_token(next)),
            }
        },
    }

    let ret = parse_type_annotation(tokenizer)?;

    let peek = tokenizer.peek(0)?;

    let (body, end) = match peek.kind {
        TokenKind::Symbol(Symbol::WideArrow | Symbol::BraceOpen) => {
            let body = parse_func_body(tokenizer, Some(Symbol::Semicolon))?;
            let slice = body.slice.clone();
            (Some(body), slice)
        }
        TokenKind::Symbol(Symbol::Semicolon) => {
            tokenizer.next()?;
            (None, peek.slice)
        }
        _ => return Err(ParserError::unexpected_token(peek)),
    };

    return Ok(Some(FunctionDecl {
        slice: start.merge(&end),
        modifier,
        name,
        this_param,
        params,
        ret,
        body,
    }));
}

pub fn parse_var_decl(tokenizer: &mut Tokenizer) -> Result<Option<VariableDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;

    let modifier = match peek.kind {
        TokenKind::Keyword(Keyword::Let) => {
            if let TokenKind::Keyword(Keyword::Match) = tokenizer.peek(1)?.kind {
                return Ok(None);
            }
            VariableModifier::Let
        }
        TokenKind::Keyword(Keyword::Const) => {
            if let TokenKind::Keyword(Keyword::Func) = tokenizer.peek(1)?.kind {
                return Ok(None);
            }
            VariableModifier::Const
        }
        TokenKind::Keyword(Keyword::Mut) => VariableModifier::Mut,
        TokenKind::Keyword(Keyword::Static) => VariableModifier::Static,
        _ => return Ok(None),
    };

    let start = peek.slice;

    tokenizer.next()?;

    let ident = tokenizer.next()?;

    let name = match ident.kind {
        TokenKind::Identifier(ident) => VariableName::Identifier(ident),
        TokenKind::Keyword(Keyword::Discard) => VariableName::Discard,
        _ => return Err(ParserError::unexpected_token(ident)),
    };

    let ty = parse_type_annotation(tokenizer)?;

    let next = tokenizer.next()?;

    match next.kind {
        TokenKind::Symbol(Symbol::Assign) => {
            let peek = tokenizer.peek(0)?;
            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::unexpected_token(peek));
            };

            let next = tokenizer.next()?;
            let TokenKind::Symbol(Symbol::Semicolon) = next.kind else {
                return Err(ParserError::unexpected_token(next));
            };

            return Ok(Some(VariableDecl {
                slice: start.merge(&next.slice),
                modifier,
                name,
                ty,
                init: Some(expr),
            }));
        }
        TokenKind::Symbol(Symbol::Semicolon) => {
            return Ok(Some(VariableDecl {
                slice: start.merge(&next.slice),
                modifier,
                name,
                ty,
                init: None,
            }))
        }
        _ => return Err(ParserError::unexpected_token(next)),
    };
}

// fn parse_attrs(tokenizer: &mut Tokenizer) -> Result<Option<Attrs>, ParserError> {
//     let peek = tokenizer.peek(0)?;
//     let TokenKind::Symbol(Symbol::Pound) = peek.kind else {
//         return Ok(None);
//     };
//     let start = peek.slice;
//     tokenizer.next()?;

//     let next = tokenizer.next()?;
//     let TokenKind::Symbol(Symbol::BracketOpen) = next.kind else {
//         return Err(ParserError::unexpected_token(next));
//     };

//     let mut attrs = vec![];
//     let mut last;

//     loop {
//         let attr = parse_attr(tokenizer)?;
//         attrs.push(attr);

//         let next = tokenizer.next()?;
//         last = next.slice.clone();
//         match next.kind {
//             TokenKind::Symbol(Symbol::Comma) => (),
//             TokenKind::Symbol(Symbol::BracketClose) => break,
//             _ => return Err(ParserError::unexpected_token(next)),
//         }
//     }

//     return Ok(Some(Attrs {
//         slice: start.merge(&last),
//         attrs,
//     }));
// }

// fn parse_attr(tokenizer: &mut Tokenizer) -> Result<Attr, ParserError> {
//     let next = tokenizer.next()?;
//     let start = next.slice.clone();

//     let TokenKind::Identifier(name) = next.kind else {
//         return Err(ParserError::unexpected_token(next));
//     };

//     let peek = tokenizer.peek(0)?;
//     let TokenKind::Symbol(Symbol::ParenOpen) = peek.kind else {
//         return Ok(Attr {
//             slice: start,
//             name,
//             params: vec![],
//         });
//     };
//     tokenizer.next()?;

//     let mut params = vec![];
//     let mut last;

//     loop {
//         let param = parse_attr_param(tokenizer)?;

//         params.push(param);

//         let next = tokenizer.next()?;
//         last = next.slice.clone();

//         match next.kind {
//             TokenKind::Symbol(Symbol::Comma) => (),
//             TokenKind::Symbol(Symbol::ParenClose) => break,
//             _ => return Err(ParserError::unexpected_token(next)),
//         }
//     }

//     return Ok(Attr {
//         slice: start.merge(&last),
//         name,
//         params,
//     });
// }

// fn parse_attr_param(tokenizer: &mut Tokenizer) -> Result<AttrParam, ParserError> {
//     let peek = tokenizer.peek(0)?;
//     let start = peek.slice.clone();
//     let name = if let TokenKind::Identifier(ident) = peek.kind
//         && tokenizer.peek(1)?.kind == TokenKind::Symbol(Symbol::Assign)
//     {
//         tokenizer.next()?;
//         tokenizer.next()?;
//         Some(ident)
//     } else {
//         None
//     };

//     if let Some(primitive) = parse_primitive(tokenizer)? {
//         return Ok(AttrParam {
//             slice: start.merge(&primitive.slice),
//             name,
//             value: AttrParamKind::Value(primitive),
//         });
//     }

//     let next = tokenizer.next()?;
//     let TokenKind::Identifier(ident) = next.kind.clone() else {
//         return Err(ParserError::unexpected_token(next));
//     };

//     return Ok(AttrParam {
//         slice: start.merge(&next.slice),
//         name,
//         value: AttrParamKind::Ident(ident),
//     });
// }

fn parse_generics_decl(tokenizer: &mut Tokenizer) -> Result<Option<GenericsDecl>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::Where) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;
    let start = peek.slice;

    let mut tys = vec![];
    let mut last = start.clone();

    while let Some(ty) = parse_generic_type(tokenizer)? {
        last = ty.slice.clone();
        tys.push(ty);
    }

    return Ok(Some(GenericsDecl {
        slice: start.merge(&last),
        tys,
    }));
}

fn parse_generic_type(tokenizer: &mut Tokenizer) -> Result<Option<GenericType>, ParserError> {
    let peek = tokenizer.peek(0)?;

    let TokenKind::Identifier(name) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;

    let start = peek.slice;

    let next = tokenizer.next()?;
    match next.kind {
        TokenKind::Symbol(Symbol::Colon) => (),
        TokenKind::Symbol(Symbol::Semicolon) => {
            return Ok(Some(GenericType {
                slice: start.merge(&next.slice),
                name,
                clauses: vec![],
            }))
        }
        _ => return Err(ParserError::unexpected_token(next)),
    }

    let mut clauses = vec![];

    loop {
        let clause = parse_type_clause(tokenizer)?;

        clauses.push(clause);

        let next = tokenizer.next()?;
        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => (),
            TokenKind::Symbol(Symbol::Semicolon) => {
                return Ok(Some(GenericType {
                    slice: start.merge(&next.slice),
                    name,
                    clauses,
                }))
            }
            _ => return Err(ParserError::unexpected_token(next)),
        }
    }
}

fn parse_type_clause(tokenizer: &mut Tokenizer) -> Result<TypeClause, ParserError> {
    let peek = tokenizer.peek(0)?;
    let start = peek.slice;

    let exclude = if let TokenKind::Symbol(Symbol::BoolNot) = peek.kind {
        tokenizer.next()?;
        true
    } else {
        false
    };

    let peek = tokenizer.peek(0)?;

    match peek.kind {
        TokenKind::Keyword(Keyword::Default) => {
            tokenizer.next()?;
            return Ok(TypeClause {
                slice: start.merge(&peek.slice),
                exclude,
                ty: ClauseKind::Default,
            });
        }
        _ => {
            let ty = parse_type(tokenizer)?;
            return Ok(TypeClause {
                slice: start.merge(&ty.slice),
                exclude,
                ty: ClauseKind::RealType(ty),
            });
        }
    };
}

pub fn parse_trait_body(tokenizer: &mut Tokenizer) -> Result<TraitBody, ParserError> {
    let next = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::BraceOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };
    tokenizer.next()?;

    let start = next.slice;

    let mut decls = vec![];

    if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(0)?.kind {
        tokenizer.next()?;

        let last = tokenizer.next()?.slice;

        return Ok(TraitBody {
            slice: start.merge(&last),
            decls,
        });
    }

    loop {
        let peek = tokenizer.peek(0)?;
        let Some(decl) = parse_lvl_2_decl(tokenizer)? else {
            return Err(ParserError::unexpected_token(peek));
        };

        decls.push(decl);

        if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(0)?.kind {
            tokenizer.next()?;

            let last = tokenizer.next()?.slice;

            return Ok(TraitBody {
                slice: start.merge(&last),
                decls,
            });
        }
    }
}

pub fn parse_int_enum_body(tokenizer: &mut Tokenizer) -> Result<IntEnumBody, ParserError> {
    let next = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::BraceOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let start = next.slice;

    let mut params = vec![];

    if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(1)?.kind {
        tokenizer.next()?;

        let last = tokenizer.next()?.slice;

        return Ok(IntEnumBody {
            slice: start.merge(&last),
            params,
        });
    }

    loop {
        tokenizer.next()?;

        let next = tokenizer.next()?;
        let TokenKind::Identifier(name) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };
        let start = next.slice;

        let peek = tokenizer.peek(0)?;
        if let TokenKind::Symbol(Symbol::Assign) = peek.kind {
            tokenizer.next()?;

            let peek = tokenizer.peek(0)?;

            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::unexpected_token(peek));
            };

            params.push(IntEnumParam {
                slice: start.merge(&expr.slice),
                name,
                value: Some(expr),
            });
        } else {
            params.push(IntEnumParam {
                slice: start,
                name,
                value: None,
            });
        }

        let next = tokenizer.peek(0)?;
        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => {
                if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(1)?.kind {
                    tokenizer.next()?;
                    break;
                }
            }
            TokenKind::Symbol(Symbol::BraceClose) => break,
            _ => return Err(ParserError::unexpected_token(next)),
        }
    }

    let next = tokenizer.next()?;

    return Ok(IntEnumBody {
        slice: start.merge(&next.slice),
        params,
    });
}

pub fn parse_struct_body(tokenizer: &mut Tokenizer) -> Result<StructBody, ParserError> {
    let next = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::BraceOpen) = next.kind else {
        return Err(ParserError::unexpected_token(next));
    };

    let start = next.slice;

    let mut params = vec![];

    if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(1)?.kind {
        tokenizer.next()?;

        let last = tokenizer.next()?.slice;

        return Ok(StructBody {
            slice: start.merge(&last),
            params,
        });
    }

    loop {
        tokenizer.next()?;
        let peek = tokenizer.peek(0)?;

        let is_pub = if let TokenKind::Keyword(Keyword::Pub) = peek.kind {
            tokenizer.next()?;
            true
        } else {
            false
        };

        let next = tokenizer.next()?;
        let TokenKind::Identifier(name) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };

        let next = tokenizer.next()?;
        let TokenKind::Symbol(Symbol::Colon) = next.kind else {
            return Err(ParserError::unexpected_token(next));
        };

        let ty = parse_type(tokenizer)?;

        params.push(StructParam {
            slice: start.merge(&ty.slice),
            is_pub,
            name,
            ty,
        });

        let next = tokenizer.peek(0)?;
        match next.kind {
            TokenKind::Symbol(Symbol::Comma) => {
                if let TokenKind::Symbol(Symbol::BraceClose) = tokenizer.peek(1)?.kind {
                    tokenizer.next()?;
                    break;
                }
            }
            TokenKind::Symbol(Symbol::BraceClose) => break,
            _ => return Err(ParserError::unexpected_token(next)),
        }
    }

    let last = tokenizer.next()?.slice;

    return Ok(StructBody {
        slice: start.merge(&last),
        params,
    });
}

pub fn parse_func_body(
    tokenizer: &mut Tokenizer,
    end_symbol: Option<Symbol>,
) -> Result<FuncBody, ParserError> {
    let peek = tokenizer.peek(0)?;
    let start = peek.slice.clone();
    match peek.kind {
        TokenKind::Symbol(Symbol::WideArrow) => {
            tokenizer.next()?;
            let t = tokenizer.peek(0)?;
            let Some(expr) = parse_expr(tokenizer)? else {
                return Err(ParserError::unexpected_token(t));
            };

            if let Some(sym) = end_symbol {
                let next = tokenizer.next()?;
                if next.kind != TokenKind::Symbol(sym) {
                    return Err(ParserError::unexpected_token(next));
                };

                return Ok(FuncBody {
                    slice: start.merge(&next.slice),
                    kind: FuncBodyKind::Expr(expr),
                });
            }

            return Ok(FuncBody {
                slice: start.merge(&expr.slice),
                kind: FuncBodyKind::Expr(expr),
            });
        }
        TokenKind::Symbol(Symbol::BraceOpen) => {
            let t = tokenizer.peek(0)?;
            let Some(block) = parse_block(tokenizer)? else {
                return Err(ParserError::unexpected_token(t));
            };

            return Ok(FuncBody {
                slice: start.merge(&block.slice),
                kind: FuncBodyKind::Block(block),
            });
        }
        _ => return Err(ParserError::unexpected_token(peek)),
    };
}

fn parse_this_param(tokenizer: &mut Tokenizer) -> Result<Option<ThisParam>, ParserError> {
    let peek = tokenizer.peek(0)?;

    let start = peek.slice;

    let is_mut = if let TokenKind::Keyword(Keyword::Mut) = peek.kind {
        tokenizer.next()?;
        true
    } else {
        false
    };

    let ref_kind = RefKind::parse(tokenizer)?.map(|it| it.1);

    let next = tokenizer.peek(0)?;
    let TokenKind::Keyword(Keyword::ThisValue) = next.kind else {
        if is_mut || ref_kind.is_some() {
            return Err(ParserError::unexpected_token(next));
        }
        return Ok(None);
    };
    tokenizer.next()?;

    return Ok(Some(ThisParam {
        slice: start.merge(&next.slice),
        is_mut,
        ref_kind,
    }));
}

fn parse_type_annotation(tokenizer: &mut Tokenizer) -> Result<Option<Type>, ParserError> {
    let peek = tokenizer.peek(0)?;
    let TokenKind::Symbol(Symbol::Colon) = peek.kind else {
        return Ok(None);
    };
    tokenizer.next()?;

    return Ok(Some(parse_type(tokenizer)?));
}
