fn ir(prefix1: &str, prefix2: &str, command: &str,arg1: &str,arg2: &str) {
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