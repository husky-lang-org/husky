use salsa::DebugWithDb;

use crate::*;

#[test]
fn toolchain_debug_works() {
    let db = DB::default();
    let toolchain = db.dev_toolchain();
    expect_test::expect![[r#"
        Toolchain {
            data: Local {
                library_path: DiffPath {
                    data: DiffPathBuf(
                        "../../../library",
                    ),
                },
            },
        }
    "#]]
    .assert_debug_eq(&toolchain.debug(&db));
}
