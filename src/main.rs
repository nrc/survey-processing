use csv::{Reader};
use std::{collections::HashMap, fmt};

fn main() {
    let mut reader = Reader::from_path("survey data.csv").unwrap();
    let questions: Vec<_> = reader.headers().unwrap().into_iter().skip(13).map(|s| s.to_owned()).collect();
    let answers: Vec<Vec<_>> = reader.records().map(|r| r.unwrap().into_iter().skip(13).map(|s| s.to_owned()).collect()).collect();

    // testing
    // println!("{:?}", histogram2(340, 341, &answers));
    // println!("{:?}", match_question("", &questions));
    // print_table2(histogram2(340, 341, &answers));
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

fn print_table(map: HashMap<String, impl fmt::Debug>) {
    let mut pairs: Vec<(_, _)> = map.into_iter().filter(|(k, _)| !k.is_empty()).map(|(k, v)| (k, format!("{v:?}"))).collect();
    pairs.sort_by_key(|&(ref k, _)| k.to_owned());
    let width = pairs.iter().map(|&(ref k, _)| k.len()).max().unwrap() + 4;

    for (k, v) in pairs {
        println!("{k:width$}{v}")
    }
}

fn print_table2(map: HashMap<(String, String), impl fmt::Debug>) {
    let mut triples: Vec<(_, _, _)> = map.into_iter().filter(|((k1, k2), _)| !k1.is_empty() && !k2.is_empty()).map(|((k1, k2), v)| (k1, k2, format!("{v:?}"))).collect();
    triples.sort_by_key(|&(ref k1, ref k2, _)| k1.to_owned() + k2);
    let width1 = triples.iter().map(|&(ref k, _, _)| k.len()).max().unwrap() + 4;
    let width2 = triples.iter().map(|&(_, ref k, _)| k.len()).max().unwrap() + 4;

    for (k1, k2, v) in triples {
        println!("{k1:width1$}{k2:width2$}{v}")
    }
}