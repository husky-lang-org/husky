use crate::*;
use salsa::DebugWithDb;

#[salsa::db(Jar)]
pub struct DB;

#[test]
fn word_debug_works() {
    let db = DB::default();
    let db = &*db;
    let haha = BaseCoword::from_ref("haha", db);
    expect_test::expect![[r#"
        Coword(
            "haha",
        )
    "#]]
    .assert_debug_eq(&haha);
}

#[test]
fn ident_debug_works() {
    let db = DB::default();
    let db = &*db;
    let haha = Ident::from_ref(db, "haha").unwrap();
    expect_test::expect![[r#"
        `haha`
    "#]]
    .assert_debug_eq(&haha);
}
