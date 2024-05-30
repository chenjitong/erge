use strut_macro::{Get, Set};

#[derive(Debug, Get, Set)]
struct TSet {
    #[Skip]
    #[Mut]
    v1: String,
    v2: String,
    v3: String,
}

#[cfg(test)]
mod tests {

    use super::TSet;

    #[test]
    fn test_t_set() {
        let mut t = TSet {
            v1: "a".to_string(),
            v2: "b".to_string(),
            v3: "c".to_string(),
        };
        println!("{:?}", t);
        t.set_v1("a1".to_string());
        println!("{:?}", t);
        println!("{:?}", t.get_v1());
        println!("{:?}", t.get_v2());
        println!("{:?}", t.get_v3());
    }
}
