pub fn input_to_array(input: &Vec<u8>) -> [u64; 9] {
    let mut out = [0; 9];
    for i in 0..=7 {
        out[i] = input.iter().filter(|&&v| v == i as u8).count() as u64;
    }

    out
}

#[allow(dead_code)]
pub fn iterative_step(v: Vec<u8>) -> Vec<u8> {
    let mut v = v;
    let new_fishes = v.iter().filter(|&&v| v == 0).count();
    v.iter_mut()
        .for_each(|v| if *v == 0 { *v = 6 } else { *v = *v - 1 });

    let mut new_fishes = vec![8].repeat(new_fishes);
    v.append(&mut new_fishes);

    v
}

pub fn optimized_step(h: [u64; 9]) -> [u64; 9] {
    let mut out: [u64; 9] = [0; 9];

    for i in 1..=8 {
        out[i - 1] = h[i];
    }
    out[6] = out[6] + h[0];
    out[8] = h[0];

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterative_step() {
        let school = vec![3, 4, 3, 1, 2];
        let result = iterative_step(school);
        assert_eq!(result, vec![2, 3, 2, 0, 1]);
    }

    #[test]
    fn test_iterative_step_with_spawn() {
        let school = vec![2, 3, 2, 0, 1];
        let result = iterative_step(school);
        assert_eq!(result, vec![1, 2, 1, 6, 0, 8])
    }

    #[test]
    fn test_school_size_after_iterations() {
        let mut input = vec![3, 4, 3, 1, 2];
        for _ in 0..18 {
            input = iterative_step(input);
        }

        assert_eq!(input.len(), 26);
    }

    #[test]
    fn test_input_to_array() {
        let school = vec![3, 4, 3, 1, 2];
        let expected: [u64; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

        assert_eq!(input_to_array(&school), expected);
    }

    #[test]
    fn test_array_step() {
        let school = input_to_array(&vec![3, 4, 3, 1, 2]);
        let expected = [1, 1, 2, 1, 0, 0, 0, 0, 0];
        let result = optimized_step(school);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_array_step_spawn() {
        let school = input_to_array(&vec![2, 3, 2, 0, 1]);
        let expected = [1, 2, 1, 0, 0, 0, 1, 0, 1];
        let result = optimized_step(school);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_equivalence() {
        let mut input = vec![3, 4, 3, 1, 2];
        let mut array_input = input_to_array(&input);

        for _ in 0..18 {
            input = iterative_step(input);
            array_input = optimized_step(array_input);
        }

        let array_sum = array_input.iter().sum::<u64>();
        assert_eq!(input.len(), array_sum as usize);
    }
}
