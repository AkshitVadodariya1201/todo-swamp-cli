#[cfg(test)]
mod tests {
    use rand::Rng;

    fn warp_5_replace_20_drop(word: &str) -> String {
        let mut rng = rand::thread_rng();
        word.chars()
            .filter_map(|c| {
                if rng.gen_bool(0.20) {
                    None
                } else if rng.gen_bool(0.05) {
                    Some(rng.gen_range(b'a', b'z' + 1) as char)
                } else {
                    Some(c)
                }
            })
            .collect()
    }

    fn warp_25_insertion(word: &str) -> String {
        let mut rng = rand::thread_rng();
        let mut warped = String::new();
        for c in word.chars() {
            if rng.gen_bool(0.25) {
                warped.push(rng.gen_range(b'a', b'z' + 1) as char);
            }
            warped.push(c);
            if rng.gen_bool(0.25) {
                warped.push(rng.gen_range(b'a', b'z' + 1) as char);
            }
        }
        warped
    }

    fn generate_standard(words: &[&str]) -> Vec<String> {
        let mut commands = vec![];
        for word in words {
            let warped_word = warp_5_replace_20_drop(word);
            if !warped_word.is_empty() {
                commands.push(format!("add \"{}\" #tag1 #tag2", warped_word));
            }
        }
        commands.push("done 1".to_string());
        commands
    }

    fn generate_ratio_done(words: &[&str]) -> Vec<String> {
        let mut commands = vec![];
        for word in words {
            let warped_word = warp_5_replace_20_drop(word);
            if !warped_word.is_empty() {
                commands.push(format!("add \"{}\" #tag1 #tag2", warped_word));
            }
        }
        for i in 1..=words.len() / 2 {
            commands.push(format!("done {}", i));
        }
        commands.push("search #tag1".to_string());
        commands
    }

    fn generate_interleaved(words: &[&str]) -> Vec<String> {
        let mut commands = vec![];
        for word in words {
            let warped_word = warp_5_replace_20_drop(word);
            if !warped_word.is_empty() {
                commands.push(format!("add \"{}\" #tag1 #tag2", warped_word));
            }
        }
        for i in 1..=words.len() {
            commands.push(format!("done {}", i));
            commands.push("search #tag1".to_string());
        }
        commands
    }

    fn generate_lattice(words: &[&str]) -> Vec<String> {
        let mut commands = vec![];
        for word in words {
            let warped_word = warp_25_insertion(word);
            if !warped_word.is_empty() {
                commands.push(format!("add \"{}\" #tag1 #tag2", warped_word));
            }
        }
        commands.push("search #tag1".to_string());
        commands
    }

    #[test]
    fn test_standard_generator() {
        let words = vec!["example", "test", "words"];
        let commands = generate_standard(&words);
        assert!(commands.len() > 0);
    }

    #[test]
    fn test_ratio_done_generator() {
        let words = vec!["example", "test", "words"];
        let commands = generate_ratio_done(&words);
        assert!(commands.len() > 0);
    }

    #[test]
    fn test_interleaved_generator() {
        let words = vec!["example", "test", "words"];
        let commands = generate_interleaved(&words);
        assert!(commands.len() > 0);
    }

    #[test]
    fn test_lattice_generator() {
        let words = vec!["example", "test", "words"];
        let commands = generate_lattice(&words);
        assert!(commands.len() > 0);
    }
}
