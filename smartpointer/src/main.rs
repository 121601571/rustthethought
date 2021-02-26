struct t1 {
    left: Option<Box<t1>>,
    right: Option<Box<t1>>,
    val: i32,
}


fn main() {
    let mut _a = t1 {
        left: None,
        right: None,
        val: 3,
    };
    let _b = t1 {
        left: None,
        right: None,
        val: 1,
    };
    let _c = t1 {
        left: None,
        right: None,
        val: 2,
    };
    _a.left = Some(Box::from(_b));
    _a.right = Some(Box::from(_c));

    println!("{}, {}", _a.val.to_string(), _a.left.unwrap().val.to_string());
}

