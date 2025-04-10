fn main() {
    let s = "stack"; // immutable - placed on the stack
    println!("{s}");

    let mut s = String::from("heap"); // mutable - placed on the heap
    // let s_copy = s;
    // The above code doesn't work because if s goes out of scope then it
    // will free the memory. Then s_copy goes out of scope and tries to free
    // memory that has already been freed, resulting in a "double free error."
    // This happens because s_copy = s copies over the length, capacity, and
    // pointer. The last one is important because it just copies the pointer
    // and not the data in the heap. This results in 2 pointers and only 1 set
    // of data, causing that error when "drop" is called.
    s.push_str(", is where this is at");
    println!("{s}");
} // s (heap version) goes out of scope and the memory is freed
