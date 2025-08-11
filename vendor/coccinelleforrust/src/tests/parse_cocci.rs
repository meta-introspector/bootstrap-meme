use crate::parsing_cocci::smpl_grammar::{DisjElem, Metavar};

macro_rules! parse {
    ($patch: expr) => {{
        let lexer = crate::parsing_cocci::lexer::CocciLexer::new(&$patch);
        smpl_grammar::SPatchParser::new()
            .parse(&$patch, lexer)
            .expect("Not parsed")
    }};
}

fn assert_disj(disj1: &Vec<DisjElem>, disj2: &Vec<DisjElem>) {
    let l = disj1.len();
    assert_eq!(disj1.len(), disj2.len());
    for i in 0..l {
        assert_disj_elem(&disj1[i], &disj2[i])
    }

    fn assert_disj_elem(db1: &DisjElem, db2: &DisjElem) {
        match (&db1, &db2) {
            (DisjElem::Mod(m1), DisjElem::Mod(m2)) => assert_eq!(m1.0.trim(), m2.0.trim()),
            (DisjElem::Mod(_), DisjElem::Disj(_, _vec)) => panic!("Cannot match DisjBranch with Mod"),
            (DisjElem::Disj(_, _vec), DisjElem::Mod(_)) => panic!("Cannot match DisjBranch with Mod"),
            (DisjElem::Disj(_, vec1), DisjElem::Disj(_, vec2)) => {
                let l = vec1.len();
                assert_eq!(l, vec2.len());
                for i in 0..l {
                    assert_disj(&vec1[i].1, &vec2[i].1);
                }
            }
        }
    }
}

macro_rules! check_parsed {
    ($parsed1: expr, $parsed2: expr) => {
        assert_eq!($parsed1.0, $parsed2.0);
        assert_eq!($parsed1.1, $parsed2.1);

        for _ in 0..3 {
            assert_disj(&$parsed1.2 .1, &$parsed2.2)
        }
    };
}

#[test]
fn parse_test1() {
    use crate::{parsing_cocci::smpl_grammar::DisjElem, smpl_grammar};

    let patch = "@ rule1 @
    expression x1;
    @@
    ";

    let parsed = parse!(patch);

    println!("{:?}", parsed);
    check_parsed!(
        parsed[0],
        (
            "rule1",
            vec![("expression", vec!["x1"])],
            vec![DisjElem::Mod(("".to_string(), vec![]))],
        )
    );
}

#[test]
fn parse_test2() {
    use crate::{parsing_cocci::smpl_grammar::DisjElem, smpl_grammar};

    let patch = "@@
    @@
    e();";

    let parsed = parse!(patch);

    println!("{:?}", parsed);
    check_parsed!(
        parsed[0],
        (
            "",
            Vec::<Metavar>::new(),
            vec![DisjElem::Mod(("e();".to_string(), vec![]))],
            false,
            3
        )
    );
}

#[test]
fn parse_test3() {
    use crate::{parsing_cocci::smpl_grammar::DisjElem, smpl_grammar};

    let patch = "
    @@
    @@
    e();
    ";

    let parsed = parse!(patch);

    println!("{:?}", parsed);
    check_parsed!(
        parsed[0],
        (
            "",
            Vec::<Metavar>::new(),
            vec![DisjElem::Mod(("e();".to_string(), vec![]))],
            false,
            3
        )
    );
}

#[test]
fn parse_test4() {
    use crate::{parsing_cocci::smpl_grammar::DisjElem, smpl_grammar};

    let patch = "
    @@
    @@
    -e();
    +f;
    ";

    let parsed = parse!(patch);

    println!("{:?}", parsed);
    check_parsed!(
        parsed[0],
        (
            "",
            Vec::<Metavar>::new(),
            vec![DisjElem::Mod(("-e();\n    +f;".to_string(), vec![]))]
        )
    );
}

#[test]
fn parse_test5() {
    use crate::{parsing_cocci::smpl_grammar::DisjElem, smpl_grammar};

    let patch = "
    @@
    @@
    -e(); ... +f;
    ";

    let parsed = parse!(patch);

    println!("{:?}", parsed);
    check_parsed!(
        parsed[0],
        (
            "",
            Vec::<Metavar>::new(),
            vec![DisjElem::Mod(("-e();   +f;".to_string(), vec![6]))]
        )
    );
}
