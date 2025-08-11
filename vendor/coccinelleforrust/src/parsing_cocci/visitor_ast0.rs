// SPDX-License-Identifier: GPL-2.0

use ra_ide_db::line_index::LineIndex;
use ra_parser::SyntaxKind;
use ra_syntax::{NodeOrToken, SourceFile, SyntaxElement, SyntaxNode, SyntaxToken};
use std::vec;

use crate::{
    commons::{
        info::{
            Modkind, COCCI_DISJ_ID, EDITION
        },
        util::worksnode,
    },
    debugcocci,
};

use super::ast0::{fill_wrap, Snode};

type Tag = SyntaxKind;

fn ttree_to_expr_list(
    tt: String,
    modkind: Modkind,
    whitespace: String,
) -> Result<(Vec<Snode>, Modkind), String> {
    let wrapped = format!(
        "fn func() {{
            fcall({})
        }}",
        tt
    );

    let lindex = LineIndex::new(&wrapped);

    let parse = SourceFile::parse(&wrapped, EDITION);
    let errors = parse.errors();
    let mut err_str = String::new();

    if !errors.is_empty() {
        for error in errors {
            let lindex = lindex.line_col(error.range().start());
            err_str.push_str(&format!(
                "Error : {} at line: {}, col {}\n",
                error.to_string(),
                lindex.line,
                lindex.col
            ));
            err_str.push_str(&format!("{}\n", parse.syntax_node().to_string()));
        }
        return Err(err_str);
    }

    let root = parse.syntax_node();

    let (mut rnode, modkind) = work_node(&lindex, SyntaxElement::Node(root), modkind, whitespace);

    let mut args = rnode //souuce, fn
        .children[3] //blockexpr, stmtlist
        .children[1] //callexpr
        .children
        .remove(1); //arglist

    //removing sorrounding brackets of fcall
    args.children.remove(0);
    args.children.remove(args.children.len() - 1);

    let info = args.children[0].wrapper.info.clone();

    //This makes it as if the expression starts at the start
    //of the file. Later an offset will be added in the calling
    //function
    args.children.iter_mut().for_each(|x| {
        worksnode(x, (), &mut |node, _| {
            node.wrapper.info.pos_info.subtract(info.pos_info.clone());
        })
    });

    return Ok((args.children, modkind));

    //let exprlist = node.chil;
}

// fn is_dots(node: &Syntaxnode) -> bool {
//     let child = node.first_child().unwrap();
//     if child.kind() == syntaxkind::macro_expr {
//         return node.to_string() == wildcard;
//     }
//     false
// }

