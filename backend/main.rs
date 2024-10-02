struct IrArgs {
    reg1: i8,
    reg2: i8,
    scale: i8,
    index: i8,
    base: i8,
    displacement8: i8,
    isdisp: bool,
    imm: i32,
    mem: String,
}
fn ir(prefix1: &str, prefix2: &str, command: &str,args: IrArgs) {
    match prefix1 {
         "s" => {
             let sib;
             if !args.isdisp {
                 sib = 04 + (8 * args.reg2) + (4 * args.scale) + (8 * args.index) + (8 * args.base);
             }else {
                 sib = 44 + (8*args.reg2)+(4*args.scale)+(8*args.index)+(8*args.base)+args.displacement8;
             }
             match command {
                "mov" => {
                   let out = 4889+sib;
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
                 "s" => {
                     let sib;
                     if !args.isdisp {
                         sib = 04 + (8 * args.reg1) + (4 * args.scale) + (8 * args.index) + (8 * args.base);
                     }else {
                         sib = 44 + (8*args.reg1)+(4*args.scale)+(8*args.index)+(8*args.base)+args.displacement8;
                     }
                     match command {
                         "mov" => {
                             let out = 488B+sib;
                         },
                     }
                 },
                 "n" => {},
                 "i" => {},
                 "r" => {
                 
                 },
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