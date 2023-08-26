use std::collections::HashMap;
use std::fs;

pub fn solve() -> (i32, i32) {
    let text = fs::read_to_string("../data/07.txt").unwrap();
    (part1(&text), part2(&text))
}

fn part1(text: &str) -> i32 {
    helper(text, "a")
}

fn part2(text: &str) -> i32 {
    let dependencies: HashMap<String, Gate> = HashMap::new();
    let mut dependencies =
        text.lines()
            .map(parser::parse_command)
            .fold(dependencies, |mut deps, b| {
                deps.entry(b.out.clone()).or_insert(b);
                deps
            });

    let first_res = compute(
        dependencies.get("a").unwrap(),
        &dependencies,
        &mut HashMap::new(),
    );

    let b = dependencies.get("b").unwrap();
    dependencies.insert(
        "b".to_string(),
        Gate {
            inp: GateInput::Straight(Input::Value(first_res)),
            out: b.out.clone(),
        },
    );

    compute(
        dependencies.get("a").unwrap(),
        &dependencies,
        &mut HashMap::new(),
    ) as i32
}

fn helper(text: &str, start: &str) -> i32 {
    let dependencies: HashMap<String, Gate> = HashMap::new();
    let dependencies = text
        .lines()
        .map(parser::parse_command)
        .fold(dependencies, |mut deps, b| {
            deps.entry(b.out.clone()).or_insert(b);
            deps
        });
    compute(
        dependencies.get(start).unwrap(),
        &dependencies,
        &mut HashMap::new(),
    ) as i32
}

fn compute(gate: &Gate, map: &HashMap<String, Gate>, computed: &mut HashMap<String, u16>) -> u16 {
    if let Some(val) = computed.get(&gate.out) {
        return *val;
    }
    match gate.inp {
        GateInput::Straight(ref inp) => {
            let res = compute_input(inp, map, computed);
            computed.insert(gate.out.clone(), res);
            res
        }
        GateInput::Not(ref inp) => {
            let res = !compute_input(inp, map, computed);
            computed.insert(gate.out.clone(), res);
            res
        }
        GateInput::LShift(ref x, ref y) => {
            let res = compute_input(x, map, computed) << y;
            computed.insert(gate.out.clone(), res);
            res
        }
        GateInput::RShift(ref x, ref y) => {
            let res = compute_input(x, map, computed) >> y;
            computed.insert(gate.out.clone(), res);
            res
        }
        GateInput::And(ref x, ref y) => {
            let res = compute_input(x, map, computed) & compute_input(&y, map, computed);
            computed.insert(gate.out.clone(), res);
            res
        }
        GateInput::Or(ref x, ref y) => {
            let res = compute_input(&x, map, computed) | compute_input(&y, map, computed);
            computed.insert(gate.out.clone(), res);
            res
        }
    }
}

fn compute_input(
    inp: &Input,
    map: &HashMap<String, Gate>,
    computed: &mut HashMap<String, u16>,
) -> u16 {
    match inp {
        Input::Value(val) => *val,
        Input::Ref(ref key) => compute(map.get(key).unwrap(), map, computed),
    }
}

#[derive(Debug, PartialEq)]
pub enum Input {
    Value(u16),
    Ref(String),
}

#[derive(Debug, PartialEq)]
pub struct Gate {
    inp: GateInput,
    out: String,
}

#[derive(Debug, PartialEq)]
pub enum GateInput {
    Straight(Input),
    Not(Input),
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, u16),
    RShift(Input, u16),
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &'static str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn compute_straight() {
        assert_eq!(
            compute(
                &Gate {
                    inp: GateInput::Straight(Input::Value(123)),
                    out: "x".to_string()
                },
                &HashMap::new(),
                &mut HashMap::new(),
            ),
            123
        );
    }

    #[test]
    fn compute_not() {
        assert_eq!(
            compute(
                &Gate {
                    inp: GateInput::Not(Input::Value(123)),
                    out: "x".to_string()
                },
                &HashMap::new(),
                &mut HashMap::new(),
            ),
            65412
        );
    }

    #[test]
    fn compute_and() {
        assert_eq!(
            compute(
                &Gate {
                    inp: GateInput::And(Input::Value(123), Input::Value(456)),
                    out: "x".to_string()
                },
                &HashMap::new(),
                &mut HashMap::new(),
            ),
            72
        );
    }

    #[test]
    fn compute_or() {
        assert_eq!(
            compute(
                &Gate {
                    inp: GateInput::Or(Input::Value(123), Input::Value(456)),
                    out: "x".to_string()
                },
                &HashMap::new(),
                &mut HashMap::new(),
            ),
            507
        );
    }

    #[test]
    fn compute_lshift() {
        assert_eq!(
            compute(
                &Gate {
                    inp: GateInput::LShift(Input::Value(123), 2),
                    out: "x".to_string()
                },
                &HashMap::new(),
                &mut HashMap::new(),
            ),
            492
        );
    }

    #[test]
    fn compute_rshift() {
        assert_eq!(
            compute(
                &Gate {
                    inp: GateInput::RShift(Input::Value(456), 2),
                    out: "x".to_string()
                },
                &HashMap::new(),
                &mut HashMap::new(),
            ),
            114
        );
    }

    #[test]
    fn full_compute() {
        assert_eq!(helper(TEST_INPUT, "g"), 114);
    }
}

