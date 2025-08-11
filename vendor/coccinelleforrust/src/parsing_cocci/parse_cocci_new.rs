use std::{collections::HashSet, rc::Rc};

use itertools::Itertools;
use ra_parser::SyntaxKind;

use crate::{
    commons::{
        info::{Modkind, MINUS_MOD_END, MINUS_MOD_START, PLUS_MOD_END, PLUS_MOD_START},
        util::{self, attach_pluses_back, attach_pluses_front, collecttree, range_in, worksnode},
    },
    ctl::{
        ctl_ast::{GenericCtl, GenericSubst},
        ctl_engine::{Pred, Subs},
        wrapper_ctl::make_ctl_simple,
    },
    debugcocci,
    engine::ctl_cocci::{BoundValue, Predicate},
    parsing_cocci::{
        ast0::{wrap_root, Mcodekind, SnodeMutRef},
        disjunctions::{expand_disj, resolve_snodes},
        lexer::CocciLexer,
    },
    smpl_grammar,
};

use super::{
    ast0::{MetaVar, MetavarName, MetavarType, Snode},
    disjunctions::ModRange,
    smpl_grammar::DisjElem,
};

#[derive(Clone)]
pub struct Patch {
    pub minus: Snode,
    pub plus: Snode,
}

#[derive(Clone)]
pub struct Rule {
    pub name: String,
    pub dependson: Dep,
    pub metavars: Vec<MetaVar>,
    pub unusedmetavars: Vec<MetaVar>,
    pub patch: Patch,
    pub freevars: Vec<MetaVar>,
    pub usedafter: HashSet<MetavarName>,
    pub hastype: bool,
    pub ctl: GenericCtl<
        <Predicate as Pred>::ty,
        <Rc<GenericSubst<MetavarName, BoundValue>> as Subs>::Mvar,
        Vec<String>,
    >,
}

#[derive(Clone)]
pub enum Dep {
    None,
}

fn insert_dots(node: &mut Snode, dots: &usize, offset: usize) -> bool {
    // eprintln!(
    //     "{} {:?} {} {}",
    //     &node.getstring().chars().take(80).collect::<String>(),
    //     node.wrapper.info.pos_info,
    //     dots,
    //     offset
    // );
    let mut toins = None;
    if node.children.len() == 0 {
        return false;
    } else if node.children.len() == 1 {
        return insert_dots(&mut node.children[0], dots, offset);
    } else {
        let clen = node.children.len();
        for i in 1..clen {
            let index = offset + dots + 1;
            if index > node.children[i - 1].get_offset() && index < node.children[i].get_offset()
            //checks if the dots are present between two children
            {
                toins = Some(i);
                break;
            }
            // if child.wrapper.info.pos_info.offset == offset + dots + 2 {
            //     toins = Some(i);
            //     break;
            // }
            if insert_dots(&mut node.children[i], dots, offset) {
                return true;
            }
        }
        if toins.is_some() {
            let index = toins.unwrap();
            debugcocci!(
                "Inserted Dots before element {}:{:?}",
                node.children[index].getstring(),
                node.children[index].kinds()
            );
            node.children
                .insert(index, Snode::make_dots(Mcodekind::Context(vec![], vec![])));
            return true;
        } else {
            return false;
        }
    }
}

impl Patch {
    // In many cases Dots need to be represented by an ident
    // and not a comment, so this does that
    pub fn fix_dots(&mut self) {
        let f = &mut |node: &mut Snode, _| {
            if node.has_kind(&SyntaxKind::TUPLE_EXPR) && node.children[1].is_dots {
                //TupleKind always has atleast two children because of the Parens
                node.change_kinds(&[SyntaxKind::PAREN_EXPR]);
            }
        };

        worksnode(&mut self.minus, (), f);
        worksnode(&mut self.plus, (), f);
    }

