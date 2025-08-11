use std::cmp::Ordering;

use itertools::Itertools;

use crate::{
    commons::info::{MINUS_MOD_END, MINUS_MOD_START, PLUS_MOD_END, PLUS_MOD_START},
    debugcocci,
    parsing_cocci::ast0::{PositionInfo, Snode},
};

use super::smpl_grammar::{DisjBranch, DisjElem};

pub type DisjId = (usize, usize); //disj number, branch number
type DisjRange = (usize, usize);
pub type ModRange = (usize, usize);
type Dinfo = (DisjId, DisjRange); //disj_id, start_char, end_char

#[derive(Clone, Debug)]
pub struct OInfo {
    pub dinfo: Vec<Dinfo>,
    pub minuses: Vec<ModRange>,
    pub pluses: Vec<ModRange>,
    pub dots: Vec<usize>,
}

impl OInfo {
    pub fn add_minuses(&mut self, elem: ModRange) {
        self.minuses.push(elem);
    }

    pub fn add_pluses(&mut self, elem: ModRange) {
        self.pluses.push(elem);
    }

    pub fn add_dots(&mut self, elem: Vec<usize>) {
        self.dots.extend(elem);
    }

    pub fn add_dinfo(&mut self, elem: Dinfo) {
        self.dinfo.insert(0, elem);
    }

    pub fn new(dinfo: Vec<Dinfo>, minuses: Vec<ModRange>, pluses: Vec<ModRange>, dots: Vec<usize>) -> Self {
        return OInfo {
            dinfo,
            minuses,
            pluses,
            dots,
        };
    }

    pub fn empty() -> Self {
        return OInfo {
            dinfo: vec![],
            minuses: vec![],
            pluses: vec![],
            dots: vec![],
        };
    }

    pub fn update_all(&mut self, stlen: usize, offset: usize) {
        self.minuses.iter_mut().for_each(|(x, y)| {
            *x = *x + stlen + offset;
            *y = *y + stlen + offset;
        });

        self.pluses.iter_mut().for_each(|(x, y)| {
            *x = *x + stlen + offset;
            *y = *y + stlen + offset;
        });

        self.dots.iter_mut().for_each(|x| {
            *x = *x + stlen + offset;
        });

        self.dinfo.iter_mut().for_each(|(_, y)| {
            y.0 = y.0 + stlen + offset;
            y.1 = y.1 + stlen + offset;
        });
    }
}

fn range_in(r1: DisjRange, r2: DisjRange) -> bool {
    (r1.0 >= r2.0) && (r1.1 <= r2.1)
}

fn is_overlap(r1: DisjRange, r2: DisjRange) -> bool {
    (r1.0 <= r2.1) && (r2.0 <= r1.1)
}

#[allow(dead_code)]
pub fn print_disj(snode: &Snode, disj_info: Vec<Dinfo>) {
    for ins in disj_info.iter().map(|(_, x)| x).collect_vec() {
        let pinfo = &snode.wrapper.info.pos_info;
        if range_in(*ins, (pinfo.cstart, pinfo.cend)) {
            // eprintln!("DD - {:?}", snode);
        } else {
            // eprintln!("nnmatch - {}:{:?}", snode.getstring(), (pinfo.cstart, pinfo.cend));
        }
    }
    snode
        .children
        .iter()
        .for_each(|x| print_disj(x, disj_info.clone()));
}

fn reverse_disjs(node: &mut Snode) {
    if node.is_disj {
        node.children.reverse();
    }
    node.children.iter_mut().for_each(|x| reverse_disjs(x));
}

