use std::io::{self, BufRead};

use section::Section;
use skatepark::Skatepark;
use trick::Trick;

pub mod section;
pub mod skatepark;
pub mod trick;

fn main() {
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_line(&mut buffer);

    let binding = buffer.trim().to_owned();
    let first_line: Vec<&str> = binding.split(' ').collect();

    let n: u8 = first_line[0].parse().unwrap();
    let k: u8 = first_line[1].parse().unwrap();

    let mut park: Skatepark = Skatepark::new();

    let mut sections: Vec<Section> = Vec::new();
    for _ in 0..n {
        buffer.clear();
        let _ = handle.read_line(&mut buffer);
        let binding: String = buffer.trim().to_owned();
        let line: Vec<&str> = binding.split(' ').collect();

        let bonus_factor: u8 = line[0].parse().unwrap();
        let crossing_time: u32 = line[1].parse().unwrap();

        sections.push(Section::new(bonus_factor, crossing_time))
    }
    park.add_section(sections);

    let mut tricks: Vec<Trick> = Vec::new();
    for i in 0..k {
        buffer.clear();
        let _ = handle.read_line(&mut buffer);
        let binding: String = buffer.trim().to_owned();
        let line: Vec<&str> = binding.split(' ').collect();

        let baseline_score: i32 = line[0].parse().unwrap();
        let time_trick: u32 = line[1].parse().unwrap();

        tricks.push(Trick::new(baseline_score, time_trick, i))
    }
    park.add_tricks(tricks);

    park.more_radical_crossing();
    

}