    fn setmetavars(&mut self, metavars: &Vec<MetaVar>) {
        fn setmetavars_aux(node: &mut Snode, metavars: &Vec<MetaVar>) {
            let mut work = |node: &mut Snode| {
                //The if statement below lists the types of metavariables allowed
                if node.isexpr() || node.istype() || node.isid() || node.islifetime() || node.isparam() {
                    let stmp = node.getstring(); //FIX ME should not convert to string before checking
                    if let Some(mvar) = metavars.iter().find(|x| x.getname() == stmp) {
                        debugcocci!("MetaVar found - {:?}", mvar);
                        node.wrapper.metavar = mvar.clone();
                    }
                }
            };
            util::worktree(node, &mut work);
        }

        setmetavars_aux(&mut self.plus, metavars);
        setmetavars_aux(&mut self.minus, metavars);
    }

    // fn set_minus(&mut self) {
    //     fn aux(node: &mut Snode, modkind: ModKind) {
    //         // if node.
    //     }
    // }
    fn _make_snodes_from_str(minusbuf: &str, plusbuf: &str) -> Patch {
        fn aux(st: &str) -> Snode {
            match wrap_root(st) {
                Ok(node) => node,
                Err(errs) => {
                    let mut new_str = String::new();
                    // Handle Params TODO
                    errs.iter().for_each(|x| new_str.push_str(&x.node_str));

                    panic!("Errors encounteresd while parsing cocci.\n{}\n", new_str)
                }
            }
        }
        Patch {
            minus: aux(minusbuf),
            plus: aux(plusbuf),
        }
    }

    pub fn group_dots(&mut self) {
        self.minus = group_dots(&self.minus);
    }

    pub fn process_dots(&mut self) {
        self.group_dots();
        process_dots_plus(&mut self.minus);
    }

    pub fn remove_comments(&mut self) {
        //This removes the padding comments //****** which are
        // inserted during the handle_mods phase. They ensure that
        // The indices are maintianed when disjiunctions are resolved.
        fn aux(snode: &mut Snode) {
            snode.children.iter_mut().for_each(|x| aux(x));
            snode
                .children
                .retain(|i| !i.has_kind(&SyntaxKind::COMMENT) || i.is_disj || i.is_dots || i.is_fake);
            if snode.children.len() == 1 && !snode.is_disj && !snode.is_fake && !snode.is_dots {
                let child = snode.children.remove(0);
                snode.kind.extend(child.kinds());

                snode.children = child.children;
                snode.wrapper.setmodkind(child.wrapper.mcodekind.to_modkind());
                snode.asttoken = child.asttoken;
            }
        }
        aux(&mut self.minus);
        aux(&mut self.plus);
    }

    // pub fn process_disjs(self, _has_type: bool) -> Self {
    //     super::disjunctions::process_disjs_old(self, _has_type)
    // }

    //remove let from around the type
    pub fn striplet(&mut self, hastype: bool) {
        if !hastype {
            return;
        }

        fn striplet_aux(node: &mut Snode) {
            //at this point node is a SourceFile
            //with a function with a stmtlist without braces
            let stmtlist = &mut node.children[3]; //function
            if stmtlist.children.len() == 0 {
                //handles empty type patches
                //Either no type or only minus
                return;
            }
            stmtlist.children = vec![stmtlist.children[0] //letstmt
                .children[3]
                .clone()] //Type;
        }
        striplet_aux(&mut self.plus);
        striplet_aux(&mut self.minus);
    }

