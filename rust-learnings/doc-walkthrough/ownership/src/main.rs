fn main() {
    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // append a literal to a string

        println!("{s}");
        // the memory "owned" by s is freed once it goes out of scope
    }

    let s1 = String::from("This is stored in heap.");

    // s1 is not copied over to s2
    // a string is composed of a ptr to the memeory in heap, length of the string and the capacity
    // {ptr, len, cap} struct is copied over to s2 i.e. actual contents of the heap stay as they are
    let s2 = s1; 

    // after assigning to s2, rust no longer considers s1 to be valid
    // since they are both in scope, and trying to free both will 
    // result in double free (since both point to the same mem location)
    
    // this is an invalid operation because the ownership of the string
    // in the heap actually belongs to s2
    // this is not a shallow copy because the initial variable is also
    // invalidated, hence s2 = s1 is termed as a "move"
    // println!("{s1}, world!"); // value borrowed after move

    // cloning creates a deep copy, however is expensive in terms of performance
    let _s3 = s2.clone();
    println!("{s2} - this is accessible");

    // the ownership issue is not relevant for integers or primitive 
    // data types since they internally implement a "Copy" trait that 
    // ensures variables are trivially copied rather than moved
    // edge case: tuples implement Copy if it only contains types that 
    // also implement Copy (i32, i32) but not (i32, String)
    let y = 2;
    let _x = y;
    println!("{y} can still be accessed.");
}
