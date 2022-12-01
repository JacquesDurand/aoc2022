use std::fs;

fn main() {
    let input = fs::read_to_string("input").expect("Error reading file");

    let elves: Vec<&str> = input.split("\n\n").collect();

    let mut tot_vec: Vec<u32> = Vec::new();
    for elf in elves {
        let mut tot_by_elf = 0;
        for calory in elf.lines() {
            let calory_int = calory.parse::<u32>().unwrap();
            tot_by_elf = tot_by_elf + calory_int;
        }
        tot_vec.push(tot_by_elf);
    }
    println!("{}", tot_vec.iter().max().unwrap());
    tot_vec.sort_by(|a,b| b.cmp(a));
    println!("{}", tot_vec[0] + tot_vec [1] + tot_vec[2]);
}
