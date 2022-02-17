#![feature(type_ascription)]

use csv::Reader;
use std::{collections::HashMap, fmt, ops::Range};

fn main() {
    let mut reader = Reader::from_path("FULL_DATASET_MASTER.csv").unwrap();
    let questions: Vec<_> = reader
        .headers()
        .unwrap()
        .into_iter()
        .skip(13)
        .map(|s| s.to_owned())
        .collect();
    let answers: Vec<Vec<_>> = reader
        .records()
        .map(|r| {
            r.unwrap()
                .into_iter()
                .skip(13)
                .map(|s| s.to_owned())
                .collect()
        })
        .collect();

    println!("total: {}", answers.len());

    // TODO group subtotlas

    // Testing
    // println!("{:?}", histogram2(340, 341, &answers));
    // println!("{:?}", match_question("", &questions));
    // print_table2(histogram2(340, 341, &answers));

    // Target platform
    // println!("{:?}", match_question("which operating systems or runtimes do you develop", &questions));
    // println!("{:#?}", &questions[24..35]);
    // print_table(histogram(24, &answers));
    // print_table(histogram(25, &answers));
    // print_table(histogram(34, &answers));
    // print_table(counts(25..35, &questions, &answers));
    // print_table(levels(25..35, &answers));
    // print_table2(counts_multi(25..35, &[2, 3, 7, 10], &questions, &answers));

    // Editors
    // println!("{:?}", match_question("which editor or ide setup", &questions));
    // println!("{:#?}", &questions[71..81]);
    print_table(counts(72..81, &questions, &answers));
    let g0: &[_] = &[72, 73, 74, 75, 76, 77, 78, 79, 80];
    let g1: &[_] = &[73, 75, 76, 79];
    let g2: &[_] = &[72, 74, 77, 78];
    let g3: &[_] = &[80];
    let g4: &[_] = &[74, 77, 78];
    let groups = count_groups(&[g0, g1, g2, g3, g4], &questions, &answers);
    print_table(groups);
    // print_table(histogram(80, &answers));

    // Debuggers
    // println!("{:?}", match_question("Which debuggers do you use for", &questions));
    // println!("{:#?}", &questions[81..91]);
    // print_table(counts(82..91, &questions, &answers));
    // let g0: &[_] = &[82, 83, 84, 85, 86, 87, 88, 89, 90];
    // let g1: &[_] = &[82, 83, 84, 85, 86, 87, 88];
    // let g2: &[_] = &[89];
    // let g3: &[_] = &[82, 83];
    // let groups = count_groups(&[g1, g2, g3, g0], &questions, &answers);
    // print_table(groups);
    // print_table(histogram(90, &answers));
}

fn counts(
    range: Range<usize>,
    questions: &[String],
    answers: &[Vec<String>],
) -> HashMap<String, u64> {
    let mut result = HashMap::new();

    for i in range {
        let mut count = 0;
        for a in answers {
            if !a[i].is_empty() {
                count += 1;
            }
        }
        result.insert(questions[i].clone(), count);
        // result.insert(format!("{i}: {}", &questions[i]), count);
    }

    result
}

fn count_groups(
    groups: &[&[usize]],
    questions: &[String],
    answers: &[Vec<String>],
) -> HashMap<String, u64> {
    let mut result = HashMap::new();

    for g in groups {
        let mut count = 0;
        for a in answers {
            for &i in *g {
                if !a[i].is_empty() {
                    count += 1;
                    break;
                }
            }
        }
        result.insert(
            format!(
                "{:?}",
                g.iter().map(|&i| questions[i].clone()).collect::<Vec<_>>()
            ),
            count,
        );
    }

    result
}

