pub fn run() {
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    /* With non-primitives, if you assign another varianle to a piece of data, the first
        variable will no longer hold that value. You'll need to use a reference (&) to the
        resource.
    */
    println!("Values: {:?}", (arr1, arr2));

    // Vecter
    let vec1 = vec![1,2,3];
    
    // Vectors are not primitives so you cannot point derectly to it.
    // You have to create a reference usin &vec1 
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}