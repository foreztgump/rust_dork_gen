use crate::libs::utils::Gen;
use crate::libs::utils::FileHandler;
use crate::libs::utils::lines_from_file;

use std::io::Write;
use std::fs;
use rayon::ThreadPoolBuilder;
use std::{sync, thread, time};
use indicatif::{ProgressBar, ProgressStyle};


pub struct DorkGen {
    path: String,
    output: String
}

impl DorkGen {
    pub fn new(path: String) -> Self {
        let keyword_path = FileHandler::read_file(path.as_str());
        let output_path = FileHandler::read_file("output.txt");
        //println!("{}", keyword_path.get_path());
        Self {
            path: keyword_path.get_path(),
            output: output_path.get_path(),
        }
    }

    pub fn run_gen(&mut self) {
        let delay = time::Duration::from_millis(250);
        let pool = ThreadPoolBuilder::new()
            .num_threads(75)
            .build()
            .expect("Failed building thread pool.");

        let (tx, rx) = sync::mpsc::channel();
        let keyword_2: Vec<String> = lines_from_file(&self.path).expect("Could not read line from file");

        println!("Starting process...\n");
        //make progress bar
        let pb = ProgressBar::new(keyword_2.len() as u64);
        pb.set_style(ProgressStyle::default_bar().template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({percent}/100%)",
        ));

        for k in keyword_2.into_iter() {
            let keyword = k.to_string();
            let tx = tx.clone();

            pool.spawn(move || {
                tx.send(Gen::new(keyword)).unwrap();
                //println!(" {:?}", thread::current().id());
                thread::sleep(delay);
            });
        };

        drop(tx);

        let mut dork_count = 0;
        rx.into_iter().for_each(|keyword| {
            pb.inc(1);
            if !keyword.get_result().is_empty() {
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&self.output)
                    .unwrap();
                for line in keyword.get_result().into_iter() {
                    dork_count += 1;
                    write!(file, "{}", line).unwrap();
                }
                //println!("Done {:?}", thread::current().id());
            }
        });
        pb.finish();
        println!("\nGenerated : {} dorks!", dork_count);
    }
}