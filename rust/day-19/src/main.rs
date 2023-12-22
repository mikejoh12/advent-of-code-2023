use regex::{self, Regex};
use std::{collections::HashMap, fs};
use std::cmp::Ordering;

enum Action {
    Accept,
    Reject,
    Forward(String),
}

enum Rule {
    Evaluate(EvaluationData),
    TakeAction(Action),
}

struct EvaluationData {
    category: String,
    comparison: Ordering,
    value: u32,
    true_action: Action,
}

struct Workflows(HashMap<String, Vec<Rule>>);

impl Workflows {
    fn new(data: &str) -> Workflows {
        let mut workflows: Workflows = Workflows(HashMap::new());

        for workflow in data.split("\n") {
            let mut parts = workflow.split("{");
            let id = parts.next().unwrap();
            let rules_str = parts.next().unwrap().trim_end_matches("}");
            let mut rules: Vec<Rule> = vec![];

            for rule_str in rules_str.split(",") {
                if rule_str == "A" {
                    rules.push(Rule::TakeAction(Action::Accept));
                    continue;
                } else if rule_str == "R" {
                    rules.push(Rule::TakeAction(Action::Reject));
                    continue;
                } else if !rule_str.contains(":") {
                    rules.push(Rule::TakeAction(Action::Forward(rule_str.to_string())));
                    continue;
                }

                let mut eval_parts = rule_str.split(":");
                let expr_str = eval_parts.next().unwrap();
                let eval_dest_str = eval_parts.next().unwrap();

                let comparison = match (
                    expr_str.contains(">"),
                    expr_str.contains("<"),
                    expr_str.contains("="),
                ) {
                    (true, _, _) => Ordering::Greater,
                    (_, true, _) => Ordering::Less,
                    (_, _, true) => Ordering::Equal,
                    _ => panic!("Error parsing expression"),
                };

                let true_action = match eval_dest_str {
                    "A" => Action::Accept,
                    "R" => Action::Reject,
                    _ => Action::Forward(eval_dest_str.to_string()),
                };

                let mut expr_values = expr_str.split(|c| c == '>' || c == '<' || c == '=');
                let category = expr_values.next().unwrap().to_string();
                let value: u32 = expr_values.next().unwrap().parse().unwrap();

                rules.push(Rule::Evaluate(EvaluationData {
                    category,
                    comparison,
                    value,
                    true_action,
                }))
            }
            workflows.0.insert(id.to_string(), rules);
        }
        workflows
    }
}

struct Parts(Vec<Part>);

impl Parts {
    fn new(data: &str) -> Parts {
        let re = Regex::new(r"\d+").unwrap();
        let mut parts: Parts = Parts(vec![]);

        for ratings in data.split("\n") {
            let nums: Vec<u32> = re
                .find_iter(ratings)
                .filter_map(|digits| digits.as_str().parse().ok())
                .collect();
            parts.0.push(Part {
                x: nums[0],
                m: nums[1],
                a: nums[2],
                s: nums[3],
            });
        }

        parts
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn evaluate(&self, workflows: &Workflows) -> bool {
        let mut cur: String = "in".to_string();

        loop {
            for rule in workflows.0.get(&cur).unwrap() {
                match rule {
                    Rule::Evaluate(eval_data) => {
                        let comp_value = match eval_data.category.as_str() {
                            "x" => self.x,
                            "m" => self.m,
                            "a" => self.a,
                            "s" => self.s,
                            _ => panic!("Invalid category"),
                        };

                        if comp_value.cmp(&eval_data.value) == eval_data.comparison {
                            match &eval_data.true_action {
                                Action::Accept => return true,
                                Action::Reject => return false,
                                Action::Forward(next_work_flow) => {
                                    cur = next_work_flow.to_string();
                                    break;
                                }
                            }
                        }
                    }
                    Rule::TakeAction(a) => match a {
                        Action::Accept => return true,
                        Action::Reject => return false,
                        Action::Forward(next_work_flow) => {
                            cur = next_work_flow.to_string();
                            break;
                        }
                    },
                }
            }
        }
    }

    fn total_rating(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

fn main() {
    let sections: Vec<String> = fs::read_to_string("input.txt")
        .expect("File should open")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect();
    let workflows: Workflows = Workflows::new(&sections[0]);
    let parts = Parts::new(&sections[1]);

    let mut accepted_ratings = 0;
    for part in parts.0 {
        if part.evaluate(&workflows) {
            accepted_ratings += part.total_rating();
        }
    }

    println!("Part 1: {}", accepted_ratings);
}
