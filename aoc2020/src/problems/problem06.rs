use std::collections::HashSet;
use log::{trace, debug, info, warn, error}; // trace, debug, info, warn, error

#[allow(dead_code)]
fn _get_rid_of_log_unused_import_warnings() {
    trace!("Example trace.");
    debug!("Example debug.");
    info!("Example info.");
    warn!("Example warn.");
    error!("Example error.");
}

fn combine_customs_forms_in_group_union(mut forms: Vec<String>) -> u32 {
    let mut yes_questions: HashSet<char> = forms.pop().unwrap().chars().collect();

    for form in forms {
        let form_set = form.chars().collect();
        yes_questions = &yes_questions | &form_set;
    }

    return yes_questions.len() as u32;
}

fn combine_customs_forms_in_group_intersection(mut forms: Vec<String>) -> u32 {
    let mut yes_questions: HashSet<char> = forms.pop().unwrap().chars().collect();

    for form in forms {
        let form_set = form.chars().collect();
        yes_questions = &yes_questions & &form_set;
    }

    return yes_questions.len() as u32;
}

fn collect_custom_forms(groups: Vec<String>, intersect: bool) -> u32 {
    let mut temp_vec: Vec<String> = Vec::new();
    let mut group_vec: Vec<u32> = Vec::new();

    for line in groups {
        if line.len() == 0 {
            if intersect {
                group_vec.push(combine_customs_forms_in_group_intersection(temp_vec.clone()));
            } else {
                group_vec.push(combine_customs_forms_in_group_union(temp_vec.clone()));
            }
            temp_vec.clear();
        } else {
            temp_vec.push(line);
        }
    }
    if intersect {
        group_vec.push(combine_customs_forms_in_group_intersection(temp_vec.clone()));
    } else {
        group_vec.push(combine_customs_forms_in_group_union(temp_vec.clone()));
    }

    return group_vec.into_iter()
        .sum()
    ;
}

/**
 *  Problem #05, part 1
 *  For each group, count the number of questions to which anyone answered "yes". 
 *   What is the sum of those counts?
 */
pub fn problem_061(input: Vec<String>) -> u32 {
    return collect_custom_forms(input, false);
}

/**
 *  Problem #05, part 2
 *  For each group, count the number of questions to which anyone answered "yes". 
 *   What is the sum of those counts?
 */
pub fn problem_062(input: Vec<String>) -> u32 {
    return collect_custom_forms(input, true);
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
    fn test_combine_customs_forms_in_group() {
        init();

        let input_1 = vec![
            "abc".to_string(),
        ];

        let input_2 = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];

        let input_3 = vec![
            "ab".to_string(),
            "ac".to_string(),
        ];

        let input_4 = vec![
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
        ];

        let input_5 = vec![
            "b".to_string(),
        ];

        // 3 + 3 + 3 + 1 + 1
        assert_eq!(combine_customs_forms_in_group_union(input_1), 3);
        assert_eq!(combine_customs_forms_in_group_union(input_2), 3);
        assert_eq!(combine_customs_forms_in_group_union(input_3), 3);
        assert_eq!(combine_customs_forms_in_group_union(input_4), 1);
        assert_eq!(combine_customs_forms_in_group_union(input_5), 1);
    }

    #[test]
    fn test_combine_customs_forms_in_group_intersect() {
        init();
        
        let input_1 = vec![
            "abc".to_string(),
        ];

        let input_2 = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
        ];

        let input_3 = vec![
            "ab".to_string(),
            "ac".to_string(),
        ];

        let input_4 = vec![
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
        ];

        let input_5 = vec![
            "b".to_string(),
        ];

        // 3 + 0 + 1 + 1 + 1
        assert_eq!(combine_customs_forms_in_group_intersection(input_1), 3);
        assert_eq!(combine_customs_forms_in_group_intersection(input_2), 0);
        assert_eq!(combine_customs_forms_in_group_intersection(input_3), 1);
        assert_eq!(combine_customs_forms_in_group_intersection(input_4), 1);
        assert_eq!(combine_customs_forms_in_group_intersection(input_5), 1);
    }
}