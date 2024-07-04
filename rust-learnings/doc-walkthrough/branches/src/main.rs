fn main() {
    let number = 5;

    if number < 5 {
        println!("number is less than 5");
    } else if number == 5 {
        println!("number is equal to 5");
    } else {
        println!("number is greater than 5");
    }

    // expression must be evaluated to a bool to be used in branch
    // condition, something like this is not allowed
    // if number { } -- disallowed, since number evaluates to 3
    // there is not automatic conversion from non-boolean types to bool

    let mut counter = 0;

    // loop is used to simply loop over infinitely unless we break or return based on a certain condition
    // can be useful in certain "retry patterns" when you need to keep retrying until some condition becomes true
    // you can label loops to explicitly state where to "break from", useful in case of nested loops
    let result = 'count_twenty: loop {
        counter += 1;

        if counter == 10 {
            break 'count_twenty counter * 2; // if you break with a certain expression that evaluation is returned
        }
    };

    println!("The result is {result}");

    // while loop to replace the above loop 
    let mut a = [10, 20, 30, 40, 50]; // need to declare as mut if you need to modify individual array elements
    let mut i = 0;

    while i < 5 {
        println!("adding +1 to value {}", a[i]);
        a[i] += 1;
        i += 1;
    }
    
    // loop through collection using for loop
    // since there is no way of going beyond the array length in this case, this is more type safe and is considered 
    for element in a {
        println!("the value (in for loop) is: {element}");
    }

    // loop through range, end is not included in the loop, start to end - 1
    for number in (1..6).rev() {
        println!("{number}!");
    }
    println!("liftoff!!!");
}