fn clean_mods(
    m: &String,
    mut dots: Vec<usize>,
) -> (String, Vec<(usize, usize)>, Vec<(usize, usize)>, Vec<usize>) {
    let lines = m.lines().collect_vec();
    let mut ret = String::new();
    let mut minuses = vec![];
    let mut pluses = vec![];
    let mut totlen = 0;
    for line in lines {
        if line.starts_with(MINUS_MOD_START) && line.ends_with(MINUS_MOD_END) {
            minuses.push((
                ret.len(),
                ret.len() + line.len() - MINUS_MOD_START.len() - MINUS_MOD_END.len(),
            ));
            ret.push_str(&line[MINUS_MOD_START.len()..line.len() - MINUS_MOD_END.len()]);

            // Since we are removing the mod comments, we must adjuct the indices
            // of the dots which were collected by the parser before the adjustment
            dots.iter_mut()
                .filter(|x| **x > totlen)
                .for_each(|x| *x = *x - MINUS_MOD_START.len() - MINUS_MOD_END.len());
        } else if line.starts_with(PLUS_MOD_START) && line.ends_with(PLUS_MOD_END) {
            pluses.push((
                ret.len(),
                ret.len() + line.len() - MINUS_MOD_START.len() - MINUS_MOD_END.len(),
            ));
            ret.push_str(&line[PLUS_MOD_START.len()..line.len() - PLUS_MOD_END.len()]);

            // Since we are removing the mod comments, we must adjuct the indices
            // of the dots which were collected by the parser before the adjustment
            dots.iter_mut()
                .filter(|x| **x > totlen)
                .for_each(|x| *x = *x - PLUS_MOD_START.len() - PLUS_MOD_END.len());
        } else {
            ret.push_str(line);
        }
        totlen += line.len() + 1;
        ret.push('\n');
    }
    return (ret, minuses, pluses, dots);
}

// In the very first call, a single DisjBranch is present. It has a
// Vec<DisjElem> present inside it which is passed to this funcion
pub fn expand_disj((_bbno, node): &mut DisjBranch, dno: &mut usize) -> Vec<(String, OInfo)> {
    node.iter_mut()
        .fold(vec![(String::new(), OInfo::empty())], |acc, node| {
            let mut new = vec![];

            let new_bs = if node.is_mod() {
                let (m, dots) = node.as_mod();
                // if !dots.is_empty() {
                // eprintln!("dots {:?}", dots);
                // }
                let (t1, minuses, pluses, dots) = clean_mods(&m, dots);
                let s = t1;
                let oinfo = OInfo::new(vec![], minuses, pluses, dots);

                vec![(s, oinfo)]
            } else {
                split_disj(node, dno)
            };

            for (st, oinfo) in acc {
                for (new_b, new_oinfo) in new_bs.iter() {
                    let mut new_oinfo = new_oinfo.clone();

                    let offset = if st.is_empty() { 0 } else { 1 };

                    new_oinfo.update_all(st.len(), offset);

                    let (mut sti, mut oinfo) = (st.clone(), oinfo.clone());

                    if !sti.is_empty() {
                        sti.push('\n');
                    }

                    sti.push_str(new_b);
                    oinfo.dinfo.extend(new_oinfo.dinfo);
                    oinfo.dots.extend(new_oinfo.dots);
                    oinfo.minuses.extend(new_oinfo.minuses);
                    oinfo.pluses.extend(new_oinfo.pluses);

                    new.push((sti, oinfo));
                }
            }
            return new;
        })
}

fn split_disj(node: &mut DisjElem, dno: &mut usize) -> Vec<(String, OInfo)> {
    assert!(node.is_disj());
    let adno = *dno;
    *dno = *dno + 1;

    node.set_dno(adno);

    let branches = node.as_disj_mut();
    branches
        .iter_mut()
        .enumerate()
        .flat_map(|(bno, mut branch)| {
            let mut bs: Vec<(String, OInfo)> = expand_disj(&mut branch, dno);
            bs.iter_mut().for_each(|(st, oinfo)| {
                // *st =
                // The indices srart from s because when comparing with nodes
                // the indices in the AST always begin from the non-white characters
                // Is that racism?
                let s = st.trim_end().len() - st.trim().len();
                if st.trim().len() != 0 {
                    // Using trim_end because spaces are attached before
                    debugcocci!("Found disjunction, expanding: {} - {:?}", st, st.trim_end().len());
                    oinfo.add_dinfo(((adno, bno), (s, s + st.trim_end().len())));
                } else {
                    oinfo.add_dinfo(((adno, bno), (s, s)));
                }
            });
            return bs;
        })
        .collect_vec()
}

