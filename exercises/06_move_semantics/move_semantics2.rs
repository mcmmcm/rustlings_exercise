fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
}

/*
Variables with known size will be implicitly copied when trying to bind from one to another.

For example,

x1 = 1;
x2 = x1;

The code is valid and has no borrowing. This is because 1 is a known size integaer
and the binding of x2 = x1 will implicitly call the clone

Rust has the `copy` traits for known size type which will be called during assignment.
Type with `copy` traits are int, bool, float, char. And tuples of any combinations that contain only those.
(int, float) can be copied. (int, string) cannot.
*/
#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
    // fix the compiler error in the test.
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // cloning vec0 because we need to access BOTH variables.
        // Therefore, passing owwnership from vec0 to vec1 thru fill_vec wouldn't work
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