mod parser {
    use super::{Gate, GateInput, Input};
    use pest::{iterators::Pair, Parser};
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "./assets/day07/grammar.pest"]
    pub struct CmdParser;

    pub fn parse_command(cmd: &str) -> Gate {
        let command = CmdParser::parse(Rule::command, cmd)
            .expect("unsuccessful parse") // unwrap the parse result
            .next()
            .unwrap();

        match command.as_rule() {
            Rule::straight => {
                let mut inner = command.into_inner();
                let inp = get_operand(inner.next().unwrap());
                let out = get_reference(inner.next().unwrap());
                Gate {
                    inp: GateInput::Straight(inp),
                    out,
                }
            }
            Rule::not => {
                let mut inner = command.into_inner();
                let inp = get_operand(inner.next().unwrap());
                let out = get_reference(inner.next().unwrap());
                Gate {
                    inp: GateInput::Not(inp),
                    out,
                }
            }
            Rule::lshift => {
                let mut inner = command.into_inner();
                let x = get_operand(inner.next().unwrap());
                let y: u16 = inner.next().unwrap().as_str().parse().unwrap();
                let out = get_reference(inner.next().unwrap());
                Gate {
                    inp: GateInput::LShift(x, y),
                    out,
                }
            }
            Rule::rshift => {
                let mut inner = command.into_inner();
                let x = get_operand(inner.next().unwrap());
                let y: u16 = inner.next().unwrap().as_str().parse().unwrap();
                let out = get_reference(inner.next().unwrap());
                Gate {
                    inp: GateInput::RShift(x, y),
                    out,
                }
            }
            Rule::and => {
                let mut inner = command.into_inner();
                let x = get_operand(inner.next().unwrap());
                let y = get_operand(inner.next().unwrap());
                let out = get_reference(inner.next().unwrap());
                Gate {
                    inp: GateInput::And(x, y),
                    out,
                }
            }
            Rule::or => {
                let mut inner = command.into_inner();
                let x = get_operand(inner.next().unwrap());
                let y = get_operand(inner.next().unwrap());
                let out = get_reference(inner.next().unwrap());
                Gate {
                    inp: GateInput::Or(x, y),
                    out,
                }
            }
            _ => unreachable!(),
        }
    }

    fn get_operand(operand: Pair<Rule>) -> Input {
        let inner = operand.into_inner().next().unwrap();
        match inner.as_rule() {
            Rule::value => Input::Value(inner.as_str().parse().unwrap()),
            Rule::reference => Input::Ref(inner.as_str().to_string()),
            _ => unreachable!(),
        }
    }

    fn get_reference(reference: Pair<Rule>) -> String {
        reference.as_str().to_string()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn straight() {
            assert_eq!(
                parse_command("123 -> x"),
                Gate {
                    inp: GateInput::Straight(Input::Value(123)),
                    out: "x".to_string()
                }
            );
        }

        #[test]
        fn not() {
            assert_eq!(
                parse_command("NOT 123 -> x"),
                Gate {
                    inp: GateInput::Not(Input::Value(123)),
                    out: "x".to_string()
                }
            );
        }

        #[test]
        fn lshift() {
            assert_eq!(
                parse_command("s LSHIFT 123 -> x"),
                Gate {
                    inp: GateInput::LShift(Input::Ref("s".to_string()), 123),
                    out: "x".to_string()
                }
            );
        }

        #[test]
        fn rshift() {
            assert_eq!(
                parse_command("123 RSHIFT 2 -> x"),
                Gate {
                    inp: GateInput::RShift(Input::Value(123), 2),
                    out: "x".to_string(),
                }
            );
        }

        #[test]
        fn and() {
            assert_eq!(
                parse_command("y AND 123 -> x"),
                Gate {
                    inp: GateInput::And(Input::Ref("y".to_string()), Input::Value(123)),
                    out: "x".to_string()
                }
            );
        }

        #[test]
        fn or() {
            assert_eq!(
                parse_command("123 OR y -> x"),
                Gate {
                    inp: GateInput::Or(Input::Value(123), Input::Ref("y".to_string())),
                    out: "x".to_string()
                }
            );
        }

        #[test]
        fn or_() {
            assert_eq!(
                parse_command("123 OR 321 -> x"),
                Gate {
                    inp: GateInput::Or(Input::Value(123), Input::Value(321)),
                    out: "x".to_string()
                }
            );
        }

        #[test]
        #[should_panic]
        fn lowercase_command() {
            parse_command("rshift 123 -> x");
        }

        #[test]
        #[should_panic]
        fn uppercase_ref() {
            parse_command("RSHIFT 123 -> X");
        }
    }
}
