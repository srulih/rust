

fn prime(n: i32) -> bool {
    for i in 2..n {
        if n%i == 0 {
            return false;
        }
    }
    return true;
}

fn main() {

    let mut fo = Vec::new();
    for i in 2..100 {
        if prime(i) {
            fo.push(i);
        }
    }
    println!("{:?}", fo);

}
