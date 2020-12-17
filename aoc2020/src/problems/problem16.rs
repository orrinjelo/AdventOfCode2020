use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use std::collections::HashMap;
use regex::Regex;
use crate::util::RetType;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

#[derive(Clone, Debug, PartialEq)]
struct Ticket {
    // valid_ranges: HashMap<String, Vec<(u32, u32)>>,
    fields: HashMap<String, u32>,
    unassigned_fields: Vec<u32>
}

impl Ticket {
    pub fn new(u_fields: Vec<u32>) -> Ticket {
        Ticket {
            // valid_ranges:      valid_r,
            fields:            HashMap::new(),
            unassigned_fields: u_fields,
        }
    }

}

pub fn parse_valid_range(line: &String) -> (String, Vec<(u32, u32)>) {
    let re = Regex::new(r"(\w+\s?\w*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let cap = re.captures(&line).unwrap();
    let field_name = cap[1].trim().to_string();
    let lower1 = cap[2].trim().parse::<u32>().unwrap();
    let upper1 = cap[3].trim().parse::<u32>().unwrap();
    let lower2 = cap[4].trim().parse::<u32>().unwrap();
    let upper2 = cap[5].trim().parse::<u32>().unwrap();
    (field_name, vec![(lower1,upper1),(lower2,upper2)])
}

#[derive(Clone, Debug, PartialEq)]
enum TicketValidity {
    Valid,
    Invalid(u32)
}

fn parse_tickets(input: Vec<String>) -> (Vec<Ticket>, HashMap<String, Vec<(u32, u32)>>) {
    let mut tickets: Vec<Ticket> = Vec::new();
    let mut valid_ranges: HashMap<String, Vec<(u32, u32)>> = HashMap::new();

    let mut i = 0;
    let mut state = 0; // 0 = valid ranges, 1 = your ticket, 2 = nearby tickets
    'sec: loop {
        if i == input.len() {
            break 'sec;
        } else if input[i] == "".to_string() {
            state += 1;
        } else if state == 0 {
            let valid_range = parse_valid_range(&input[i]);
            valid_ranges.insert(valid_range.0, valid_range.1);
        } else if state == 1 || state == 2 {
            if input[i] == "your ticket:" || input[i] == "nearby tickets:" {
                i += 1;
            }
            let values: Vec<u32> = input[i].split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            tickets.push(Ticket::new(values));
        }
        i += 1;
    }
    (tickets, valid_ranges)
}

fn is_valid_ticket(ticket: Ticket, valid_ranges: &HashMap<String, Vec<(u32, u32)>>) -> TicketValidity {
    for field in ticket.unassigned_fields {
        let mut ok = false;
        for values in valid_ranges.values() {
            for entry in values {
                if field >= entry.0 && field <= entry.1 {ok = true;}
            }
        }
        if !ok {
            return TicketValidity::Invalid(field);
        }
    }
    return TicketValidity::Valid;
}

fn in_range(value: u32, ranges: Vec<(u32, u32)>) -> bool {
    if (value < ranges[0].0 || value > ranges[0].1) && (value < ranges[1].0 || value > ranges[1].1) {
        return false;
    }
    // debug!("Value {} is in range {:?}", value, ranges);
    return true;
}

fn vote_system(tickets: &Vec<Ticket>, rules: &HashMap<String, Vec<(u32, u32)>>) -> HashMap<String, usize> {
    let mut rules_keys: Vec<String> = Vec::new();
    let mut rules_values: Vec<Vec<(u32, u32)>> = Vec::new();

    for (key, val) in rules.iter() {
        rules_keys.push(key.clone().to_string());
        rules_values.push(val.clone());
    }

    let mut table = vec![vec![1; rules.len()]; rules.len()];

    for i in 0..rules.len() {
        for ticket in tickets {
            for j in 0..ticket.unassigned_fields.len() {
                if !in_range(ticket.unassigned_fields[j], rules_values[i].clone()) {
                    table[i][j] = 0;
                }
            }
        }
    }

    reduce(&mut table);
    debug!("Table: {:?}", table);

    let mut res: HashMap<String, usize> = HashMap::new();
    for i in 0..table.len() {
        let index = table[i].iter().position(|&x| x == 1).unwrap();
        res.insert(rules_keys[i].clone(), index);
    }

    res
}