fn counts_multi(
    question_range: Range<usize>,
    count_bounds: &[usize],
    questions: &[String],
    answers: &[Vec<String>],
) -> HashMap<(String, usize), u64> {
    let mut result = HashMap::new();

    for a in answers {
        let mut count = 0;
        for i in question_range.clone() {
            if !a[i].is_empty() {
                count += 1;
            }
        }
        let bound = bound_of_count(count, count_bounds);

        for i in question_range.clone() {
            if !a[i].is_empty() {
                *result.entry((questions[i].clone(), bound)).or_insert(0) += 1;
            }
        }
    }

    result
}

fn bound_of_count(count: usize, count_bounds: &[usize]) -> usize {
    let mut bound = 0;
    for &b in count_bounds {
        if count < b {
            return b;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bound_of_count() {
        fn test_range(count_bounds: &[usize], results: &[usize]) {
            for (i, r) in results.into_iter().enumerate() {
                assert_eq!(bound_of_count(i, count_bounds), *r);
            }
        }

        test_range(&[0], &[0, 0, 0]);
        test_range(&[0, 1], &[1, 0, 0]);
        test_range(&[0, 2], &[2, 2, 0]);
        test_range(&[1, 2], &[1, 2, 0]);
        test_range(&[3, 5, 6], &[3, 3, 3, 5, 5, 6, 0, 0]);
    }
}

fn levels(range: Range<usize>, answers: &[Vec<String>]) -> HashMap<usize, u64> {
    let mut result = HashMap::new();

    for a in answers {
        let mut count = 0;
        for i in range.clone() {
            if !a[i].is_empty() {
                count += 1;
            }
        }
        *result.entry(count).or_insert(0) += 1;
    }

    result
}

fn histogram(index: usize, answers: &[Vec<String>]) -> HashMap<String, u64> {
    let mut result = HashMap::new();

    for a in answers {
        *result.entry(a[index].clone()).or_insert(0) += 1;
    }

    result
}

fn histogram2(i1: usize, i2: usize, answers: &[Vec<String>]) -> HashMap<(String, String), u64> {
    let mut result = HashMap::new();

    for a in answers {
        *result.entry((a[i1].clone(), a[i2].clone())).or_insert(0) += 1;
    }

    result
}

fn match_question(s: &str, questions: &[String]) -> Option<usize> {
    let s = &s.to_lowercase();
    let mut index = None;
    for (i, q) in questions.iter().enumerate() {
        if q.to_lowercase().starts_with(s) {
            if index.is_some() {
                eprintln!("Multiple hits");
                return None;
            }
            index = Some(i)
        }
    }
    index
}

fn print_table(map: HashMap<impl fmt::Display, impl fmt::Display>) {
    let mut pairs: Vec<(_, _)> = map
        .into_iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .filter(|(k, _)| !k.is_empty())
        .collect();

    if pairs.is_empty() {
        println!("<Empty>");
        return;
    }

    // TODO better to sort before stringifying, but then need to filter later too
    pairs.sort_by_key(|&(ref k, _)| k.clone());
    let width = pairs.iter().map(|&(ref k, _)| k.len()).max().unwrap() + 4;

    for (k, v) in pairs {
        println!("{k:width$}{v}");
    }
}

fn print_table2(map: HashMap<(impl fmt::Display, impl fmt::Display), impl fmt::Display>) {
    let mut triples: Vec<(_, _, _)> = map
        .into_iter()
        .map(|((k1, k2), v)| (k1.to_string(), k2.to_string(), v.to_string()))
        .filter(|(k1, k2, _)| !k1.is_empty() && !k2.is_empty())
        .collect();

    if triples.is_empty() {
        println!("<Empty>");
        return;
    }

    // TODO better to sort before stringifying, but then need to filter later too
    triples.sort_by_key(|&(ref k1, ref k2, _)| k1.clone() + k2);

    let width1 = triples.iter().map(|&(ref k, _, _)| k.len()).max().unwrap() + 4;
    let width2 = triples.iter().map(|&(_, ref k, _)| k.len()).max().unwrap() + 4;

    for (k1, k2, v) in triples {
        println!("{k1:width1$}{k2:width2$}{v}")
    }
}