    pub fn tag_plus(&mut self) {
        fn tagplus_aux<'a>(
            achildren1: Vec<SnodeMutRef<'a>>,
            bchildren1: Vec<SnodeMutRef<'a>>,
        ) -> Result<Vec<&'a mut Snode>, ()> {
            //There is no need to propagate pluses
            //because a plus cannot exist without code around it
            //when a '+' mod is written an ast is pushed at that
            //very level in the tree. That is I cannot write a plus
            //statement after a minus or context code and not have it
            //in a level same as the statement above it even around braces
            let mut pvec: Vec<Snode> = vec![];
            let mut achildren = achildren1.into_iter();
            let mut bchildren = bchildren1.into_iter();
            let mut lasta = vec![];
            let mut a = achildren.next();
            let mut b = bchildren.next();
            loop {
                // eprintln!("comparing {:?} {:?}", a, b);
                match (a, b) {
                    (Some(ak), Some(bk)) => {
                        // eprintln!("Matching {:?} with {:?}", ak, bk);
                        if ak.is_disj() || bk.is_disj() && false {
                            assert!(ak.is_disj() && bk.is_disj());
                            let disjid = ak.get_disj_id().unwrap();
                            let mut at = ak.get_disj().unwrap();
                            let mut bt = bk.get_disj().unwrap();

                            assert_eq!(at.len(), bt.len(), "[Internal Error] Plus and Minus trees have different Disjunction branch lengths for Disjunction:{}", disjid);
                            let len = at.len();

                            let mut tmp = vec![];
                            // swap_remove used because we dont care about order
                            for _ in 0..len {
                                tmp.extend(
                                    tagplus_aux(at.swap_remove(0), bt.swap_remove(0)).expect(&format!(
                                        "[Error] Error while tagging Disjunction {}",
                                        disjid
                                    )),
                                );
                            }
                            lasta = tmp;

                            a = achildren.next();
                            b = bchildren.next();
                        } else {
                            match (&ak.get_mcodekind(), &bk.get_mcodekind()) {
                                (Mcodekind::Minus(_), _) => {
                                    //minus code
                                    //with any thing other than a plus
                                    // // eprintln!("Comes with {:?}", pvec);

                                    // pvec should be empty. because if plus is persent
                                    // before minus the minus is consumed before
                                    // the plus is encountered
                                    assert_eq!(pvec.len(), 0);

                                    // attach_pluses_front(ak, pvec);
                                    // pvec = vec![];

                                    if ak.get_snode_ref().unwrap().is_dots {
                                        lasta = vec![]
                                    } else {
                                        lasta = vec![ak.get_snode().unwrap()];
                                    }
                                    a = achildren.next();
                                    b = Some(bk);
                                }
                                (_, Mcodekind::Plus) => {
                                    pvec.push((*bk.get_snode_ref().unwrap()).clone());
                                    a = Some(ak);
                                    b = bchildren.next();
                                }
                                (Mcodekind::Context(_, _), Mcodekind::Context(_, _)) => {
                                    //context code
                                    //with context code
                                    // if ak.wrapper.isdisj {
                                    //     //DISJUNCTIONS ARE THE ONLY PART
                                    //     //WHERE PLUSES ARE ADDED TO A NODE
                                    //     //AND NOT A TOKEN
                                    //     ak.wrapper.mcodekind.push_pluses_front(pvec);
                                    //     //ak.wrapper.plusesbef.extend(pvec);
                                    // } else
                                    if ak.is_disj() || bk.is_disj() {
                                        assert!(ak.is_disj() && bk.is_disj());
                                        let disjid = ak.get_disj_id().unwrap();
                                        let mut at = ak.get_disj().unwrap();
                                        let mut bt = bk.get_disj().unwrap();

                                        assert_eq!(at.len(), bt.len(), "[Internal Error] Plus and Minus trees have different Disjunction branch lengths for Disjunction:{}", disjid);
                                        let len = at.len();

                                        let mut tmp = vec![];
                                        // swap_remove used because we dont care about order
                                        for _ in 0..len {
                                            tmp.extend(
                                                tagplus_aux(at.swap_remove(0), bt.swap_remove(0)).expect(
                                                    &format!(
                                                        "[Error] Error while tagging Disjunction {}",
                                                        disjid
                                                    ),
                                                ),
                                            );
                                        }
                                        lasta = tmp;

                                        a = achildren.next();
                                        b = bchildren.next();
                                    } else {
                                        let is_dots = ak.get_snode_ref().unwrap().is_dots;
                                        let snode = ak.get_snode().unwrap();
                                        attach_pluses_front(snode, pvec);

                                        pvec = vec![];
                                        if is_dots {
                                            lasta = vec![];
                                        } else {
                                            lasta = vec![snode];
                                        }
                                        a = achildren.next();
                                        b = bchildren.next();
                                    }
                                }
                                (Mcodekind::Star, Mcodekind::Star) => {
                                    // lasta = vec![ak.get_snode().unwrap()];
                                    // a = achildren.next();
                                    // b = bchildren.next();
                                    panic!("The star modifier is under maintainance");
                                }
                                (Mcodekind::Star, _) | (_, Mcodekind::Star) => {
                                    panic!("Minus and Plus buffers do not have matching stars");
                                }
                                _ => {
                                    panic!(
                                    "There are plusses in the minus buffer, or minuses in the plus buffer."
                                );
                                }
                            }
                        }
                    }
                    (None, Some(bk)) => match bk.get_snode_ref().unwrap().wrapper.mcodekind {
                        Mcodekind::Plus => {
                            pvec.push((*bk.get_snode_ref().unwrap()).clone());
                            a = None;
                            b = bchildren.next();
                        }
                        _ => {
                            break;
                        }
                    },
                    (Some(_snode), None) => {
                        break;
                    } //means only minuses are left
                    (None, None) => {
                        break;
                    }
                }
            }

