/*  

Box is used to store the value inside it on heap and return a pointer to that value. Since we are only storing the pointer to an
object on the heap, the size is known at compile time.

Mutation is allowed

Exclusive access. Only one owner.
 */
struct Node {
    value : u32,
    next : Option<Box<Node>>
}





fn main() {
    let n1 = Node {
        value:20,
        next:Some(Box::new(Node{
            value:30,
            next:None
        }))
    };
}
