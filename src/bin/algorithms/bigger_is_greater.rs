pub(crate) fn biggerIsGreater(w: &str) -> String {
    // Converting the input into characters
    let mut chars = w.chars().collect::<Vec<_>>();

    for x in chars.clone() {
        println!("{} - {}", x, x as u32);
    }

    // Finding the first index to swap - using a backwards to forwards sweep
    let mut first_idx = None;
    for j in (0..chars.len()).rev() {
        match j.checked_sub(1) {
            Some(i) => {
                let x = chars.get(i).unwrap();
                let y = chars.get(j).unwrap();
                if x < y {
                    first_idx = Some(i);
                    break;
                }
            }
            None => {
                break;
            }
        }
    }

    println!("{:?}, {:?}", chars, first_idx);

    match first_idx {
        Some(first_idx) => {
            let first_idx_val = &chars.get(first_idx).unwrap().clone();
            let mut second_idx = chars.len() - 1; // End of the character array
            loop {
                let second_idx_val = chars.get(second_idx).unwrap();
                if second_idx_val > first_idx_val {
                    println!(
                        "Current array: {:?}, swapping {} and {}",
                        chars, first_idx_val, second_idx_val
                    );
                    chars.swap(first_idx, second_idx);
                    break;
                }
                second_idx = second_idx - 1;
            }

            // Performing sorting of the trailing values
            let trailing_chars = chars.get_mut(first_idx + 1..).unwrap();
            trailing_chars.sort();

            return chars.into_iter().collect::<String>();
        }
        None => return String::from("no answer"),
    }
}