            if pvec.len() != 0 {
                //Plus statements exist after
                //the context and need to be attached to the
                //closes context above/before
                lasta
                    .into_iter()
                    .for_each(|x| attach_pluses_back(x, pvec.clone()));
                // attach_pluses_back(lasta.unwrap().get_last_elem_mut(), pvec);
                return Ok(vec![]);
            } else {
                return Ok(lasta);
            }
        }

        let v1 = self.minus.into_preorder_mut();
        let v2 = self.plus.into_preorder_mut();
        // // eprintln!("v1 - {:?}\nv2 - {:?}", v1.iter().map(|x| x.getstring()).join(" "), v2.iter().map(|x| x.getstring()).join(" "));
        tagplus_aux(v1, v2)
            .expect("[Error] Error merging plus and minus trees while parsing semantic patch.");
    }

    pub fn getunusedmetavars(&self, bindings: &Vec<MetaVar>) -> Vec<MetaVar> {
        let mut used = vec![];
        let mut f = |x: &Snode| match &x.wrapper.metavar {
            MetaVar::NoMeta => {}
            MetaVar::Exp(info)
            | MetaVar::Id(info)
            | MetaVar::Lifetime(info)
            | MetaVar::Type(info)
            | MetaVar::Parameter(info)
            | MetaVar::Adt(_, info) => {
                if let Some(mvar) = bindings.iter().find(|x| x.getname() == info.0.varname) {
                    used.push(mvar.clone());
                }
            }
        };

        collecttree(&self.minus, &mut f);
        collecttree(&self.plus, &mut f);
        debugcocci!("Used Metavars:- {:?}", used);
        let unused = bindings
            .iter()
            .filter(|x| !used.contains(x))
            .map(|x| x.clone())
            .collect_vec();
        return unused;
    }

    pub fn insert_dots(&mut self, dots: &Vec<usize>, pno: usize, mno: usize) {
        for dot in dots {
            if !insert_dots(&mut self.minus, dot, mno) {
                panic!("Cannot insert dot at position {dot} and offset {mno} in minus buffer.");
            }
            if !insert_dots(&mut self.plus, dot, pno) {
                panic!("Cannot insert dot at position {dot} and offset {pno} in plus buffer.");
            }
        }
    }
}

