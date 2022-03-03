use crate::libs::utils::SearchExpression;

pub fn pattern1(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    let symbol_use = symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    for x in 0..3 {
        let dork = &format!("{} {} {} {} inurl:= AND {}\n", kw1, symbol_use, kw2, symbol_use, ext_vec[x]);
        let dork_to_vec = dork.to_string();
        dork_vec.push(dork_to_vec);
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern2(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    let symbol_use = symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    for x in 0..3 {
        let dork = &format!("inurl:= AND {} {} {} AND {}\n", kw1, symbol_use, kw2, ext_vec[x]);
        let dork_to_vec = dork.to_string();
        dork_vec.push(dork_to_vec);
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern3(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    let symbol_use = symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    for x in 0..3 {
        let dork = &format!("{} {} {} / inurl= / {}\n", kw1, symbol_use, kw2, ext_vec[x]);
        let dork_to_vec = dork.to_string();
        dork_vec.push(dork_to_vec);
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern4(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    let symbol_use = symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    for x in 0..3 {
        let dork = &format!("inurl= / {} {} {} / {}\n", kw1, symbol_use, kw2, ext_vec[x]);
        let dork_to_vec = dork.to_string();
        dork_vec.push(dork_to_vec);
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern5(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    let symbol_use = symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    for x in 0..3 {
        let dork = &format!("{} {} {} {} inurl:= AND {}\n", kw2, symbol_use, kw1, symbol_use, ext_vec[x]);
        let dork_to_vec = dork.to_string();
        dork_vec.push(dork_to_vec);
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern6(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    let symbol_use = symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    for x in 0..3 {
        let dork = &format!("inurl:= AND {} {} {} AND {}\n", kw2, symbol_use, kw1, ext_vec[x]);
        let dork_to_vec = dork.to_string();
        dork_vec.push(dork_to_vec);
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern7(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    let symbol_use = symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    for x in 0..3 {
        let dork = &format!("{} {} {} / inurl= / {}\n", kw2, symbol_use, kw1, ext_vec[x]);
        let dork_to_vec = dork.to_string();
        dork_vec.push(dork_to_vec);
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern8(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    let symbol_use = symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    for x in 0..3 {
        let dork = &format!("inurl= / {} {} {} / {}\n", kw2, symbol_use, kw1, ext_vec[x]);
        let dork_to_vec = dork.to_string();
        dork_vec.push(dork_to_vec);
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern9(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    if !kw1.contains(char::is_whitespace) {
        for x in 0..3 {
            let dork = &format!("({}) AND ({} | inurl:=) AND ({})\n", kw1, kw2, ext_vec[x]);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    } else {
        let kw1_split:Vec<_> = kw1.split([' '].as_ref()).collect();
        for x in 0..3 {
            let dork = &format!("({} | {}) AND ({} | inurl:=) AND ({})\n", kw1_split[0], kw1_split[1], kw2, ext_vec[x]);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern10(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    if !kw1.contains(char::is_whitespace) {
        for x in 0..3 {
            let dork = &format!("({}) / ({} | inurl:=) / ({})\n", kw1, kw2, ext_vec[x]);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    } else {
        let kw1_split:Vec<_> = kw1.split([' '].as_ref()).collect();
        for x in 0..3 {
            let dork = &format!("({} | {}) / ({} | inurl:=) / ({})\n", kw1_split[0], kw1_split[1], kw2, ext_vec[x]);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern11(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    if !kw1.contains(char::is_whitespace) {
        for x in 0..3 {
            let dork = &format!("({}) AND ({}) | inurl:= AND {}\n", kw1, kw2, ext_vec[x]);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    } else {
        let kw1_split:Vec<_> = kw1.split([' '].as_ref()).collect();
        for x in 0..3 {
            let dork = &format!("({} | {}) AND ({}) | inurl:= AND {}\n", kw1_split[0], kw1_split[1], kw2, ext_vec[x]);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern12(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    if !kw1.contains(char::is_whitespace) {
        for x in 0..3 {
            let dork = &format!("({}) / ({}) | inurl:= / {}\n", kw1, kw2, ext_vec[x]);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    } else {
        let kw1_split:Vec<_> = kw1.split([' '].as_ref()).collect();
        for x in 0..3 {
            let dork= &format!("({} | {}) / ({}) | inurl:= / {}\n", kw1_split[0], kw1_split[1], kw2, ext_vec[x]);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern13(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    if !kw1.contains(char::is_whitespace) {
        for x in 0..3 {
            let dork = &format!("({}) AND ({}) AND ({}) AND (inurl:=)\n", ext_vec[x], kw1, kw2);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    } else {
        let kw1_split:Vec<_> = kw1.split([' '].as_ref()).collect();
        for x in 0..3 {
            let dork = &format!("({}) AND ({} | {}) AND ({}) AND (inurl:=)\n", ext_vec[x], kw1_split[0], kw1_split[1], kw2);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern14(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    if !kw1.contains(char::is_whitespace) {
        for x in 0..3 {
            let dork= &format!("({}) / ({}) / ({}) / (inurl:=)\n", ext_vec[x], kw1, kw2);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    } else {
        let kw1_split:Vec<_> = kw1.split([' '].as_ref()).collect();
        for x in 0..3 {
            let dork= &format!("({}) / ({} | {}) / ({}) / (inurl:=)\n", ext_vec[x], kw1_split[0], kw1_split[0], kw2);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern15(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    if !kw1.contains(char::is_whitespace) {
        for x in 0..3 {
            let dork = &format!("{} AND ({}) AND ({}) AND inurl:=\n", ext_vec[x], kw1, kw2);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    } else {
        let kw1_split:Vec<_> = kw1.split([' '].as_ref()).collect();
        for x in 0..3 {
            let dork = &format!("{} AND ({} | {}) AND ({}) AND inurl:=\n", ext_vec[x], kw1_split[0], kw1_split[1], kw2);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    }
    //println!("{:?}", dork_vec);
    dork_vec
}

pub fn pattern16(kw1: &String, kw2: &String) -> Vec<String> {
    let symbol = SearchExpression::search_exp();
    symbol.get_search_exp();
    let ext_vec = vec!["ext:aspx".to_string(),
                       "ext:php".to_string(),
                       "ext:asp".to_string()];
    let mut dork_vec = vec![];
    if !kw1.contains(char::is_whitespace) {
        for x in 0..2 {
            let dork = &format!("{} / ({}) / ({}) / inurl:=\n", ext_vec[x], kw1, kw2);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    } else {
        let kw1_split:Vec<_> = kw1.split([' '].as_ref()).collect();
        for x in 0..2 {
            let dork = &format!("{} / ({} | {}) / ({}) / inurl:=\n", ext_vec[x], kw1_split[0], kw1_split[1], kw2);
            let dork_to_vec = dork.to_string();
            dork_vec.push(dork_to_vec);
        }
    }
    //println!("{:?}", dork_vec);
    dork_vec
}