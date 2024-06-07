#[warn (unused_imports)]
use strut_macro::{Get, Mut, Set};

struct Kkk {
    v1: String,
}

#[derive (Debug, Set, Get, Mut)]
struct TSet<'a> {
    #[Skip [Get]] //v1 skip Set and Get
    v1: String,
    #[Trim]
    v2: String,
    #[NoChain] //v3 setter without chain call
    v3: String,
    // #[Skip [Set]] //v4 skip all
    //v4: bool,
    // #[Trim]
    //pub v5: &'a str,
    #[Trim]
    v6: &'a String,
    #[Trim]
    pub v7: Box<String>,
    // #[Trim]
    //v8 : Box<&'a str>,
    // #[Trim]
    //v9 : Box<&'a String>,
}

#[cfg (test)]
mod tests {

    use crate::Kkk;

    use super::TSet;

    #[test]
    fn test_t_set () {
        let k = Kkk {v1:"v1".to_string (),};
        //let binding = String::from ("this is v7 means the Box");
        let mut t = TSet {
            v1: "a".to_string (),
            v2: "b".to_string (),
            v3: "c".to_string (),
            //v4: false,
            //v5: "b",
            v6: &String::from ("this is v6"),
            v7 : Box::new ("this is v7 means the Box".to_string ()),
            //v8: Box::new ("this is v8"),
            //v9 : Box::new (&binding),
        };
        println!("{:?}", t);
        t.set_v2();
        let _ = t.get_v6 ();
        //let _ = t.get_v1 ();
        t.set_v1 ("a1".to_string ()); //skipped
        t.set_v2 ("test string".to_string ());
        println!("{:?}", t);
        //println!("{:?}", t.get_v1 ()); //skipped
        //println!("{:?}", t.get_v2 ());
        //println!("{:?}", t.get_v3 ());
        t.set_v2 ("v2".to_string ()).set_v3 ("v3".to_string ()); //v2 可以链式调用
        t.set_v3 ("v3".to_string ()); //v3 no chain 不能链式调用
        //let _ = t.get_v7 ().deref ();
        //let _ = *t.v7;
    }
}
