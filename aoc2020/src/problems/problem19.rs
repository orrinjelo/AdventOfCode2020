use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::util::RetType;
use std::collections::HashMap;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rule {
    Pattern(u32,u32),
    Char(char),
    Split(Box<Rule>, Box<Rule>)
}

fn parse_rule_string(input: String) -> (u32, Rule) {
    let rule: Rule;
    let parts: Vec<&str> = input.split(':').collect();
    let rule_no = parts[0].parse::<u32>().unwrap();
    let rules: Vec<&str> = parts[1].split('|').collect();
    if rules.len() == 1 {
        if rules[0].contains("\"") {
            rule = Rule::Char(rules[0].trim().chars().nth(1).unwrap());
        } else {
            let split: Vec<&str> = rules[0].trim().split(' ').collect();
            rule = Rule::Pattern(split[0].parse::<u32>().unwrap(), split[1].parse::<u32>().unwrap());
        }
    } else {
        let split1: Vec<&str> = rules[0].trim().split(' ').collect();
        let split2: Vec<&str> = rules[1].trim().split(' ').collect();
        let left_rule = Box::new(Rule::Pattern(split1[0].parse::<u32>().unwrap(), split1[1].parse::<u32>().unwrap()));
        let right_rule = Box::new(Rule::Pattern(split2[0].parse::<u32>().unwrap(), split2[1].parse::<u32>().unwrap()));
        rule = Rule::Split(left_rule, right_rule);
    }

    (rule_no, rule)
}

fn make_rulebook(input: Vec<String>) -> (HashMap<u32, Rule>, Vec<String>) {
    let mut rule_bool: bool = true;
    let mut rule_map: HashMap<u32, Rule> = HashMap::new();
    let mut entries: Vec<String> = Vec::new();
    for line in input {
        if line == "".to_string() {
            rule_bool = false;
        }
        if rule_bool {
            let parsed_rule = parse_rule_string(line);
            rule_map.insert(parsed_rule.0, parsed_rule.1);
        } else {
            entries.push(line);
        }
    }
    (rule_map, entries)
}

fn make_dictionary(rulebook: HashMap<u32, Rule>) -> Vec<String> {

    Vec::new()
}

fn expand_word(rulebook: &HashMap<u32, Rule>, current_rule: Rule) -> Vec<Vec<char>> {

    match current_rule {
        Rule::Char(x) => {  // Easiest case: end case.  'a'
            return vec![vec![x]];
        },
        Rule::Pattern(x,y) => {  // 2, 3
            let left = expand_word(rulebook, rulebook.get(&x).unwrap().clone());
            let right = expand_word(rulebook, rulebook.get(&y).unwrap().clone());

            let mut ret: Vec<Vec<char>> = Vec::new();

            for i in 0..left.len() {
                for j in 0..right.len() {
                    let mut temp: Vec<char> = Vec::new();
                    temp.extend(&left[i]);
                    temp.extend(&right[j]);
                    ret.push(temp)
                }
            }
            return right;
        },
        Rule::Split(x,y) => { // 2, 3 | 3, 2
            let left = *x;
            let right = *y;

            let left_res = expand_word(rulebook, left);
            let right_res = expand_word(rulebook, right);

            let mut ret: Vec<Vec<char>> = Vec::new();

            for i in 0..left_res.len() {
                for j in 0..right_res.len() {
                    let mut temp: Vec<char> = Vec::new();
                    temp.extend(&left_res[i]);
                    temp.extend(&right_res[j]);
                    ret.push(temp)
                }
            }
            return ret;
        }
    }

}

/// Problem #19, part 1
pub fn problem_191(input: Vec<String>) -> RetType {
    RetType::U32(0u32)
}

/// Problem #19, part 2
pub fn problem_192(input: Vec<String>) -> RetType {
    RetType::U32(0u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        match env_logger::try_init() {
            Ok(_) => {
                info!("Initializing logging...");
            },
            Err(_) => {

            }
        }
    }

    #[test]
    fn test_parsing_rules() {
        init();

        let rule0 = "0: 4 1".to_string();
        let rule1 = "1: 2 3 | 3 2".to_string();
        let rule2 = "2: 4 4 | 5 5".to_string();
        let rule3 = "3: 4 5 | 5 4".to_string();
        let rule4 = "4: \"a\"".to_string();
        let rule5 = "5: \"b\"".to_string();

        assert_eq!(parse_rule_string(rule0), (0, Rule::Pattern(4,1)));
        assert_eq!(parse_rule_string(rule1), (1, Rule::Split(Box::new(Rule::Pattern(2,3)),Box::new(Rule::Pattern(3,2)))));
        assert_eq!(parse_rule_string(rule2), (2, Rule::Split(Box::new(Rule::Pattern(4,4)),Box::new(Rule::Pattern(5,5)))));
        assert_eq!(parse_rule_string(rule3), (3, Rule::Split(Box::new(Rule::Pattern(4,5)),Box::new(Rule::Pattern(5,4)))));
        assert_eq!(parse_rule_string(rule4), (4, Rule::Char('a')));
        assert_eq!(parse_rule_string(rule5), (5, Rule::Char('b')));
    }

    #[test]
    fn test_parsing_more_rules() {
        init();

        let input = vec![
            "0: 4 1".to_string(),
            "1: 2 3 | 3 2".to_string(),
            "2: 4 4 | 5 5".to_string(),
            "3: 4 5 | 5 4".to_string(),
            "4: \"a\"".to_string(),
            "5: \"b\"".to_string(),
        ];

        let (rulebook, _entries) = make_rulebook(input);

        let words = expand_word(&rulebook, rulebook.get(&0).unwrap().clone());

        debug!("Words: {:?}", words);
    }
}
