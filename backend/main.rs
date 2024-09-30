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
    let syntax = &s[word+1..];
    let slice = first_word(&syntax);
    let command = &syntax[..slice];
    let commandargs = &syntax[slice+1..];
    match prefix1 {
         "s" => {
             //prefix2= r
             //only register available 
             // 48 89 
             // 00 000(reg) 100 00(scale) 000(index) 000(base)
         },
         "n" => {
             match command {
                 "init" => {
                     println!("554889E5");
                 },
                 "ret" => {
                     println!("C3");
                 },
                 "clean" => {
                     println!("4889EC5D");
                 },
                 "syscall" => {
                     println!("0F05");
                 },
                 _ => {
                    println!("error syntax error");
                 }, //error
             }
             //prefix2 = n
             //enter ## assembly enter
             //ret ## assembly ret
             //leave ## assembly leave
             //syscall ## assembly syscall
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
                 "n" => {},
                 "i" => {},
                 "r" => {},
                 _ => {}, //error 
            }
         },
         "r-i" => {
             match prefix2 {
                 "n" => {},
                 "i" => {},
                 "r" => {},
                 _ => {}, //error 
             }
         },
         "m" => {
             match prefix2 {
                 "n" => {},
                 "i" => {},
                 "r" => {},
                 "r+i" => {},
                 "r-i" => {},
                 _ => {}, //error 
             }
         },
         _ => {}, //error 
    }
} 