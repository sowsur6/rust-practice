
fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut pyramid: Vec<String> = Vec::new();
    let mut num_spaces = 0;
    let mut num_chars = 0;


    for _ in 0..i {
    num_spaces += 1;
    num_chars += 1;
    let line = format!("{:width$}{}", "", v.repeat(num_chars), width = num_spaces);
    pyramid.push(line);
    }
    for _ in (1..num_spaces).rev() {
    num_spaces -= 1;
    num_chars -= 1;
    let line = format!("{:width$}{}", "", v.repeat(num_chars), width = num_spaces);
    pyramid.push(line);
    }

    pyramid
}

fn main() {
    let a = inv_pyramid(String::from("#"), 1);
    let b = inv_pyramid(String::from("a"), 2);
    let c = inv_pyramid(String::from(">"), 5);
    let d = inv_pyramid(String::from("&"), 8);

    for v in a.iter() {
        println!("{:?}", v);
    }
    for v in b.iter() {
        println!("{:?}", v);
    }
    for v in c.iter() {
        println!("{:?}", v);
    }
    for v in d.iter() {
        println!("{:?}", v);
    }
}

#[test]
fn it_works() {
    let data_sets = vec![
        vec![],
        vec![" #"],
        vec![" a", "  aa", " a"],
        vec![
            " >",
            "  >>",
            "   >>>",
            "    >>>>",
            "     >>>>>",
            "    >>>>",
            "   >>>",
            "  >>",
            " >",
        ],
        vec![
            " &",
            "  &&",
            "   &&&",
            "    &&&&",
            "     &&&&&",
            "      &&&&&&",
            "       &&&&&&&",
            "        &&&&&&&&",
            "       &&&&&&&",
            "      &&&&&&",
            "     &&&&&",
            "    &&&&",
            "   &&&",
            "  &&",
            " &",
        ],
    ];
    assert_eq!(inv_pyramid(String::from("#"), 0), data_sets[0]);
    assert_eq!(inv_pyramid(String::from("#"), 1), data_sets[1]);
    assert_eq!(inv_pyramid(String::from("a"), 2), data_sets[2]);
    assert_eq!(inv_pyramid(String::from(">"), 5), data_sets[3]);
    assert_eq!(inv_pyramid(String::from("&"), 8), data_sets[4]);
}