// Gives ((a...b)...c)
// This is important for process_dots_plus
fn group_dots(snode: &Snode) -> Snode {
    if snode.children.is_empty() {
        return snode.clone();
    }

    let rchildren = snode.children.iter();
    let mut nchidlren: Vec<Snode> = vec![];
    let mut prev: Option<Snode> = None; //unwrap because the list is not empty
    let mut prev_dot: Option<Snode> = None;

    for child in rchildren {
        let child = group_dots(child);
        match (prev_dot.is_some(), prev.is_some(), child.is_dots) {
            (true, true, true) => {
                //b  ...       a      ...
                //  ^prev_dot  ^ prev ^child
                let mut dots = Snode::make_dots(child.wrapper.mcodekind);
                dots.children = vec![prev_dot.unwrap()];
                prev_dot = Some(dots);
                prev = None;
            }
            (true, true, false) => {
                // ...        b        c
                // ^prev_dots ^prev    ^child
                nchidlren.push(prev_dot.unwrap());
                prev = Some(child);
                prev_dot = None;
            }
            (true, false, true) => {
                // ... x ...
                //not possible
                panic!("Not possible")
            }
            (true, false, false) => {
                // ... x c
                prev_dot.as_mut().unwrap().children.push(child.clone());
                prev = Some(child);
            }
            (false, true, true) => {
                //b ...
                let mut dots = Snode::make_dots(child.wrapper.mcodekind);
                dots.children = vec![prev.unwrap()];
                prev = None;
                prev_dot = Some(dots);
            }
            (false, true, false) => {
                // a b c
                nchidlren.push(prev.unwrap());
                prev = Some(child);
                prev_dot = None;
            }
            (false, false, true) => {
                // x x ...
                //starts with ...
                let mut dots = Snode::make_dots(child.wrapper.mcodekind);
                dots.children = vec![Snode::make_fake()];
                prev = None;
                prev_dot = Some(dots);
            }
            (false, false, false) => {
                //x x b
                //start
                prev = Some(child);
                prev_dot = None;
            }
        }
    }

    match (prev_dot.is_some(), prev.is_some()) {
        (true, true) => {
            //... a
            nchidlren.push(prev_dot.unwrap());
        }
        (true, false) => {
            //ends with ...
            let mut dot = prev_dot.unwrap();
            dot.children.push(Snode::make_fake());
            nchidlren.push(dot);
        }
        (false, true) => {
            //a b
            nchidlren.push(prev.unwrap());
        }
        (false, false) => {
            //x
            panic!("Not possibe")
        }
    }

    let mut snode = snode.clone_without_children();
    snode.children = nchidlren;
    return snode;
}

pub fn handlemods(block: &Vec<&str>) -> (String, String) {
    let mut plusbuf = String::new();
    let mut minusbuf = String::new();

    for line in block {
        if line.starts_with(PLUS_MOD_START) {
            plusbuf.push_str(line);
            plusbuf.push('\n');
            let to_add_line = line.len() - 2;

            minusbuf.push_str("//");
            minusbuf.push_str(&"*".repeat(to_add_line)); //to make the indices match up for both the buffers
            minusbuf.push('\n');
        } else if line.starts_with(MINUS_MOD_START) {
            minusbuf.push_str(line);
            minusbuf.push('\n');
            let to_add_line = line.len() - 2;

            plusbuf.push_str("//");
            plusbuf.push_str(&"*".repeat(to_add_line)); //to make the indices match up for both the buffers
            plusbuf.push('\n');
        } else {
            plusbuf.push_str(line);
            plusbuf.push('\n');

            minusbuf.push_str(line);
            minusbuf.push('\n');
        }
    }
    return (plusbuf, minusbuf);
}

fn process_dots_plus(snode: &mut Snode) {
    if snode.children.is_empty() {
        return;
    }

    fn aux(snode: &mut Snode, pluses: Vec<Snode>) {
        if snode.is_dots {
            aux(&mut snode.children[0], pluses);
        } else {
            attach_pluses_back(snode, pluses);
        }
    }

    if snode.is_dots {
        if snode.wrapper.mcodekind.has_pluses() {
            let pluses = snode.wrapper.mcodekind.getpluses();
            aux(&mut snode.children[0], pluses.0);
            attach_pluses_front(&mut snode.children[1], pluses.1);
            // snode.children[1].wrapper.mcodekind.push_pluses_front(pluses.1);
        }
    }

    for snode in &mut snode.children {
        process_dots_plus(snode)
    }
}

fn get_body(snode: &mut Snode) {
    let stmtlist = &mut snode.children[3];
    let _bef = stmtlist.children.remove(0);
    let _aft = stmtlist.children.remove(stmtlist.children.len() - 1);
}

