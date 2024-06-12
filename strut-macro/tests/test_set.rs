use strut_macro::{Get, Mut, New, Set, With};

#[derive(Debug)]
struct Kkk {
    v1: String,
}

impl Default for Kkk {
    fn default() -> Self {
        todo!()
    }
}

impl Kkk {
    /// Creates a new [`Kkk`].
    fn new(v1: String) -> Self {
        Self { v1 }
    }

    /// Creates a new [`Kkk`].
    fn new_default() -> Self {
        Self { v1: "".to_owned() }
    }
}

#[derive(Debug, Set, Get, Mut, With, New)]
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
    // #[Trim]
    v6: &'a String,
    #[Trim]
    pub v7: String,
    #[Trim]
    #[NoChain]
    v8: &'a str,
    // #[Trim]
    // v9: Box<&'a String>,
}

#[cfg(test)]
mod tests {

    use crate::Kkk;

    use super::TSet;

    #[test]
    fn test_t_set() {
        let k = Kkk {
            v1: "v1".to_string(),
        };
        let _ = Kkk::new("v1".to_owned());
        let _ = Kkk::new_default();
        println!("{:?} {:?}", k, k.v1);
        //let binding = String::from ("this is v7 means the Box");
        let mut t = TSet {
            v1: "a".to_string(),
            v2: "b".to_string(),
            v3: "c".to_string(),
            //v4: false,
            //v5: "b",
            v6: &String::from("this is v6"),
            v7: String::from("this is v7 means the Box"),
            v8: "this is v8",
            //v9 : Box::new (&binding),
        };
        println!("{:?}", t);
        // t.set_v2();
        let _ = t.get_v6();
        //let _ = t.get_v1 ();
        t.set_v1("a1".to_string()); //skipped
        t.set_v2("test string".to_string());
        println!("{:?}", t);
        //println!("{:?}", t.get_v1 ()); //skipped
        //println!("{:?}", t.get_v2 ());
        //println!("{:?}", t.get_v3 ());
        t.set_v2("v2".to_string()).set_v3("v3".to_string()); //v2 可以链式调用
        t.set_v3("v3".to_string()); //v3 no chain 不能链式调用
        let _ = t.get_v7();
        let _ = t.get_v8();

        t.set_v2("   v2 has trim   ".to_string());
        println!("v2 has trim {:?}", t.get_v2());

        let f = "   v6 has tirm     ".to_string();
        t.set_v6(&f);
        println!("v6 has trim {:?}", t.get_v6());

        t.set_v7("   v7 has trim   ".to_string());
        println!("v7 has trim {:?}", t.get_v7());

        t.set_v8("   v8 has trim   ");
        println!("v8 has trim {:?}", t.get_v8());

        let binding = "v6".to_owned();
        let test_new = TSet::new(
            "v1".to_owned(),
            "v2".to_owned(),
            "v3".to_owned(),
            &binding,
            "v7".to_owned(),
            "v8",
        );
        println!("{:?} {:?} {:?}", file!(), line!(), test_new);
    }
}
