use strut_macro::{Get, Set};

#[derive (Debug, Get, Set)]
struct TSet {
    #[Skip [Set, Get]] //v1 skip Set and Get
    v1: String,
    #[Trim]
    v2: String,
    #[NoChain] //v3 setter without chain call
    v3: String,
    #[Skip] //v4 skip all
    v4: bool,
}

#[cfg (test)]
mod tests {

    use super::TSet;

    #[test]
    fn test_t_set () {
        let mut t = TSet {
            v1: "a".to_string (),
            v2: "b".to_string (),
            v3: "c".to_string (),
            v4: false,
        };
        println!("{:?}", t);
        //t.set_v1 ("a1".to_string ()); //skipped
        t.set_v2 ("test string".to_string ());
        println!("{:?}", t);
        //println!("{:?}", t.get_v1 ()); //skipped
        println!("{:?}", t.get_v2 ());
        println!("{:?}", t.get_v3 ());
        t.set_v2 ("v2".to_string ()).set_v3 ("v3".to_string ()); //v2 可以链式调用
        t.set_v3 ("v3".to_string ()); //v3 no chain 不能链式调用
    }
}