fn parse_patch_from_snode(pnode: Snode, mnode: Snode, metavars: &Vec<MetaVar>, hastype: bool) -> Patch {
    let mut patch = Patch {
        plus: pnode,
        minus: mnode,
    };

    patch.setmetavars(metavars);
    patch.remove_comments(); // Keep this before get_body else, it will mess it up
    patch.minus.print_tree_kinds();
    get_body(&mut patch.minus);
    get_body(&mut patch.plus);
    patch.striplet(hastype);
    patch.tag_plus();
    // eprintln!("patch tagplus minus - {}", patch.minus.getstring());

    // eprintln!("Before dots grouping");
    patch.process_dots();
    // patch.minus.print_tree_kinds();
    patch
}

// Only for continuous code without
// disjunctions
fn wrap_code(currrulename: &str, pbufmod: &str, blanks: usize, istype: bool, suf: &str) -> (String, usize) {
    let mut plusbuf = String::new();
    plusbuf.push_str(format!("fn {currrulename}_{suf}() {{\n").as_str());

    plusbuf.push_str(&"\n".repeat(blanks));

    let mut pno = 9 + suf.len() + currrulename.len() + blanks;

    if istype {
        let pbufmod = if pbufmod.trim() != "" {
            pno += 15;
            format!("let COCCIVAR: \n{}\n;", pbufmod)
        } else {
            String::new()
        };

        plusbuf.push_str(&pbufmod);
    } else {
        plusbuf.push_str(pbufmod);
    }

    plusbuf.push_str("}");

    (plusbuf, pno)
}

fn set_mods(node: &mut Snode, minuses: Vec<ModRange>, pluses: Vec<ModRange>) {
    let set_mods = &mut |node: &mut Snode, _| {
        if node.children.len() == 0 {
            if minuses.iter().any(|(x, y)| range_in(node.get_pos(), (*x, *y))) {
                node.wrapper.setmodkind(Modkind::Minus);
            }

            if pluses.iter().any(|(x, y)| range_in(node.get_pos(), (*x, *y))) {
                node.wrapper.setmodkind(Modkind::Plus);
            }
        }
    };

    worksnode(node, (), set_mods);
}

fn buildrule(
    currrulename: &str,
    currdepends: Dep,
    mut metavars: Vec<MetaVar>, //mutable because unused metavars are removed
    _blanks: usize,
    p_patch: Vec<DisjElem>,
    n_patch: Vec<DisjElem>,
    _lastruleline: usize,
    istype: bool,
) -> Rule {
    // we can check either ppatch or npatch
    // They should have a simillar structureS
    // Mod mixed with Disj

    // Here I have expanded the single Disjuntion Branch into a list of
    // Strings along with some disjunction information and others which can
    // be parsed by RA. These strings already have their mod operator converted
    // to comments. So when we call wrap_root, the mods will be assigned.
    // But here the mods still havent been distinguished. We dont have the plus/minus
    // snodes yet.
    let expanded = expand_disj(&mut (None, p_patch), &mut 0); // The None is just dummy
    let p_snodes = expanded
        .into_iter()
        .map(|(raw, mut oinfo)| {
            let (wrapped, ono) = wrap_code(currrulename, &raw, 0, istype, "plus");
            let mut snode = wrap_root(&wrapped)
                .expect("[INTERNAL ERROR] Malformed Semantic patch during disjunction expansion");
            oinfo.update_all(ono, 0);
            oinfo.dots.iter().for_each(|d| {
                insert_dots(&mut snode, d, 0);
            });
            set_mods(&mut snode, oinfo.minuses, oinfo.pluses);
            // eprintln!("psnode - {}", snode.getstring());
            (snode, oinfo.dinfo)
        })
        .collect_vec();
    let psnode = resolve_snodes(p_snodes);
    psnode.print_tree_kinds();

    let expanded = expand_disj(&mut (None, n_patch), &mut 0); // The None is just dummy
    let n_snodes = expanded
        .into_iter()
        .map(|(raw, mut oinfo)| {
            // eprintln!("oinfo - {:?}", oinfo);
            let (wrapped, ono) = wrap_code(currrulename, &raw, 0, istype, "minus");
            let mut snode = wrap_root(&wrapped)
                .expect("[INTERNAL ERROR] Malformed Semantic patch during disjunction expansion");
            oinfo.update_all(ono, 0);
            oinfo.dots.iter().for_each(|d| {
                insert_dots(&mut snode, d, 0);
            });
            set_mods(&mut snode, oinfo.minuses, oinfo.pluses);
            // eprintln!("nsnode - {}", snode.getstring());
            (snode, oinfo.dinfo)
        })
        .collect_vec();

    let nsnode = resolve_snodes(n_snodes);
    let patch = parse_patch_from_snode(psnode, nsnode, &metavars, istype);

    // eprintln!("assimilated node - {}", patch.minus.getstring());
    // patch.minus.print_tree_kinds();
    let unused_mvars = patch.getunusedmetavars(&metavars);
    let mut freevars = vec![];

    for metavar in &unused_mvars {
        eprintln!(
            "Warning: Unused metavariable {}.{}",
            metavar.getrulename(),
            metavar.getname()
        );
        if let Some(index) = metavars.iter().position(|x| x.getname() == metavar.getname()) {
            //All this will be optimised when using hashsets
            metavars.remove(index);
        }
    }

    for metavar in &metavars {
        if metavar.getrulename() == currrulename {
            debugcocci!("Freevar found- {:?}", metavar);
            freevars.push(metavar.clone());
        }
    }

    let ctl = make_ctl_simple(&get_stmtlist(&patch.minus), false, &freevars);
    // eprintln!("CTL - {}", ctl);
    // panic!();

    return Rule {
        name: String::from(currrulename),
        dependson: currdepends,
        metavars: metavars,
        unusedmetavars: unused_mvars,
        patch: patch,
        freevars: freevars,
        usedafter: HashSet::new(),
        hastype: istype,
        ctl: ctl,
    };
}

