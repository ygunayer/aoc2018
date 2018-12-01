pub fn collect_into<T, E, I>(bucket: &mut Vec<T>, iter: I) -> Result<(), E>
    where I: Iterator<Item = Result<T, E>>
{
    let coll_result = iter
        .fold(Ok(()), |acc, item| {
            if acc.is_err() {
                return acc;
            }

            match item {
                Ok(n) => {
                    bucket.push(n);
                    acc
                },
                Err(e) => Err(e)
            }
        });
    
    match coll_result {
        Err(e) => Err(e),
        _ => Ok(())
    }
}

pub fn repeated<T>(vec: &mut Vec<T>, n: i32)
    where T: Copy
{
    let clone = vec.clone();
    for _ in 0..n {
        vec.extend(clone.iter());
    }
}
