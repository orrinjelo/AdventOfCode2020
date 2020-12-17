use regex::Regex;
use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error
use crate::matrix;
use crate::util::RetType;

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

/// light red bags contain 1 bright white bag, 2 muted yellow bags.
/// dark orange bags contain 3 bright white bags, 4 muted yellow bags.
/// bright white bags contain 1 shiny gold bag.
/// muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
/// shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
/// dark olive bags contain 3 faded blue bags, 4 dotted black bags.
/// vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
/// faded blue bags contain no other bags.
/// dotted black bags contain no other bags.
fn parse_luggage_line(line: String) -> (String, Vec<(u32, String)>) {
    let mut contents: Vec<(u32, String)> = Vec::new();

    let content_re = Regex::new(r"(.+) bags contain (.+)").unwrap();
    let bag_types_re = Regex::new(r"(\d+) (\w+\s\w+) bag").unwrap();
    let content_cap = content_re.captures(&line).unwrap();

    let container = content_cap[1].parse::<String>().unwrap();

    // Contents would be <original>, <desc left>, "no other bags"
    if content_cap.len() == 3 {
        if content_cap[2].parse::<String>().unwrap() != "no other bags." {
            let bag = content_cap[2].parse::<String>().unwrap();
            for cont in bag_types_re.find_iter(&bag) {
                let cont_entry = cont.as_str();
                let bag_cap = bag_types_re.captures(&cont_entry).unwrap();
                let num = bag_cap[1].parse::<u32>().unwrap();
                let bag_type = bag_cap[2].parse::<String>().unwrap();
                contents.push((num, bag_type.trim().to_string()));
                trace!("{:?} {:?}", num, bag_type.trim());
            }
        }
    }

    return (container.to_string(), contents);
}

// These should probably be spun out into its own directed graph
fn add_node_and_edges(g: &mut matrix::SquareMatrix<u32>, n: String, v: Vec<(u32, String)>) {
    for edge in v {
        g.set_in_labelled_row(n.clone(), edge.1, edge.0);
    }
}

fn contains(g: &mut matrix::SquareMatrix<u32>, n: String, m: String) -> bool {
    let sg_index = g.index_of(m.clone());

    match sg_index {
        Some(sg) => {
            let mut res = false;

            let container = g.get_labelled_row(n.clone());

            // trace!("Container: {:?}", container);
            // trace!("labels: {:?}", g.get_labels());
            // trace!("g: {:?}", g.get_values());
            // trace!("n: {:?}", n.clone());
            // trace!("m: {:?}", m.clone());

            if container[sg] > 0 {
                return true;
            } else {
                for i in 0..container.len() {
                    if container[i] > 0 {
                        res |= contains(g, g.get_label(i), m.clone());
                    }
                }
            }

            res
        },
        None => false
    }
}

fn forward_count(g: &mut matrix::SquareMatrix<u32>, n: String) -> u32 {
        let mut count = 0;
        let container = g.get_labelled_row(n.clone());

        trace!("n = {}, {:?}", n.clone(), container);

        for i in 0..container.len() {
            if container[i] != 0 {
                count += container[i] + container[i]*forward_count(g, g.get_labels()[i].clone());
            }
        }

        count
}

/// Problem #07, part 1
/// How many bag colors can eventually contain at least one shiny gold bag?
pub fn problem_071(input: Vec<String>) -> RetType {
    let mut count: u32 = 0;
    let mut g = matrix::SquareMatrix::new(0);
    for line in input.into_iter() {
        let (bag, contents) = parse_luggage_line(line);
        add_node_and_edges(&mut g, bag, contents);
    }

    trace!("Labels: {:?}", g.get_labels());
    trace!("Values: {:?}", g.get_values());

    for bag_type in g.get_labels() {
        if contains(&mut g, bag_type, "shiny gold".to_string()) {
            count += 1;
        }
    }

    return RetType::U32(count);
}

/// Problem #07, part 2
/// How many bag colors can eventually contain at least one shiny gold bag?
pub fn problem_072(input: Vec<String>) -> RetType {
    let mut g = matrix::SquareMatrix::new(0);
    for line in input.into_iter() {
        let (bag, contents) = parse_luggage_line(line);
        add_node_and_edges(&mut g, bag, contents);
    }

    RetType::U32(forward_count(&mut g, "shiny gold".to_string()))
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
    fn test_parse_luggage_line() {
        init();
        let (bag, contents) = parse_luggage_line("light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string());

        assert_eq!(bag, "light red".to_string());
        assert_eq!(contents.len(), 2);
        assert_eq!(contents[0], (1, "bright white".to_string()));
        assert_eq!(contents[1], (2, "muted yellow".to_string()));

        let (bag2, contents2) = parse_luggage_line("bright white bags contain 1 shiny gold bag.".to_string());
        assert_eq!(bag2, "bright white".to_string());
        assert_eq!(contents2.len(), 1);
        assert_eq!(contents2[0], (1, "shiny gold".to_string()));

        let (bag3, contents3) = parse_luggage_line("faded blue bags contain no other bags.".to_string());
        assert_eq!(bag3, "faded blue".to_string());
        assert_eq!(contents3.len(), 0);
    }

    #[test]
    fn test_create_graph_1() {
        init();

        let mut g = matrix::SquareMatrix::new(0);

        add_node_and_edges(&mut g, "light red".to_string(), vec![(1, "bright white".to_string()), (2, "muted yellow".to_string())]);

        assert_eq!(g.size(), 3);
        assert_eq!(g.get_labelled_row("light red".to_string()), vec![0, 1, 2]);

        add_node_and_edges(&mut g, "bright white".to_string(), vec![(1, "shiny gold".to_string())]);

        assert_eq!(g.size(), 4);
        assert_eq!(g.get_labelled_row("bright white".to_string()), vec![0, 0, 0, 1]);
    }

    #[test]
    fn test_contains() {
        init();

        let mut g = matrix::SquareMatrix::new(0);

        add_node_and_edges(&mut g, "light red".to_string(), vec![(1, "bright white".to_string()), (2, "muted yellow".to_string())]);

        assert_eq!(g.size(), 3);
        assert_eq!(g.get_labelled_row("light red".to_string()), vec![0, 1, 2]);

        add_node_and_edges(&mut g, "bright white".to_string(), vec![(1, "shiny gold".to_string())]);

        assert!(contains(&mut g, "light red".to_string(), "shiny gold".to_string()));
    }

    #[test]
    fn test_problem_071() {
        init();

        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];

        let res = problem_071(input);

        assert_eq!(res, 4);
    }

    #[test]
    fn test_forward_count() {
        init();

        let input = vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ];
    
        let mut g = matrix::SquareMatrix::new(0);
        for line in input.into_iter() {
            let (bag, contents) = parse_luggage_line(line);
            add_node_and_edges(&mut g, bag, contents);
        }

        trace!("Labels: {:?}", g.get_labels());
        trace!("Values: {:?}", g.get_values());


        let res = forward_count(&mut g, "shiny gold".to_string());

        assert_eq!(res, 126);
    }
}