/// Gets the stmtlist for the collapsed tree
fn get_stmtlist<'a>(snode: &'a Snode) -> &'a Snode {
    &snode.children[3]
}

fn parse_metavars(
    rules: &Vec<Rule>,
    rulename: &str,
    metavars: &Vec<(&str, Vec<&str>)>,
) -> (Vec<MetaVar>, bool) {
    fn makemetavar(rules: &Vec<Rule>, rulename: &str, mvar: &str, mtype: &MetavarType) -> MetaVar {
        if !mvar.contains(".") {
            return MetaVar::new(rulename, mvar, mtype, false);
        } else {
            let split = mvar.split(".").collect_vec();
            if split.len() > 2 {
                panic!("Invalid Metavariable: {}", mvar);
            }
            let rule = rules
                .iter()
                .find(|x| x.name == split[0].trim())
                .expect("Invalid inherited Metavar rule");
            let varname = split[1].trim();
            if let Some(mvar) = rule.metavars.iter().find(|x| x.getname() == varname) {
                if let Some(_minfo) = rule.unusedmetavars.iter().find(|x| x.getname() == varname) {
                    panic!("Cannot inherit unused metavars")
                }
                debugcocci!("Added Metavar {}.{}", mvar.getrulename(), mvar.getname());
                let inhertedvar = mvar.makeinherited();
                return inhertedvar;
            } else {
                panic!("No metavariable in {} rule {}", varname, split[0]);
            }
        }
    }

    let mut all_mvars: Vec<MetaVar> = vec![];
    let mut has_adt_types = false;

    for (mtype, mvars) in metavars {
        let ty = MetavarType::build(mtype);
        if ty.is_adt() {
            has_adt_types = true;
        }

        for mvar in mvars {
            let mvar = mvar.trim().to_string();
            if !all_mvars.iter().any(|x| x.getname() == mvar) {
                all_mvars.push(makemetavar(rules, rulename, &mvar, &ty));
            }
        }

        // match *mtype {
        //     "expression" => mvars.iter().map(|mvar|  MetaVar::Exp(format!("{}.{}", rulename, mvar), KeepBinding::UNITARY, true)).collect_vec(),
        //     "identifier" => mvars.iter().map(|mvar|  MetaVar::Id(format!("{}.{}", rulename, mvar))).collect_vec(),
        //     "type" => mvars.iter().map(|mvar|  MetaVar::Type(format!("{}.{}", rulename, mvar))).collect_vec(),
        //     "lifetime" => mvars.iter().map(|mvar|  MetaVar::Lifetime(format!("{}.{}", rulename, mvar))).collect_vec(),
        //     "Parameter" => mvars.iter().map(|mvar|  MetaVar::Parameter(format!("{}.{}", rulename, mvar))).collect_vec(),
        //     adt => mvars.iter().map(|mvar|  MetaVar::Adt(format!("{}.{}", rulename, mvar))).collect_vec(),
        // }
    }

    (all_mvars, has_adt_types)
}

