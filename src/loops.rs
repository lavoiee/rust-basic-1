pub fn run() {
    let xs = vec!["lorem", "ipsum", "dolor"];

    let mut result = Vec::new();
    for i in 0..xs.len() {
        if result.len() <= 5 && xs[i].ends_with('m') {
            return result.push(xs[i]);
        }
    }
    //return result;

    let xs = vec!["lorem", "ipsum", "dolor"];
    xs.iter()
        .filter(|item| item.ends_with('m'))
        .take(5)
        .collect()

}