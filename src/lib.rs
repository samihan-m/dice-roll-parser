pub mod parsing;

#[cfg(test)]
mod tests {
    use super::parsing::*;
    use rand::Rng;
    
    fn get_distribution_samples(die_size: i32, sample_count: i32) -> Vec<i32> {
        let mut rng = rand::thread_rng();
        return (0..sample_count)
            .map(|_| rng.gen_range(1..=die_size))
            .collect::<Vec<i32>>();
    }

    #[test]
    fn rng_distributions_are_comparable() {
        let xs = get_distribution_samples(20, 1000);
        let ys = get_distribution_samples(20, 1000);
        let confidence = 0.95;

        let result = kolmogorov_smirnov::test(&xs, &ys, confidence);

        assert!(!result.is_rejected, "Cannot say the two distributions are different with {}% confidence", confidence * 100.0);
    }

    #[test]
    fn regex_parse_basic_roll_works() {
        let sample_count = 1000;
        let xs = get_distribution_samples(20, sample_count);
        let ys = (0..sample_count).map(|_| parse_roll_with_regex("d20")).collect::<Vec<i32>>();
        let confidence = 0.99;

        let result = kolmogorov_smirnov::test(&xs, &ys, confidence);

        assert!(!result.is_rejected, "Cannot say the two distributions are different with {}% confidence", confidence * 100.0);
    }

    #[test]
    fn state_machine_parse_basic_roll_works() {
        let sample_count = 1000;
        let xs = get_distribution_samples(20, sample_count);
        let ys = (0..sample_count).map(|_| parse_roll_with_state_machine("d20")).collect::<Vec<i32>>();
        let confidence = 0.99;

        let result = kolmogorov_smirnov::test(&xs, &ys, confidence);

        assert!(!result.is_rejected, "Cannot say the two distributions are different with {}% confidence", confidence * 100.0);
    }

    #[test]
    fn string_split_parse_basic_roll_works() {
        let sample_count = 1000;
        let xs = get_distribution_samples(20, sample_count);
        let ys = (0..sample_count).map(|_| parse_roll_with_string_splits("d20")).collect::<Vec<i32>>();
        let confidence = 0.99;

        let result = kolmogorov_smirnov::test(&xs, &ys, confidence);

        assert!(!result.is_rejected, "Cannot say the two distributions are the same with {}% confidence", confidence * 100.0);
    }

}
