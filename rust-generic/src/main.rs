fn main() {

  let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest = get_largest(char_list);
    println!("The largest number is {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
  let mut largest = list[0];
  for number in list {
    if number > largest {
      largest = number
    }
  }
  largest
}

// fn get_largest(number_list: Vec<i32>) -> i32 {
//     let mut largest = number_list[0];
//     for number in number_list {
//       if number > largest {
//         largest = number
//       }
//     }
//     largest
// }

fn get_largest_char(number_list: Vec<char>) -> char {
  let mut largest = number_list[0];
  for number in number_list {
    if number > largest {
      largest = number
    }
  }
  largest
}
