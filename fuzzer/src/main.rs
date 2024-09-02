use rand::Rng;

fn main() {
    let string = "Mutable!";
    let str = delete_random_char(string.to_string());
    println!("{str}");
}


fn delete_random_char(str: String) -> String {
    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(0..str.len());
    let mut mutated: String = str;
    mutated.remove(n).to_string();
    return mutated;
}
