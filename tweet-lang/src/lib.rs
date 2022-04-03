mod ast;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub lang);

#[allow(unused_must_use)]
#[test]
fn lang() {
    // let massert = |a:&str,b:&str|{
    //     println!("Comparing: \n{a} \n{b}");
    //     let parser = lang::InstructionsParser::new();
    //     assert_eq!(dbg!(parser.parse(a)),dbg!(parser.parse(b)));
    // };

    use crate::ast::Action;
    let instructions = lang::InstructionsParser::new()
        .parse("punch left. walk right. Do jump left 3 times.")
        .unwrap();
    let actions: Vec<Action> = instructions.into();
    dbg!(actions);

    // dbg!(lang::NumParser::new().parse("12456543"));
    // dbg!(lang::ActionParser::new().parse("walk left"));
    // dbg!(lang::ActionParser::new().parse("jump left"));
    // dbg!(lang::ActionParser::new().parse("punch left"));
    // dbg!(lang::InstructionsParser::new().parse("walk left"));
    // dbg!(lang::InstructionsParser::new().parse("Do walk left 12456543 times"));
    // dbg!(lang::InstructionsParser::new().parse("Do walk left 12456543 times."));

    // massert("Do walk left 12456543 times","Do walk left 12456543 times.");
    // massert("Do walk left 12456543 times","Do  walk left 12456543 times.");
    // massert("Do walk left 12456543 times","Do walk  left 12456543 times.");
    // massert("Do walk left 12456543 times","Do walk left  12456543 times.");
    // massert("Do walk left 12456543 times","Do walk left 12456543 times.");
}
