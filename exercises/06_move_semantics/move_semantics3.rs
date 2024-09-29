// TODO: Fix the compiler error in the function without adding any new line.
// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {

    /*
    Why doesn't this work then?
    fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {

    https://github.com/rust-lang/rustlings/issues/2060
    > vec in the original code is 'moved' because the signature is vec: Vec<i32>.
    > As something is moved, the function grabs ownership and be able to mutate it.
    > Something like '&mut' is the borrowed syntax which creates a reference to the original variable.
    */
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
