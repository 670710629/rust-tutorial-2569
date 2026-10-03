use std::io;
fn main(){
    let  number: i32 = 0;

    while number >= 0 {
        let  msg = String::new();
        println!("Enter a number (negative number to stop):");

        io::stdin()
            .read_line(msg)
            .expect("Failed input");

        // number = match msg.trim().parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         println!("Please type a valid number!");
        //         0
        //     }
        // };
        if number > 0{
            println!("Input = {}", number);
        }
    }

    println!("Stopped! You entered negative number: {}", number);
    
    // ข้อผิดพลาด การรับค่าจากผู้ใช้ที่เป็นตัวเลข ต้องรับค่าเป็น String ก่อน เพราะภาษา Rust ไม่มีตัวที่รับค่าเฉพาะตัวเลขต้องทำการรับค่าเป็นสตริงแล้วค่อยแปลงมาเป็น int

    // {} ถ้าเราไม่ได้ใส่ ค่าที่เราจะปริ้นออกมามันจะไม่ออก สัญลักษณ์นี้เป็นตัวแทนในการแสดงผลตัวแปรออกมา

    // mut ถ้าเราไม่ได้ทำการใส่หน้าตัวแปร ตัวแปรนั้นจะเป็น const



    //  let mut number: i32 = 0;

    // while number >= 0 {
    //     let mut msg = String::new();
    //     println!("Enter a number (negative number to stop):");

    //     io::stdin()
    //         .read_line(&mut msg)
    //         .expect("Failed input");

    //     number = match msg.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => {
    //             println!("Please type a valid number!");
    //             0
    //         }
    //     };
    //     if number > 0{
    //         println!("Input = {}", number);
    //     }
    // }

    // println!("Stopped! You entered negative number: {}", number);

}