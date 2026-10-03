fn main(){
    let names = vec!["Bob", "Carol","Ted"];

    // for name in &names{
    //     println!("Names --> {}", name);
    // }

    // println!("===========================");

    // for name in names{
    //     println!("Names --> {}", name);
    // }

    // Common Mistake 
    for i in 0..3{
         println!("Names --> {}", names.get(i));
    }

    // .get(i) ใน Rust จะไม่คืนค่าสตริงออกมาตรงๆ แต่จะคืนค่าเป็นชนิดข้อมูล Option<&T> เพื่อความปลอดภัย
}