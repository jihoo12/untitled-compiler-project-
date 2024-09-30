fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn findprefix(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            return i;
        }
    }

    s.len()
}
fn ir(s: &String) {
    //i will add error handling 
    let word = first_word(s);
    let start_word = &s[..word];
    let colon = findprefix(s);
    let prefix1 = &start_word[..colon];
    let prefix2 = &start_word[colon+1..];
    match prefix1 {
         "s" => {
             match prefix2 {
                "n" =>{},
                "i" => {},
                "r" => {},
                "r+i" => {},
                "r-i" => {},
                "m" => {},
                _ => {}, //error 
             }
         },
         "n" => {
             match prefix2 {
                "s" => {},
                "n" => {},
                "i" => {},
                "r" => {},
                "r+i" => {},
                "r-i" => {},
                "m" => {},
                _ => {}, //error 
             }
         },
         "i" => {
             match prefix2 {
                 "s" => {},
                 "n" => {},
                 "i" => {},
                 "r" => {},
                 "r+i" => {},
                 "r-i" => {},
                 "m" => {},
                 _ => {}, //error 
             }
         },
         "r" => {
             match prefix2 {
                 "s" => {},
                 "n" => {},
                 "i" => {},
                 "r" => {},
                 "r+i" => {},
                 "r-i" => {},
                 "m" => {},
                 _ => {}, //error 
             } 
         },
         "r+i" => {
             match prefix2 {
                 "s" => {},
                 "n" => {},
                 "i" => {},
                 "r" => {},
                 "r+i" => {},
                 "r-i" => {},
                 "m" => {},
                 _ => {}, //error 
            }
         },
         "r-i" => {
             match prefix2 {
                 "s" => {},
                 "n" => {},
                 "i" => {},
                 "r" => {},
                 "r+i" => {},
                 "r-i" => {},
                 "m" => {},
                 _ => {}, //error 
             }
         },
         "m" => {
             match prefix2 {
                 "s" => {},
                 "n" => {},
                 "i" => {},
                 "r" => {},
                 "r+i" => {},
                 "r-i" => {},
                 "m" => {},
                 _ => {}, //error 
             }
         },
         _ => {}, //error 
    }
} 