fn extract_disj_mut(node: &mut Snode, id: usize) -> Option<&mut Snode> {
    for child in &mut node.children {
        if child.is_disj && child.wrapper.disj_id.unwrap() == id {
            return Some(child);
        } else {
            match extract_disj_mut(child, id) {
                Some(child) => return Some(child),
                None => continue,
            }
        }
    }
    None
}

fn extract_braches(node: &Snode, info: DisjRange) -> Vec<Snode> {
    //this node is one of the possible branches
    let mut ret = vec![];
    for child in &node.children {
        let pos = child.wrapper.info.get_pos_tuple();
        if !is_overlap(info, pos) {
            continue;
        }
        // There is overlap
        // disj maybe fully inside this node
        // or exactly or the node may be fully
        // inside disj. This is because it is assumed
        // that partial overlap cannot happen in case
        // of a disjunction and any node.
        //
        // For example, let (a = 23; let) s= 20;
        //             |-----------|
        //                  |-----------|
        // is not allowed
        // but
        // let (a = 23;) let s = 10;
        // |-----------|
        //     |-------|
        // is valid
        if info == pos {
            //exact
            ret = vec![child.clone()];
            break;
        } else if range_in(info, pos) {
            // inside
            ret = extract_braches(child, info);
            break;
        } else {
            // other inside
            ret.push(child.clone());
        }
    }
    return ret;
}

fn merge_snodes(
    (mut main_node, mut maininfo): (Snode, Vec<((usize, Vec<usize>), DisjRange)>),
    (tmd, tmi): (Snode, Vec<Dinfo>),
) -> (Snode, Vec<((usize, Vec<usize>), DisjRange)>) {
    // Vec<Dinfo> of mainnode must be sorted
    for (did, _inds) in maininfo.iter_mut() {
        let ((_tid, bno), tminfo) = match tmi.iter().position(|(x, _)| x.0 == did.0) {
            Some(ii) => tmi[ii],
            None => continue,
        };
        if did.1.contains(&bno) {
            // // eprintln!("skipped");
            continue;
        }

        let mdisj =
            extract_disj_mut(&mut main_node, did.0).expect("Disjunction cannot be found while merging");
        let parsed_nodes2 = extract_braches(&tmd, tminfo);
        let mut branch2 = Snode::make_fake();
        branch2.children = parsed_nodes2;
        mdisj.children.push(branch2);
        did.1.push(bno);
    }
    return (main_node, maininfo);
}