/// Converts mods to comments to make
/// it parsable
fn replace_mods(contents: &str) -> String {
    let mut new_buf = String::new();
    for line in contents.lines() {
        match line.chars().next() {
            Some('+') => new_buf.push_str(&format!("{}{}{}\n", PLUS_MOD_START, &line[1..], PLUS_MOD_END)),
            Some('-') => new_buf.push_str(&format!("{}{}{}\n", MINUS_MOD_START, &line[1..], MINUS_MOD_END)),
            Some('*') => new_buf.push_str(&format!("{}{}{}\n", MINUS_MOD_START, &line[1..], MINUS_MOD_END)),
            Some(_) => new_buf.push_str(&format!("{line}\n")),
            None => new_buf.push('\n'),
        }
    }
    return new_buf;
}

fn preprocess(s: &str) -> String {
    let mut st = String::new();
    let lines = s.lines().collect_vec();
    let nlines = lines.len();

    for lineno in 0..nlines {
        if lines[lineno].starts_with("(") && lines[lineno + 1].starts_with("(") {
            st.push_str("(\n");
            st.push('\n');
        } else if lineno < nlines - 1 && lines[lineno].starts_with(")") && lines[lineno].starts_with(")") {
            st.push_str(")\n");
            st.push('\n');
        } else {
            st.push_str(lines[lineno]);
            st.push('\n');
        }
    }
    return st;
}

// fn parse_simple(contents: &str) -> Result<Snode, String> {}
pub fn process_cocci(contents: &str) -> (Vec<Rule>, bool, bool) {
    let contents = &preprocess(contents);
    let contents = &replace_mods(&contents);
    let (plusbuf, minusbuf) = handlemods(&contents.lines().collect_vec());

    // eprintln!("plsbuf - {}\n minusbuf - {}", plusbuf, minusbuf);
    let lexer = CocciLexer::new(&plusbuf);
    // let lexer = lexer.inspect(|x| eprintln!("{:?}", x));

    let mut prules = smpl_grammar::SPatchParser::new()
        .parse(&plusbuf, lexer)
        .expect("ERROR HANDLING NOT IMPLEMENTED");

    let lexer = CocciLexer::new(&minusbuf);
    // let lexer = lexer.inspect(|x| eprintln!("{:?}", x));

    let mut nrules = smpl_grammar::SPatchParser::new()
        .parse(&minusbuf, lexer)
        .expect("ERROR HANDLING NOT IMPLEMENTED");

    assert_eq!(prules.len(), nrules.len());

    let mut rules = vec![];
    let hastypes = false;

    for i in 0..prules.len() {
        let prule = prules.remove(i);
        let nrule = nrules.remove(i);

        // Some extra work is done as the header is parsed twice
        // Blame my shortsightedness ugh
        let (_, _, p_patch, _, _) = prule;
        let (rulename, metavars, n_patch, istype, patch_offset) = nrule;

        //For a given rule this should be the same
        let (metavars, _hast_adt_types) = parse_metavars(&rules, rulename, &metavars);

        rules.push(buildrule(
            rulename,
            Dep::None,
            metavars,
            patch_offset,
            p_patch.1,
            n_patch.1,
            0,
            istype,
        ));
    }
    (rules, false, hastypes)
}
