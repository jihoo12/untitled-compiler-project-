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
    let out;
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
                   out = 4889+sib;
                },
             }
         },
         "n" => {
             match command {
                 "init" => {
                     out = 554889E5
                 },
                 "ret" => {
                     out = C3;
                 },
                 "clean" => {
                     out = 4889EC5D;
                 },
                 "syscall" => {
                     out = 0F05;
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
                             out = 488B+sib;
                         },
                     }
                 },
                 "n" => {},
                 "i" => {},
                 "r" => {
                     match command {
                         "mov" => {
                            out = 4889C0+args.reg1+(8*args.reg2);
                         },
                     }
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
             //prefix2=n
         },
         _ => {}, //error 
    }
} 