pub fn clamp<T: Ord + Clone>(x: T, min_size: Option<T>, max_size: Option<T>) -> T{
    let x = x;
    let min_size = min_size.unwrap_or(x.clone());
    let max_size = max_size.unwrap_or(x.clone());

    if x < min_size{
        return min_size
    }
    else if x > max_size{
        return max_size
    }
    else{
        return x
    }
    

}