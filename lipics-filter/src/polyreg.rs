/// Some simple polyregular functions on vectors.
///
/// This is not optimised, and will probably never be.

/// This function splits a vector of either A or B
/// into a prefix of Bs, followed by A's with blocks of B's.
///
/// ex: aabbababa -> (ε, (a,ε)(a,bb)(a,b)(a,b)(a,ε))
pub fn split_vec<A, B>(vector: Vec<Result<A, B>>) -> (Vec<B>, Vec<(A, Vec<B>)>) {
    let iter = vector.into_iter();
    let mut before = vec![];
    let mut after = vec![];
    let mut current_header: Option<A> = None;
    let mut current = vec![];
    for item in iter {
        match item {
            Ok(a) => match current_header {
                Some(h) => {
                    after.push((h, current));
                    current_header = Some(a);
                    current = vec![];
                }
                None => {
                    current_header = Some(a);
                    before = current;
                    current = vec![];
                }
            },
            Err(b) => {
                current.push(b);
            }
        }
    }
    match current_header {
        Some(h) => {
            after.push((h, current));
        }
        None => {
            before = current;
        }
    }
    (before, after)
}