pub fn work_node<'a>(
    lindex: &LineIndex,
    node: SyntaxElement,
    gmodkind: Modkind,
    whitespace: String,
) -> (Snode, Modkind) {
    let wrap_node = &|lindex: &LineIndex,
                      node: SyntaxElement,
                      modkind: Modkind,
                      df: &mut dyn FnMut(&SyntaxElement, Modkind) -> (Vec<Snode>, Modkind),
                      whitespace: String|
     -> (Snode, Modkind) {
        let mut wrapped = fill_wrap(&lindex, &node);
        let mut kinds = vec![node.kind()];
        let (mut children, modkind) = df(&node, modkind);
        let node = if children.len() == 0 {
            if !modkind.is_context() {
                wrapped.is_modded = true;
                debugcocci!("{}:{:?} set to {:?}", node.to_string(), node.kind(), modkind)
            }
            wrapped.setmodkind(modkind);
            Some(node)
        } else if children.len() == 1 {
            let child = children.remove(0);
            kinds.extend(child.kinds());

            // node = child.asttoken;
            children = child.children;
            wrapped.setmodkind(child.wrapper.mcodekind.to_modkind());

            child.asttoken
        } else {
            None
        };
        // let mut node = if children.len() == 0 { Some(node) } else { None };

        let mut snode = Snode {
            wrapper: wrapped,
            is_dots: false,
            is_fake: false,
            is_disj: false,
            asttoken: node, //Change this to SyntaxElement
            kind: kinds,
            children,
        };
        snode.attach_wspaces_back(whitespace);

        // DEPRECATED
        // if snode.kind() == SyntaxKind::EXPR_STMT && snode.children.len() == 1 {
        //     // this means there is an expression statement without a ; at the ens
        //     //the reason these are removed because rust-analyzer seems to alter between
        //     //assigning ExprStmt and IfExprs(maybe others too)
        //     return snode.children.into_iter().next().unwrap();
        // }
        // // eprintln!("return {:?}", modkind);
        (snode, modkind)
    };

    let df =
        &mut |node: &NodeOrToken<SyntaxNode, SyntaxToken>, mut modkind: Modkind| -> (Vec<Snode>, Modkind) {
            // // eprintln!("got {:?}", modkind);
            let mut children: Vec<Snode> = vec![];
            let mut whitespace = String::new();
            //let mut children = vec![];

            match node {
                SyntaxElement::Node(node) => {
                    for child in node.children_with_tokens() {
                        // // eprintln!("next {:?}", child.kind());
                        match child.kind() {
                            /* */
                            // Tag::COMMENT => {
                            //     // let commlen: usize = child.text_range().len().into();
                            //     let commstr = child.to_string();
                            //     if commstr == MINUS_MOD_START
                            //         || commstr == MINUS_MOD_END
                            //         || commstr == PLUS_MOD_START
                            //         || commstr == PLUS_MOD_END
                            //     // && lindex.line_col(child.text_range().start()).col == 0
                            //     {
                            //         //checks for /*??*/ modifiers
                            //         modkind = get_modkind(&commstr);
                            //         //in the next iteration the node gets the modkind
                            //     } else if child.to_string().eq(WILDCARD) {
                            //         let mut wc =
                            //             Snode::make_dots(super::ast0::Mcodekind::Context(vec![], vec![])); //the mcode here is just a dummy

                            //         // if !modkind.is_context() {
                            //         //     wc.wrapper.is_modded = true;
                            //         // }
                            //         // turning on is_modded in wrapper is taken care of
                            //         // inside the setmodkind function
                            //         wc.wrapper.setmodkind(modkind);
                            //         children.push(wc);
                            //         modkind = Modkind::Context;
                            //         whitespace.push_str(&child.to_string());
                            //     } else {
                            //         // Comment by user
                            //         whitespace.push_str(&child.to_string());
                            //     }
                            // }
                            Tag::TOKEN_TREE
                                if children.len() >= 2 // ..., Ident, Bang, ...
                                && children[children.len() - 2].getstring() != COCCI_DISJ_ID =>
                            //checks if the currently processed macro is a disj
                            {
                                let (mut exprlist, tmpm) = match ttree_to_expr_list(
                                    child.as_node().unwrap().to_string(),
                                    modkind,
                                    whitespace.clone(),
                                ) {
                                    Ok(exprlist) => exprlist,
                                    Err(_) => {
                                        let ret = work_node(lindex, child, modkind, whitespace);
                                        whitespace = String::new();
                                        modkind = ret.1;
                                        children.push(ret.0);
                                        continue;
                                    }
                                };
                                whitespace = String::new();

                                modkind = tmpm;
                                let info = work_node(lindex, child, modkind, String::new()).0.wrapper.info;

                                //position is fixed only for errors
                                exprlist.iter_mut().for_each(|x| {
                                    worksnode(x, (), &mut |node, _| {
                                        node.wrapper.info.pos_info.add(info.pos_info.clone());
                                    })
                                });
                                children.extend(exprlist);
                            }
                            Tag::WHITESPACE => {
                                whitespace.push_str(&child.to_string());
                            }
                            _ => {
                                let (child, md) = work_node(lindex, child, modkind, whitespace);
                                whitespace = String::new();
                                children.push(child);
                                modkind = md;
                                // modkind = None;
                            }
                        }
                    }
                }
                SyntaxElement::Token(_token) => {
                    // To note
                    // Tokens dont have children
                    // A token is repeated after every leaf node
                    // So the redundancy is removed here
                }
            }
            // // eprintln!("Leaving {:?}", modkind);
            (children, modkind)
        };

    wrap_node(lindex, node, gmodkind, df, whitespace)
}
