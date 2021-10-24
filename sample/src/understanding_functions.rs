fn puls3(x: i32) -> i32 {
    x + 3
}

fn times2(x: i32) -> i32 {
    x * 3
}

fn square(x: i32) -> i32 {
    x * x
}

fn add_generator(x: i32) -> impl FnOnce(i32) -> i32 {
    move |y| x + y
}

fn add_generator1(x: i32) -> impl FnOnce(i32) -> Box<dyn FnOnce(i32) -> i32> {
    move |y| Box::new(move |z| x + y + z)
}

fn add_generator2(
    x: i32,
) -> impl FnOnce(i32) -> Box<dyn FnOnce(i32) -> Box<dyn FnOnce(i32) -> i32>> {
    move |y| Box::new(move |z| Box::new(move |a| x + y + z + a))
}

fn greeting(s1: String, s2: String) {
    println!("{} {}", s1, s2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let list_of_functions = [puls3, times2, square, |x| x % 2];
        for func in list_of_functions {
            let result = func(100);
            println!("If 100 is the input, the output is {}", result);
        }

        // curry
        let add = |x: i32| move |y: i32| move |z: i32| x + y + z;
        println!("If 1, 2, 3 is the input, the output is {}", add(1)(2)(3));
        let add_1 = add(1);
        println!("If 2, 3 is the input, the output is {}", add_1(2)(3));
        let add_1_2 = add_1(2);
        println!("If 3 is the input, the output is {}", add_1_2(3));

        // partial application
        let greeting_hello = |s2| greeting("Hello".to_string(), s2);
        greeting_hello("everyone".to_string());
    }
}
