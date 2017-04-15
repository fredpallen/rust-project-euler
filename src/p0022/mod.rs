use std::fs;
use std::io::Read;

pub fn solve() -> usize {
    let contents = {
        let mut file = fs::File::open("src/p0022/p022_names.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };

    let names = {
        let mut names =
            contents.split(',')
            .map(|s| s.replace('"', ""))
            .collect::<Vec<String>>();
        names.sort();
        names
    };

    let char_value = |c: char| -> usize {
        (((c as u8) - ('A' as u8)) as usize) + 1
    };

    let mut score = 0;
    for (index, name) in names.iter().enumerate() {
        let order_factor = index + 1;
        let name_score: usize = name.chars().map(&char_value).sum();
        score += order_factor * name_score;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(871198282, solve());
    }
}
