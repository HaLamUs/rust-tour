
mod front_of_house;

// use crate::front_of_house::hosting;
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
  // front_of_house::hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}
