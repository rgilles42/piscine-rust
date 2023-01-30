use inv_pyramid::*;

fn main() {
    println!("{:?}", inv_pyramid(String::from("&"), 8));
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
