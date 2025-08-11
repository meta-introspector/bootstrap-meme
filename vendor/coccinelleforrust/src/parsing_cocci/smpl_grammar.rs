pub type ModType = (String, Vec<usize>); // usize are the indices where there are ellipses
pub type RuleInfo<'a> = &'a str; //rulename as of now
pub type RuleType<'a> = (RuleInfo<'a>, Metavars<'a>, DisjBranch, bool, usize);
pub type Metavar<'a> = (&'a str, Vec<&'a str>);
pub type Metavars<'a> = Vec<Metavar<'a>>;
pub type DisjBranch = (Option<usize>, Vec<DisjElem>);

/// A Mod is a string of contiguous characters that dont have any disjunction in them
/// A DisjBranches element contains vector of posisble branches
/// Each branch is made up of a vector of DisjElems which mean that there can be
/// mods followed by other disjs and then more mods
/// It is enforced during parsing that no mod are next to each other
#[derive(Debug, Eq)]
pub enum DisjElem {
    Mod(ModType),
    Disj(Option<usize>, Vec<DisjBranch>),
}

impl PartialEq for DisjElem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Mod((l00, l01)), Self::Mod((r00, r01))) => l00.trim() == r00.trim() && l01 == r01,
            (Self::Disj(_, l0), Self::Disj(_, r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl<'a> DisjElem {
    pub fn is_disj(&self) -> bool {
        match self {
            DisjElem::Mod(_) => false,
            DisjElem::Disj(_, _vec) => true,
        }
    }

    pub fn is_mod(&self) -> bool {
        match self {
            DisjElem::Mod(_) => true,
            DisjElem::Disj(_, _vec) => false,
        }
    }

    pub fn set_dno(&mut self, dno: usize) {
        assert!(self.is_disj());

        match self {
            DisjElem::Mod(_) => {
                panic!("Cannot set DisjId for Mod.")
            }
            DisjElem::Disj(pdno, _) => *pdno = Some(dno),
        }
    }

    pub fn as_disj_mut(&'a mut self) -> &'a mut Vec<(Option<usize>, Vec<DisjElem>)> {
        assert!(self.is_disj());

        match self {
            DisjElem::Mod(_mods) => {
                panic!("Trying to extract disj from Mod.")
            }
            DisjElem::Disj(_, d_elems) => d_elems,
        }
    }

    pub fn as_mod(&self) -> ModType {
        assert!(self.is_mod());

        match self {
            DisjElem::Mod(mods) => {
                return mods.clone();
            }
            DisjElem::Disj(_, _vec) => panic!("should not be possible"),
        }
    }

    pub fn into_mod(self) -> ModType {
        assert!(self.is_mod());

        match self {
            DisjElem::Mod(mods) => {
                return mods;
            }
            DisjElem::Disj(_, _vec) => panic!("should not be possible"),
        }
    }
}

// impl DisjElems {
//     fn add_elem(&mut self, elem: DisjElems) {
//         match (self, elem) {
//             (DisjElems::Mod(_), DisjElems::Mod(_)) => panic!("Not possible"),
//             (DisjElems::Mod(_), DisjElems::DisjBranches(vec)) => todo!(),
//             (DisjElems::DisjBranches(vec), DisjElems::Mod(_)) => todo!(),
//             (DisjElems::DisjBranches(vec), DisjElems::DisjBranches(vec)) => todo!(),
//         }
//     }
// }

pub fn make_metavar<'a>(t: &'a str, def: Vec<&'a str>) -> Metavar<'a> {
    return (t, def);
}

pub fn add_metavar<'a>(mut mvars: Metavars<'a>, t: &'a str, def: Vec<&'a str>) -> Metavars<'a> {
    mvars.push((t, def));
    return mvars;
}
