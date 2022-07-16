pub fn map<F, T1, T2>(input: Vec<T1>, mut function: F) -> Vec<T2> 
    where F: FnMut(T1) -> T2 {
    
    let mut out_vec = Vec::new();

    for i in input {
        out_vec.push(function(i));
    }

   out_vec
}

