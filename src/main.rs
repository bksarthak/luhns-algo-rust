mod luhns_algo {

    pub fn remove_spaces(x: String) -> String {
        if x.len() <= 2 {
            return String::from("The input is less than 2 characters long");
        } else {
            let y: String = stringr::Stringr::remove_whitespace(&x);
            return y;
        }
    }
    pub fn core_luhns(y: String) -> Vec<i32> {
        // we will use Vector to store value which will be the number
        let v: Vec<i32> = y
            .into_bytes()
            .into_iter()
            .map(|b: u8| b as i32 - 48)
            .collect::<Vec<i32>>(); // TODO : understand this line
        return v;
    }

    pub fn validation(z: Vec<i32>) -> bool {
        // TODO: P0 : Finish this function
        if z.is_empty() {
            println!("There are no elements in the vector"); // Print this message if the vector is empty
        }
        let mut temp_vec: Vec<i32> = vec![]; // will store the sum of the digts of the array
        for (pos, elem) in z.iter().rev().enumerate() {
            // println!("position: {} has {}", pos, elem); //confirm the vector is reversed
            if pos % 2 != 0 {
                temp_vec.push(elem * 2);
            }
        }
        println!(
            "The vector which is double every 2nd digits in reverse: {:?}",
            temp_vec
        ); // this is the reverse doubled array
        let mut sum: i32 = 0; // this is the variable to hold the final sum
        for (_pos, elem) in temp_vec.iter().enumerate() {
            // let mut dbl_digit_vec: Vec<i8> = vec![];
            if elem >= &i32::from(10) {
                let digits: Vec<u32> = elem
                    .to_string()
                    .chars()
                    .map(|d: char| d.to_digit(10).unwrap())
                    .collect();
                let sum_ind: u32 = digits.iter().sum();
                sum = sum + i32::try_from(sum_ind).unwrap(); //unwrap is used to get the i32 from the `try_from` struct
            } else {
                sum = sum + elem;
            }
        }
        println!("the sum of all the digits is : {}", sum);
        if sum % 10 == 0 {
            return true;
        } else {
            return false;
        }
    }
}

fn main() {
    // TODO add user input
    let x: String = luhns_algo::remove_spaces(String::from("79927398713"));
    println!("The input digits are {}", x);
    let formatted_unwrapped_digits = format!("{}", x);
    let num_vec: Vec<i32> = luhns_algo::core_luhns(formatted_unwrapped_digits);
    if luhns_algo::validation(num_vec) {
        println!("The cc number is valid");
    } else {
        println!("The cc number is invalid");
    }
}
