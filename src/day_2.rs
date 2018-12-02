use std::fs::File;
use std::io::Read;

pub fn run() {
    println!("Day 2!");

    let mut f = File::open("day_2.txt").expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Unable to load file");

    let mut w_2 = 0;
    let mut w_3 = 0;
    let mut found_pair = false;

    let box_ids = s.split_whitespace().collect::<Vec<_>>();

    for box_id in box_ids.iter() {
        let mut chars: Vec<char> = box_id.chars().collect();
        chars.sort();
        chars.dedup();

        let mut h_2 = false;
        let mut h_3 = false;

        for c in chars {
            let mut count = 0;

            for inner_c in box_id.chars() {
                if inner_c == c {
                    count = count + 1;
                }
            }

            if count == 2 && !h_2 {
                w_2 = w_2 + 1;
                h_2 = true;
            }

            if count == 3 && !h_3 {
                w_3 = w_3 + 1;
                h_3 = true;
            }
        }

        if !found_pair {
            for other_box_id in box_ids.iter() {
                if box_id == other_box_id {
                    continue;
                }

                let count = box_id.chars().zip(other_box_id.chars())
                    .filter(|(a, b)| a != b)
                    .count();

                if count == 1 {
                    println!("Found codes {} {}", box_id, other_box_id);

                    for (a, b) in box_id.chars().zip(other_box_id.chars()) {
                        if a == b {
                            print!("{}", a);
                        }
                    }

                    println!("");

                    found_pair = true;

                    break;
                }
            }
        }
    }

    let checksum = w_2 * w_3;
    println!("Final checksum {}", checksum);
}
