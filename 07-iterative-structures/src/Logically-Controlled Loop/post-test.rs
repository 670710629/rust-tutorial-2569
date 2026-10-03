//use std::io;
fn main(){
    
    //loop{
      //  let mut msg = String::new();
        //io::stdin()
          //  .read_line(&mut msg)
            //.expect("Failed input");

        // let numbers: i32 = match msg.trim().parse(){
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Please type a valid number!");
        //         continue;
        //     }
        // };

        // if numbers < 0{
        //     println!("Stopped! Entered negative numbers: {}",  numbers);
        //     break;
        // }    

        //println!("Input numbers: {}", numbers);
    //}

    // loop เป็นลูปแบบอนันต์ คล้ายๆกับ while loop ซึ่ง loop นี้เราต้องมาทำการดัก if เพื่อให้ได้ค่าที่ต้องการ ซึ่งถ้าเราไม่ได้ใใส่เงื่อนไขการหยุดก็ทำให้ loop ทำงานไปตลอด 

    // บรรทัดที่ 11-15 ถ้าเราทำการแปลง String --> int แล้วไม่มีการเช็ค Error Handling ทำให้เราไม่สามารถรับรู้ได้ว่า code นี้ผิดพลาดอะไรสำหรับมือใหม่หัดเขียน

    // use std::io ถ้าไม่ได้ใส่หรือ import เข้ามาทำให้ไม่สามารถรับค่าจากทาง keyboard ได้








    loop{
       let mut msg = String::new();
        io::stdin()
           .read_line(&mut msg)
            .expect("Failed input");

        let numbers: i32 = match msg.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a valid number!");
                continue;
            }
        };

        if numbers < 0{
            println!("Stopped! Entered negative numbers: {}",  numbers);
            break;
        }    

        println!("Input numbers: {}", numbers);
    }
}