pub fn resolve_snodes(mut expanded: Vec<(Snode, Vec<Dinfo>)>) -> Snode {
    // eprintln!("INFO {:?}", expanded);
    let sort_f = |(_, range1): &Dinfo, (_, range2): &Dinfo| match is_overlap(*range1, *range2) {
        true => match range_in(*range1, *range2) {
            true => std::cmp::Ordering::Less,
            false => std::cmp::Ordering::Greater,
        },
        false => {
            if range1.1 < range2.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    };

    expanded.sort_by(|x, y| match y.1.len() > x.1.len() {
        true => Ordering::Greater,
        false => Ordering::Less,
    });

    expanded.get_mut(0).unwrap().1.sort_by(sort_f);

    let (mut main_node, maininfo) = expanded.remove(0);

    for (did, inds) in &maininfo {
        fn to_disj(node: &mut Snode, (did, info): (DisjId, DisjRange)) {
            let is_empty_branch = info.0 == info.1;

            //converts the node snode into
            let (mut start, mut end) = (None, None);

            for (ind, child) in node.children.iter_mut().enumerate() {
                let pos = child.wrapper.info.get_pos_tuple();

                // eprintln!(
                //     "{}:{:?} - {:?}    {:?}",
                //     child.getstring(),
                //     child.kinds(),
                //     pos,
                //     info
                // );
                if !is_overlap(info, pos) {
                    continue;
                }
                // There is overlap
                // disj maybe fully inside this node
                // or node may be fully inside the disj
                if info == pos {
                    //exact
                    (start, end) = (Some(ind), Some(ind + 1));
                    break;
                } else if range_in(info, pos) {
                    // inside
                    if child.children.is_empty() && is_empty_branch {
                        start = Some(ind);
                        end = Some(ind);
                        break;
                    } else {
                        to_disj(child, (did, info));
                        return;
                    }
                } else {
                    // assert!(is_empty_branch);
                    if is_empty_branch {
                        let mut disj = Snode::make_disj(vec![]);
                        disj.wrapper.disj_id = Some(did.0);

                        node.children.insert(ind, disj);
                        return;
                    } else {
                        if start.is_none() && info.0 == pos.0 {
                            start = Some(ind)
                        } else if info.1 == pos.1 {
                            // ind + 1 because we want this included too
                            end = Some(ind + 1)
                        } else {
                            continue;
                        }
                    }
                }
            }
            // eprintln!("{:?}, {:?}", start, end);

            if start.is_none() || end.is_none() {
                assert!(is_empty_branch);
                if node.children.len() == 0 {
                    start = Some(0);
                    end = Some(0);
                } else if node.children.len() == 1 {
                    let pos = node.get_pos();
                    if info.0 < pos.0 {
                        start = Some(0);
                        end = Some(0);
                    } else {
                        start = Some(1);
                        end = Some(1);
                    }
                }
                for ind in 1..node.children.len() {
                    let child = &node.children[ind];
                    let p1 = node.children.get(ind - 1).unwrap().get_pos();
                    let p2 = child.get_pos();

                    if info.0 > p1.1 && info.0 < p2.0 {
                        // There should always be atleast a \n even for an empty disjunction
                        start = Some(ind);
                        end = Some(ind);
                    }
                }
            }

            let new_pos = (
                node.children[start.unwrap()].wrapper.info.pos_info.cstart,
                node.children[end.unwrap() - 1].wrapper.info.pos_info.cend,
            );
            let disj_children = node.children.drain(start.unwrap()..end.unwrap()).collect_vec();
            let mut disj_branch = Snode::make_fake();
            disj_branch.wrapper.info.pos_info =
                PositionInfo::new(0, 0, 0, 0, 0, new_pos.0, new_pos.1, new_pos.0);
            disj_branch.children = disj_children;

            let mut disj = Snode::make_disj(vec![disj_branch]);
            disj.wrapper.disj_id = Some(did.0);
            // This is only to help this function, because after this the disjunction
            // may have more branches with different position infos
            disj.wrapper.info.pos_info = PositionInfo::new(0, 0, 0, 0, 0, new_pos.0, new_pos.1, new_pos.0);

            node.children.insert(start.unwrap(), disj);
        }
        // main_node.print_tree_kinds();
        to_disj(&mut main_node, (*did, *inds));
        // main_node.print_tree_kinds();
    }

    let maininfo = maininfo
        .into_iter()
        .map(|((x, y), z)| ((x, vec![y]), z))
        .collect_vec();
    // eprintln!("Mainnode - {}", main_node.getstring());
    let mut m = (main_node, maininfo);
    // main node is now well formed.

    for (ex_branch, ex_info) in expanded {
        // eprintln!(
        //     "MUT - {}:{:?}\nMMM - {}:{:?}",
        //     m.0.getstring(),
        //     m.1,
        //     ex_branch.getstring(),
        //     ex_info
        // );
        m = merge_snodes(m, (ex_branch, ex_info));
    }
    // eprintln!("FIlneal - {}", m.0.getstring());

    // m.0.print_tree_kinds();
    reverse_disjs(&mut m.0);
    m.0
}

// pub fn process_disjs(mut branch: DisjBranch) -> Snode {
//     let bs = expand_disj(&mut branch, &mut 0);
//     // eprintln!("bs - {:#?}", bs);
//     let expanded = bs
//         .into_iter()
//         .map(|(b, inf, mod_inds)| {
//             let snode = wrap_root(&b)
//                 .expect("[INTERNAL ERROR] Malformed Semantic patch during disjunction expansion");
//             // let mut make_token_minus = |snode: &mut Snode| {
//             //     if mod_inds.contains(&snode.get_pos()) {
//             //         snode.wrapper.setmodkind(Modkind::Minus);
//             //     }
//             // };
//             // worktree(&mut snode, &mut make_token_minus);
//             (snode, inf)
//         })
//         .collect_vec();
//     // eprintln!("{:#?}", expanded);
//     let snode = resolve_snodes(expanded);
//     return snode;
// }