fn reduce(m: &mut Vec<Vec<u32>>) {
    let mut current: u32 = 1;
    let mut history: Vec<u32> = Vec::new();

    'searcher:  loop {
        if current > m.len() as u32 {
            break 'searcher;
        }
        for i in 0..m.len() {
            let mut s = 0;
            let mut idx = 0;
            for j in 0..m[i].len() {
                s += m[i][j];
                if m[i][j] == 1 && !history.contains(&(j as u32)) {
                    idx = j;
                }
            }
            if s == current {
                history.push(idx as u32);
                for j in 0..m[i].len() {
                    if j != idx && m[i][j] == 1 {
                        m[i][j] = 0;
                    }
                }
                current += 1;
                continue 'searcher;
            }
        }
    }
}

/// Problem #16, part 1
pub fn problem_161(input: Vec<String>) -> RetType {
    let (tickets, rules) = parse_tickets(input);

    let mut error_rate = 0;
    for ticket in tickets {
        if let TicketValidity::Invalid(x) = is_valid_ticket(ticket.clone(), &rules) {
            error_rate += x;
        }
    }

    RetType::U128(error_rate as u128)
}

/// Problem #16, part 2
pub fn problem_162(input: Vec<String>) -> RetType {
    let (tickets, rules) = parse_tickets(input);

    let valid_tickets: Vec<Ticket> = tickets
        .into_iter()
        .filter(|x| is_valid_ticket(x.clone(), &rules) == TicketValidity::Valid)
        .collect()
    ;

    let res = vote_system(&valid_tickets, &rules);

    let mut ans: u128 = 1;
    for (key, _val) in rules.iter() {
        if key.clone().contains("departure") {
            let idx = res.get(&key.clone()).unwrap();
            ans *= valid_tickets[0].unassigned_fields[*idx] as u128;
        }
        
    }

    RetType::U128(ans)
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
    fn test_ticket_parsing() {
        init();

        let input = vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ];

        let res = parse_tickets(input);

        // Test tickets
        assert_eq!(res.0[0].unassigned_fields, vec![7,1,14]);
        assert_eq!(res.0[1].unassigned_fields, vec![7,3,47]);
        assert_eq!(res.0[2].unassigned_fields, vec![40,4,50]);
        assert_eq!(res.0[3].unassigned_fields, vec![55,2,20]);
        assert_eq!(res.0[4].unassigned_fields, vec![38,6,12]);

        // Test valid ranges
        assert_eq!(*res.1.get("class").unwrap(), vec![(1,3),(5,7)]);
        assert_eq!(*res.1.get("row").unwrap(), vec![(6,11),(33,44)]);
        assert_eq!(*res.1.get("seat").unwrap(), vec![(13,40),(45,50)]);

        // Test ticket validity
        assert_eq!(is_valid_ticket(res.0[0].clone(), &res.1), TicketValidity::Valid);
        assert_eq!(is_valid_ticket(res.0[1].clone(), &res.1), TicketValidity::Valid);
        assert_eq!(is_valid_ticket(res.0[2].clone(), &res.1), TicketValidity::Invalid(4));
        assert_eq!(is_valid_ticket(res.0[3].clone(), &res.1), TicketValidity::Invalid(55));
        assert_eq!(is_valid_ticket(res.0[4].clone(), &res.1), TicketValidity::Invalid(12));
    }

    #[test]
    fn test_ticket_magicking() {
        init();

        let input = vec![
            "class: 0-1 or 4-19".to_string(),
            "row: 0-5 or 8-19".to_string(),
            "seat: 0-13 or 16-19".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "11,12,13".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "3,9,18".to_string(),
            "15,1,5".to_string(),
            "5,14,9".to_string(),
        ];
        let (tickets, rules) = parse_tickets(input);

        let valid_tickets: Vec<Ticket> = tickets
            .into_iter()
            .filter(|x| is_valid_ticket(x.clone(), &rules) == TicketValidity::Valid)
            .collect()
        ;

        let res = vote_system(&valid_tickets, &rules);

        debug!("res: {:?}", res);

    }

    #[test]
    fn test_reduce() {
        let mut input: Vec<Vec<u32>> = vec![
            vec![0, 0, 1, 1],
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 1],
            vec![0, 0, 0, 1],
        ];

        reduce(&mut input);
    }
}