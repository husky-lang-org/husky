//! Test that a `tracked` fn on a `salsa::input`
//! compiles and executes successfully.

use expect_test::expect;
use husky_salsa_log_utils::HasLogger;
use salsa::Db;
use test_log::test;

#[salsa::jar]
struct Jar(
    MyInput,
    MyTracked,
    maybe_specified,
    read_maybe_specified,
    create_tracked,
    final_result,
);

#[salsa::input(db = Db)]
struct MyInput {
    field: u32,
}

#[salsa::tracked(db = Db)]
struct MyTracked {
    input: MyInput,
}

/// If the input is in the range 0..10, this is specified to return 10.
/// Otherwise, the default occurs, and it returns the input.
#[salsa::tracked(specify)]
fn maybe_specified(db: &Db, tracked: MyTracked) -> u32 {
    db.push_log(format!("maybe_specified({:?})", tracked));
    tracked.input(db).field(db)
}

/// Reads maybe-specified and multiplies it by 10.
/// This is here to show whether we can detect when `maybe_specified` has changed
/// and control down-stream work accordingly.
#[salsa::tracked]
fn read_maybe_specified(db: &Db, tracked: MyTracked) -> u32 {
    db.push_log(format!("read_maybe_specified({:?})", tracked));
    maybe_specified(db, tracked) * 10
}

/// Create a tracked value and *maybe* specify a value for
/// `maybe_specified`
#[salsa::tracked(jar = Jar)]
fn create_tracked(db: &Db, input: MyInput) -> MyTracked {
    db.push_log(format!("create_tracked({:?})", input));
    let tracked = MyTracked::new(db, input);
    if input.field(db) < 10 {
        maybe_specified::specify(db, tracked, 10);
    }
    tracked
}

#[salsa::tracked]
fn final_result(db: &Db, input: MyInput) -> u32 {
    db.push_log(format!("final_result({:?})", input));
    let tracked = create_tracked(db, input);
    read_maybe_specified(db, tracked)
}

#[salsa::db(Jar)]
#[derive(Default)]
struct Database;

#[test]
fn test_run_0() {
    let mut db = Database::default();
    let db = &mut *db;

    let input = MyInput::new(&db, 0, salsa::Durability::LOW);
    assert_eq!(final_result(&db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "create_tracked(MyInput(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
        ]"#]]);
}

#[test]
fn test_run_5() {
    let mut db = Database::default();

    let input = MyInput::new(&db, 5, salsa::Durability::LOW);
    assert_eq!(final_result(&db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "create_tracked(MyInput(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
        ]"#]]);
}

#[test]
fn test_run_10() {
    let mut db = Database::default();

    let input = MyInput::new(&db, 10, salsa::Durability::LOW);
    assert_eq!(final_result(&db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "create_tracked(MyInput(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
            "maybe_specified(MyTracked(Id { value: 1 }))",
        ]"#]]);
}

#[test]
fn test_run_20() {
    let mut db = Database::default();

    let input = MyInput::new(&db, 20, salsa::Durability::LOW);
    assert_eq!(final_result(&db, input), 200);
    db.assert_logs(expect![[r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "create_tracked(MyInput(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
            "maybe_specified(MyTracked(Id { value: 1 }))",
        ]"#]]);
}

#[test]
fn test_run_0_then_5_then_20() {
    let mut db = Database::default();

    // Set input to 0:
    //
    // * `create_tracked` specifies `10` for `maybe_specified`
    // * final resuilt of `100` is derived by executing `read_maybe_specified`
    let input = MyInput::new(&db, 0, salsa::Durability::LOW);
    assert_eq!(final_result(&db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "create_tracked(MyInput(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
        ]"#]]);

    // Set input to 5:
    //
    // * `create_tracked` does re-execute, but specifies same value for `maybe_specified` as before
    // * `read_maybe_specified` does not re-execute (its input has not changed)
    input.set_field(salsa::Durability::LOW, &mut db).to(5);
    assert_eq!(final_result(&db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "create_tracked(MyInput(Id { value: 1 }))",
        ]"#]]);

    // Set input to 20:
    //
    // * `create_tracked` re-executes but does not specify any value
    // * `read_maybe_specified` is invoked and it calls `maybe_specified`, which now executes
    //   (its value has not been specified)
    input.set_field(salsa::Durability::LOW, &mut db).to(20);
    assert_eq!(final_result(&db, input), 200);
    db.assert_logs(expect![[r#"
        [
            "create_tracked(MyInput(Id { value: 1 }))",
            "maybe_specified(MyTracked(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
            "final_result(MyInput(Id { value: 1 }))",
        ]"#]]);
}

#[test]
fn test_run_0_then_5_then_10_then_20() {
    let mut db = Database::default();

    // Set input to 0:
    //
    // * `create_tracked` specifies `10` for `maybe_specified`
    // * final resuilt of `100` is derived by executing `read_maybe_specified`
    let input = MyInput::new(&db, 0, salsa::Durability::LOW);
    assert_eq!(final_result(&db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "create_tracked(MyInput(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
        ]"#]]);

    // Set input to 5:
    //
    // * `create_tracked` does re-execute, but specifies same value for `maybe_specified` as before
    // * `read_maybe_specified` does not re-execute (its input has not changed)
    input.set_field(salsa::Durability::LOW, &mut db).to(5);
    assert_eq!(final_result(&db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "create_tracked(MyInput(Id { value: 1 }))",
        ]"#]]);

    // Set input to 10:
    //
    // * `create_tracked` does re-execute and specifies no value for `maybe_specified`
    // * `maybe_specified_value` returns 10; this is the same value as was specified.
    // * `read_maybe_specified` therefore does NOT need to execute.
    input.set_field(salsa::Durability::LOW, &mut db).to(10);
    assert_eq!(final_result(&db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "create_tracked(MyInput(Id { value: 1 }))",
            "maybe_specified(MyTracked(Id { value: 1 }))",
        ]"#]]);

    // Set input to 20:
    //
    // * Everything re-executes to get new result (200).
    input.set_field(salsa::Durability::LOW, &mut db).to(20);
    assert_eq!(final_result(&db, input), 200);
    db.assert_logs(expect![[r#"
        [
            "create_tracked(MyInput(Id { value: 1 }))",
            "maybe_specified(MyTracked(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
            "final_result(MyInput(Id { value: 1 }))",
        ]"#]]);
}

#[test]
fn test_run_5_then_20() {
    let mut db = Database::default();
    let db = &mut *db;

    let input = MyInput::new(db, 5, salsa::Durability::LOW);
    assert_eq!(final_result(db, input), 100);
    db.assert_logs(expect![[r#"
        [
            "final_result(MyInput(Id { value: 1 }))",
            "create_tracked(MyInput(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
        ]"#]]);

    input.set_field(salsa::Durability::LOW, db).to(20);
    assert_eq!(final_result(db, input), 200);
    db.assert_logs(expect![[r#"
        [
            "create_tracked(MyInput(Id { value: 1 }))",
            "maybe_specified(MyTracked(Id { value: 1 }))",
            "read_maybe_specified(MyTracked(Id { value: 1 }))",
            "final_result(MyInput(Id { value: 1 }))",
        ]"#]]);
}
