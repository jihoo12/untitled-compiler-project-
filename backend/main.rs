struct IrArgs {
    reg1: i32,
    reg2: i32,
    scale: i32,
    index: i32,
    base: i32,
    displacement32: i32,
    displacement8: i8,
    isi8: bool,
    isdisp: bool,
    imm: i32,
    mem: String,
}
fn ir(prefix1: &str, prefix2: &str, command: &str,args: IrArgs) {
    match prefix1 {
         "s" => {
             match command {
                "mov" => {
                   
                },
             }
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