#![allow(dead_code)]
use super::transformationtest::TransformTest;

static DISJTESTS: TransformTest = TransformTest {
    prefix: "./src/tests/disjunctions/",
};

#[test]
pub fn test1() {
    assert!(DISJTESTS.testtransformation("test1.cocci", "test1.rs", "expected1.rs"))
}

#[test]
pub fn test2() {
    assert!(DISJTESTS.testtransformation("test2.cocci", "test2.rs", "expected2.rs"))
}

#[test]
pub fn test3() {
    assert!(DISJTESTS.testtransformation("test3.cocci", "test3.rs", "expected3.rs"))
}

#[test]
pub fn test4() {
    assert!(DISJTESTS.testtransformation("test4.cocci", "test4.rs", "expected4.rs"))
}

#[test]
pub fn test5() {
    assert!(DISJTESTS.testtransformation("test5.cocci", "test5.rs", "expected5.rs"))
}

#[test]
pub fn test6() {
    assert!(DISJTESTS.testtransformation("test6.cocci", "test6.rs", "expected6.rs"))
}

#[test]
pub fn test7() {
    assert!(DISJTESTS.testtransformation("test7.cocci", "test7.rs", "expected7.rs"))
}

#[test]
pub fn test8() {
    assert!(DISJTESTS.testtransformation("test8.cocci", "test8.rs", "expected8.rs"))
}

#[test]
pub fn test9() {
    assert!(DISJTESTS.testtransformation("test9.cocci", "test9.rs", "expected9.rs"))
}

#[test]
pub fn test10() {
    assert!(DISJTESTS.testtransformation("test10.cocci", "test10.rs", "expected10.rs"))
}
