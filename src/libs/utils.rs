use std::env;
use std::fs::{File};
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader, self};
use rand::{seq::SliceRandom, thread_rng};
use rand::distributions::{Distribution, Uniform};

use crate::libs::patterns::{pattern1, pattern2, pattern3, pattern4, pattern5, pattern6
                           , pattern7, pattern8, pattern9, pattern10, pattern11, pattern12
                           , pattern13, pattern14, pattern15, pattern16};

pub struct Gen {
    result: Vec<String>,
}

impl Gen {
    pub fn new(keyword2: String) -> Self {
        //let output_path = FileHandler::read_file("output.txt");
        let keyword_1_path = FileHandler::read_file("keyword_1.txt");
        let keyword_1: Vec<String> = lines_from_file(keyword_1_path.get_path()).expect("Could not read line from file");
        let kw1 = get_kw_1(&keyword_1);

        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..16);

        let mut result: Vec<String> = vec![];
        let mut dork_result: Vec<String> = vec![];

        let mut patterns_rand_vec = vec![];
        let mut patterns_rand = die.sample(&mut rng);
        //println!("random number {}", patterns_rand);
        for _n in 0..5 {
            while patterns_rand_vec.contains(&mut patterns_rand) {
                patterns_rand = die.sample(&mut rng);
                if !patterns_rand_vec.contains(&mut patterns_rand) {
                    break;
                }
            }
            match patterns_rand {
                1 => { dork_result = pattern1(&kw1, &keyword2); },
                2 => { dork_result = pattern2(&kw1, &keyword2); },
                3 => { dork_result = pattern3(&kw1, &keyword2); },
                4 => { dork_result = pattern4(&kw1, &keyword2); },
                5 => { dork_result = pattern5(&kw1, &keyword2); },
                6 => { dork_result = pattern6(&kw1, &keyword2); },
                7 => { dork_result = pattern7(&kw1, &keyword2); },
                8 => { dork_result = pattern8(&kw1, &keyword2); },
                9 => { dork_result = pattern9(&kw1, &keyword2); },
                10 => { dork_result = pattern10(&kw1, &keyword2); },
                11 => { dork_result = pattern11(&kw1, &keyword2); },
                12 => { dork_result = pattern12(&kw1, &keyword2); },
                13 => { dork_result = pattern13(&kw1, &keyword2); },
                14 => { dork_result = pattern14(&kw1, &keyword2); },
                15 => { dork_result = pattern15(&kw1, &keyword2); },
                16 => { dork_result = pattern16(&kw1, &keyword2); },
                _ => {}
            }
            result.append(&mut dork_result);
            //println!("{:?}", result);
            patterns_rand_vec.push(patterns_rand);
            patterns_rand = die.sample(&mut rng);
            //println!("{:?}", patterns_rand_vec);
        }
        // let result_to_write = result.clone();
        // let mut file = fs::OpenOptions::new()
        //     .write(true)
        //     .append(true)
        //     .open(&output_path.get_path())
        //     .unwrap();
        // for line in result_to_write.into_iter() {
        //     write!(file, "{}", line).unwrap();
        // }
        //println!("Done {:?}", result);
        Self {
            result,
        }
    }

    pub fn get_result(&self) ->Vec<String> {
        self.result.clone()
    }
}

pub fn inner_main() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    let dir = exe.parent().expect("Executable must be in some directory");
    let dir = dir.join("resources");
    Ok(dir)
}

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?)
        .lines()
        .collect()
}

#[derive(Clone, Debug)]
pub struct FileHandler {
    public_path: String,
}

impl FileHandler {
    pub fn read_file(file_name: &str) -> Self {
        let mut dir = inner_main().expect("Couldn't find path");
        dir.push(file_name);
        Self {
            public_path: dir.display().to_string(),
        }
    }

    pub fn get_path(&self) -> String {
        self.public_path.clone()
    }
}

#[warn(non_camel_case_types)]
#[derive(Debug, Clone)]
pub struct SearchExpression {
    search_expression: String,
}

impl SearchExpression {
    pub fn search_exp() -> Self {
        let search_exp = vec!["|".to_string(),
                              "AND".to_string(),
                              "*".to_string(),
                              "/".to_string()];
        let rand_search_exp = search_exp.choose(&mut thread_rng()).unwrap().to_string();

        Self {
            search_expression: rand_search_exp,
        }
    }

    pub fn get_search_exp(&self) -> String { self.search_expression.clone() }
}

pub fn get_kw_1(kw1_vec: &[String]) -> String{
    kw1_vec.choose(&mut thread_rng()).unwrap().to_string()
}