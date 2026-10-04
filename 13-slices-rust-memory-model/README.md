# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 13
> **Topic No.:** 13
> **Topic Name:** Slices & Rust Memory Model
> **ประเด็นหลักที่ควรครอบคลุม:** slices, &str, array/vector slices, stack vs heap และความสัมพันธ์กับ ownership

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นายปภังกร มงคลนรกิจ | 670710293 | `@670710293` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นางสาวศิริกานต์ หรุ่นมาบแค | 670710294 | `@670710294` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายสิรวิชญ์ เชี่ยวชาญ | 670710296 | `@670710296` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายวีรภัทร พุฒหอม | 670710336 | `@[กรอก GitHub username]` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

> แก้ไข GitHub Username ของแต่ละคนให้ตรงกับบัญชีจริงก่อนเริ่มทำงาน (ผู้สอนจะใช้คอลัมน์นี้เชิญเป็น collaborator ของ repository)

---

## 2. Learning Objectives

หลังจากศึกษา Topic นี้แล้ว ผู้เรียนสามารถ:

1. `[อธิบายแนวคิดสำคัญได้]`
2. `[เขียนโปรแกรม Rust ที่เกี่ยวข้องได้]`
3. `[วิเคราะห์พฤติกรรม/กฎของภาษาได้]`
4. `[เปรียบเทียบ Rust กับภาษาอื่นได้]`

---

## 3. Introduction

อธิบายว่า Topic นี้คืออะไร มีความสำคัญอย่างไร และใช้แก้ปัญหาอะไรในการเขียนโปรแกรม

`Topic นี้จะพูดถึงการแบ่งย่อยในตัวแปรต่างๆ ไม่ว่าจะเป้น List , Array , String หรืออื่นๆ และการจัดการ Memory ของภาษา Rust`

`เริ่มที่การจัดการ Memory หากไม่มีการจัดการ Memory แบบ Rust`
1) [ตัวแปรข้อมูลจะไม่ปลอดภัย เนื่องจากใครๆก็มาหยิบไปใช้ได้ แก้ข้อมูลได้เสมอ]
2) [หากไม่มี memory Model ตัวข้อมูลจะมั่วซั่วไปหมด เก็บที่ไหนไปเรื่อย และจะทิ้งเป็น Garbage ไว้ใน Ram ซึ่งจะทำให้เปลืองทรัพยากรมากๆ]
3) [Slicing ช่วยให้ลดการจองพื้นที่ Ram แบบไม่จำเป็นทิ้งไป]  

`ส่วนในพาร์ทของ Memory Model ใน Rust จะมี Stack + Heap`  
`โดย Memory Model : ชุดการจัดเก็บข้อมูล/ตัวแปร การใช้งาน การคืนความจำ โดยใช้หลัก Ownership , Borrowing  และ Lifetime`
1) `Stack ใช้ Concept แบบ LIFO`  
`จะเก็บค่าตัวแปรที่รู้ขนาดแน่นอนเท่านั้น เช่น integer,อาเรย์ที่กำหนดขนาดแล้ว`  
2) `Heap ใช้เก็บค่าตัวแปรที่ไม่ทราบขนาดแน่นอน ( เพิ่มขึ้นหรือลดลงได้ตอน Compile )`  
` โดยหลักการมันจะเก็บตัว pointer กับชื่อของตัวแปรไว้ที่ Stack แล้วให้ชี้มาที่ Heap ของค่านั้นๆ `  
` จะเก็บตัวแปรประเภท String , Dynamic Collection เป็นต้น `  


`ส่วนตัวของ Slices คือการแบ่งย่อยจากตัวเซตข้อมูลหลัก ไม่ว่าจะเป็น List , Array หรือ String เป็นต้น`  
`ที่ช่วยทำให้เกิด Zero - Heap Allocation หรือก็คือ ไม่เกิดการจองข้อมูลใน Heap เพิ่มเติม`  
`(แต่ยังคงเก็บใน stack นิดนึงนะ คือชื่อกับ Pointer เพื่อบ่งบอกว่าเราชี้ไปที่ Heap จุดไหน)`  

`เช่น เรามี List หรือ Array ที่มีข้อมูลเป็น [2,3,5,6,7] แต่เราต้องการตั้งแต่ตัวที่ 3 ( คือเลข 5 ) เป็นต้นไป`  
`เพื่อนำไปคำนวณต่อ ก็สามารถใช้การ slices เพื่อตัดแค่ข้อมูลที่ต้องการ แล้วนำมาใช้ต่อได้เลย`  


---

## 4. Key Concepts

### 4.1 `[Memory Model ของ Stack]`

**คำอธิบาย**

`[Stack สำหรับเก็บค่าที่เป็น Imutable หรือค่าที่ถูกฟิคขนาดไว้แล้ว ในช่วงเวลานั้นๆ]`

**ตัวอย่าง**

```rust
fn main(){
    let x = 60;
    let y = 7;
    let z = x + y;
}
```

**Explanation**

`[ตัว Memory Model ของภาษา rust จะเก็บค่า x และ y เข้าไปใน stack ก่อน และให้มันมีค่าเป็น 60 และ 7 ตามลำดับ]`  
`[จากนั้น Memory ของภาษา rust จะ อ่านค่า x และ y นำมาบวกกันแล้วเก็บเข้าไปในค่า z ของ stack]`  


---

### 4.2 `[Memory Model ของ Heap]`

`[Heap จะเป็นก้อนเก็บข้อมูลก้อนนึง สำหรับตัวแปรที่ยืดหยุ่นเรื่องขนาดระหว่างการคอมไพล์ โดยตัวแปรเหล่านั้นจะมีทั้งเก็บค่าไว้ที่ stack และ heap]`
`[โดยหลักๆจะแบ่งเป็น 2 ประเภท 1.เก็บค่าใน stack เป็น thin pointer , 2.เก็บค่าใน stack เป็น fat pointer เพื่อชี้ข้อมูลไปที่ ก้อนใน heap]`

```rust
fn main(){
    let s = String::from("Hello");
    let myVec = vec![1, 2, 3, 5];
}
```
**Explanation**

`เริ่มที่ตัว s จะสร้างก้อนheap ที่เก็บคำว่า ['H' , 'e' , 'l' , 'l' , 'o']ไว้ แล้วจะเก็บค่าใน stack เป็น pointer + len + capacity `
`โดย pointer จะชี้ไปที่ก้อน heap`  
`ส่วน myVec ก็จะทำงานในทำนองเดียวกัน`  

---

### 4.3 `[Slice]`

`[อธิบายแนวคิด]`

```rust
// Rust code
```

---



## 5. Important Syntax / Rules

| Syntax / Rule | Meaning | Example |
|---|---|---|
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |
| `[syntax/rule]` | `[ความหมาย]` | `[ตัวอย่าง]` |

### Important Rules

1. `[กฎสำคัญข้อที่ 1]`
2. `[กฎสำคัญข้อที่ 2]`
3. `[กฎสำคัญข้อที่ 3]`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code ทีละส่วนที่สำคัญ]`

---

### Example 2 — `[ชื่อ Example]`

**Purpose:** `[ต้องการสาธิตอะไร]`

```rust
fn main() {
    // Write your runnable Rust code here
}
```

**Expected Output**

```text
[expected output]
```

**Explanation**

`[อธิบาย code]`

---

## 7. Common Mistakes

### Mistake 1 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

### Mistake 2 — `[ชื่อข้อผิดพลาด]`

**Problem**

`[อธิบายปัญหา]`

**Incorrect Code**

```rust
// Incorrect example
```

**Correct Code**

```rust
// Correct example
```

**Why?**

`[อธิบายสาเหตุ]`

---

## 8. Exercises

> จัดทำแบบฝึกหัด **2 ข้อ** ที่สอดคล้องกับ Topic และมีระดับความยากเหมาะสม

### Exercise 1 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

### Exercise 2 — `[ชื่อโจทย์]`

**Problem**

`[เขียนโจทย์]`

**Hint**

`[คำใบ้]`

**Solution**

```rust
// Solution code
```

**Explanation**

`[อธิบายแนวทางแก้]`

---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

Rust ใช้ slice เพื่ออ้างอิงข้อมูลบางส่วนของ collection โดยไม่ต้องสร้าง collection ใหม่ขึ้นมา

รูปแบบที่พบบ่อยคือ

```rust
&collection[start..end]
```
หรือถ้าเริ่มจากตำแหน่งแรก
```rust
&collection[..end]
```

ตัวอย่าง
```rust
let numbers = [10, 20, 30, 40, 50];

let part = &numbers[1..4];

println!("{:?}", part);
```
ผลที่ได้คือ
```rust
[20, 30, 40]
```
สำหรับ String สามารถใช้
```rust
let text = String::from("Hello Rust");

let word = &text[0..5];
```
โดย `word` จะมีชนิดเป็น `&str` ซึ่งเป็น string slice

สำหรับ `Vec`
```rust
let numbers = vec![10, 20, 30, 40, 50];

let part = &numbers[1..4];
```
`part` จะเป็น slice ที่อ้างอิงข้อมูลบางส่วนของ `Vec` โดยไม่ได้สร้าง `Vec` ใหม่

ดังนั้น Syntax ของ Slice จะเน้นการใช้ `&` ร่วมกับช่วง `[start..end]` เพื่อสร้าง reference ไปยังข้อมูลเดิม

### 9.2 Semantics

Slice มีความหมายว่าเป็น ส่วนหนึ่งของข้อมูลเดิม ไม่ใช่ข้อมูลชุดใหม่

ตัวอย่าง

```rust
let data = [10, 20, 30, 40, 50];
let part = &data[1..4];
```
`part` หมายถึงข้อมูล `20, 30, 40` แต่ข้อมูลยังอยู่ใน `data` เหมือนเดิม

แนวคิดสำคัญคือ Slice เป็น borrowed reference จึงไม่ได้เป็นเจ้าของข้อมูล
```rust
let numbers = vec![10, 20, 30, 40];

let part = &numbers[1..3];
```
ในกรณีนี้ `part` ยืมข้อมูลจาก `numbers` มาใช้ชั่วคราว ดังนั้นไม่สามารถใช้ `part` หลังจากเจ้าของข้อมูลหมดอายุได้

อีกประเด็นหนึ่งคือการใช้ String slice ต้องระวังเรื่อง UTF-8 เพราะ `&str` ไม่ได้แบ่งข้อมูลตามตัวอักษรโดยตรง แต่ใช้ byte range
```rust
let text = String::from("Hello");
let part = &text[0..2];
```
กรณีนี้จะได้ `"He"` เพราะตัวอักษร ASCII ใช้ 1 byte ต่อตัว

แต่ String ที่มีภาษาไทยหรือตัวอักษร Unicode การตัด byte ผิดตำแหน่งอาจทำให้เกิด panic ได้ เพราะไม่ใช่ทุก byte position ที่เป็นขอบเขตของตัวอักษร

ดังนั้น Semantics ของ Slice คือ การอ้างอิงข้อมูลเดิมบางช่วงผ่าน borrowing โดยไม่โอน ownership

### 9.3 Type System

Rust มี Type System ที่ทำงานร่วมกับ Slice และ Ownership อย่างชัดเจน

ตัวอย่างชนิดที่เกี่ยวข้อง

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];

let slice: &[i32] = &arr[1..4];

let text: &str = "Hello Rust";

let vec: Vec<i32> = vec![1, 2, 3, 4, 5];

let vec_slice: &[i32] = &vec[1..4];
```
ความแตกต่างที่สำคัญคือ `[i32; 5]` เป็น Array ที่มีขนาดแน่นอน `&[i32]` เป็น Slice Reference ซึ่งขนาดของข้อมูลที่อ้างอิงสามารถเปลี่ยนได้ตามช่วงที่เลือก `&str` เป็น String Slice ใช้สำหรับอ้างอิงข้อมูล String ที่เป็น UTF-8 และ `Vec<i32>` เป็น Collection ที่สามารถเพิ่มหรือลดจำนวนสมาชิกได้ และเป็นเจ้าของข้อมูลของตัวเอง

ดังนั้น Rust จึงใช้ Type System เพื่อแยกให้ชัดว่าอะไรเป็นเจ้าของข้อมูล และอะไรเป็นเพียง reference ที่ยืมข้อมูลมาใช้

### 9.4 Memory / Resource Management

Rust มีการจัดการ Memory โดยใช้แนวคิด Ownership, Borrowing และ Lifetime แทนการใช้ Garbage Collector แบบภาษาอย่าง Java

ตัวอย่าง

```rust
let numbers = vec![10, 20, 30, 40, 50];
let part = &numbers[1..4];
```
ในตัวอย่างนี้ `numbers` เป็นเจ้าของ `Vec` และข้อมูลของ `Vec` โดยทั่วไปจะเก็บอยู่บน Heap

ส่วนตัวแปร `part` เป็น slice reference ที่ใช้ชี้ไปยังข้อมูลบางส่วนของ `numbers`

สำหรับ Array ที่มีขนาดคงที่ เช่น 
```rust
let numbers = [1, 2, 3, 4, 5];
```
ข้อมูลมักจะถูกเก็บบน Stack เมื่อ local variable ถูกสร้างขึ้นแบบปกติ

แต่ `Vec` จะเก็บตัวข้อมูลไว้บน Heap และตัวแปร `Vec` บน Stack จะเก็บข้อมูลสำหรับจัดการ buffer เช่น pointer, length และ capacity

เมื่อออกจาก Scope Rust จะเรียก `drop` และคืนทรัพยากรที่เจ้าของข้อมูลรับผิดชอบโดยอัตโนมัติ

จุดสำคัญคือ Slice ไม่ได้เป็นเจ้าของข้อมูล
```rust
let data = vec![1, 2, 3, 4];
let part = &data[1..3];
```
เมื่อ `data` หมด Scope ข้อมูลที่ `part` อ้างอิงอยู่ก็ไม่สามารถถูกใช้งานต่อได้ เพราะ Rust ใช้ Borrow Checker ป้องกันไม่ให้เกิดการอ้างอิงข้อมูลที่หมดอายุแล้ว

### 9.5 Abstraction / Other PPL Concepts

แนวคิด Slice ของ Rust เชื่อมโยงกับ PPL หลายเรื่อง ได้แก่ Abstraction, Scope, Binding และ Lifetime

**Abstraction**

Slice ช่วยให้โปรแกรมเมอร์สนใจเฉพาะ “ข้อมูลช่วงที่ต้องการ” โดยไม่ต้องจัดการรายละเอียดการสร้าง collection ใหม่

```rust
fn print_data(data: &[i32]) {

    println!("{:?}", data);

}
```
ฟังก์ชันนี้สามารถรับได้ทั้ง
```rust
let arr = [1, 2, 3, 4];
let vec = vec![5, 6, 7, 8];
```
เพราะทั้ง Array และ Vector สามารถถูกยืมมาเป็น `&[i32]` ได้

จึงเป็นการสร้าง Abstraction ให้ฟังก์ชันทำงานกับ “ลำดับของข้อมูล” โดยไม่ต้องสนใจว่าเจ้าของข้อมูลจริง ๆ เป็น Array หรือ Vector

**Scope**

ตัวแปรและ reference จะใช้งานได้ภายใน Scope ที่กำหนด

```rust
let data = vec![1, 2, 3];

{
    let part = &data[0..2];
}
```
`part` จะมี Scope อยู่ภายใน block เท่านั้น

**Binding**

Rust ใช้ `let` สำหรับ Binding ตัวแปรกับค่า

```rust
let data = vec![1, 2, 3, 4];
let part = &data[1..3];
```
ในที่นี้ `part` ถูก Binding ให้เป็น reference ไปยังส่วนหนึ่งของ `data`

**Lifetime**

Slice ต้องไม่สามารถมีอายุยาวกว่า Owner ของข้อมูล

```rust
let data = vec![1, 2, 3];

let part = &data[0..2];
```
Lifetime ของ `part` จึงขึ้นอยู่กับ `data`

แนวคิดนี้ช่วยให้ Rust ตรวจสอบปัญหาเกี่ยวกับ Memory ตั้งแต่ Compile Time

### 9.6 Why Rust?

Rust ออกแบบมาให้สามารถควบคุม Memory ได้ดี แต่ในขณะเดียวกันก็พยายามป้องกันปัญหาที่เกิดจากการจัดการ Memory

สำหรับ Slice จุดเด่นคือสามารถ เข้าถึงข้อมูลบางส่วนโดยไม่ต้อง Copy ข้อมูลใหม่

ตัวอย่าง
```rust
let data = vec![10, 20, 30, 40, 50];

let part = &data[1..4];
```
`part` เพียงแค่อ้างอิงข้อมูลเดิม ทำให้ลดการสร้างข้อมูลซ้ำและช่วยเรื่อง Performance

ดังนั้น Rust ให้ความสำคัญกับ Memory Safety โดยใช้ Ownership, Borrowing และ Lifetime ในการจัดการหน่วยความจำ แนวคิดเหล่านี้ช่วยป้องกันปัญหาที่พบบ่อยในภาษาแบบ Manual Memory Management เช่น dangling reference คือ reference ที่ชี้ไปยังข้อมูลที่หมดอายุแล้ว, use-after-free คือการใช้ข้อมูลหลังจาก Memory ถูกคืนไปแล้ว, double free คือการคืน Memory เดิมมากกว่าหนึ่งครั้ง และ invalid memory access คือการเข้าถึง Memory ในตำแหน่งที่ไม่ถูกต้อง เช่น การเข้าถึง Array เกินขอบเขต

ในกรณีของ Slice จะเห็นว่า Slice ไม่ได้เป็นเจ้าของข้อมูล แต่เป็นการ Borrow ข้อมูลจาก Owner ดังนั้น Rust จึงตรวจสอบความสัมพันธ์ระหว่าง Slice กับข้อมูลต้นทางก่อน Compile เพื่อป้องกันการอ้างอิงข้อมูลที่ไม่มีอยู่แล้ว และช่วยให้โปรแกรมมีทั้ง Memory Safety และ Performance

---

## 10. Rust vs. Other Language

**Comparison Language:** Python / C / C++ / Java

| Aspect | Rust | Python | C | C++ | Java |
|---|---|---|---|---|---|
| Syntax | ใช้ `&[T]` สำหรับ slice และ `&str` สำหรับ string slice เช่น `&data[1..4]` | ใช้ slicing เช่น `data[1:4]` และ `text[1:4]` | ไม่มี slice type โดยตรง มักใช้ pointer ร่วมกับ length เช่น `int *p = &data[1]` | มี `std::span` และ `std::string_view` สำหรับมองข้อมูลบางช่วงโดยไม่เป็นเจ้าของ | Array ไม่มี slice โดยตรง มักใช้ `Arrays.copyOfRange()` ซึ่งสร้าง array ใหม่ หรือ `List.subList()` สำหรับ view |
| Semantics / Behavior | Slice เป็น borrowed reference ไปยังข้อมูลเดิม ไม่ได้สร้างข้อมูลใหม่ และไม่เป็นเจ้าของข้อมูล | Slice ของ `list`, `str` โดยทั่วไปได้ข้อมูลใหม่ ส่วนตัวแปรเดิมยังเป็นเจ้าของข้อมูลของตัวเอง | Pointer เพียงชี้ไปยังตำแหน่ง Memory โปรแกรมเมอร์ต้องจัดการว่า pointer และ length ถูกต้อง | `span` / `string_view` เป็น non-owning view จึงไม่เป็นเจ้าของข้อมูล ต้องระวัง lifetime ของข้อมูลต้นทาง | การ slice array แบบ `copyOfRange()` เป็นการ Copy ข้อมูล ส่วน `subList()` เป็น view ที่อ้างอิง List เดิม |
| Type System | Static + Strong typing มีชนิดชัดเจน เช่น `&[i32]`, `&str`, `Vec<i32>` | Dynamic typing ชนิดของตัวแปรถูกตรวจสอบตอน Runtime | Static typing แต่มี pointer และ implicit conversion หลายกรณี | Static + Strong typing มี pointer, reference, `span`, `string_view` | Static + Strong typing ไม่มี pointer arithmetic แบบ C/C++ |
| Memory Management | ใช้ Ownership, Borrowing และ Lifetime โดยไม่มี Garbage Collector | จัดการ Memory อัตโนมัติตาม implementation เช่น reference counting/garbage collection | Manual memory management เช่น `malloc()` / `free()` | ใช้ RAII, destructor และ smart pointers ช่วยจัดการ Memory แต่ยังสามารถใช้ raw pointer ได้ | ใช้ Garbage Collector เป็นหลัก Object และ Array อยู่บน Heap ส่วน local references/stack frames อยู่ใน Stack |
| Safety | เน้น Memory Safety ตั้งแต่ Compile Time เช่น ป้องกัน dangling reference และ use-after-free | มีการจัดการ Memory อัตโนมัติและตรวจสอบหลายอย่างตอน Runtime | Safety ต่ำกว่า เพราะสามารถเกิด dangling pointer, buffer overflow, use-after-free ได้ | ปลอดภัยกว่า C ในหลายด้านเมื่อใช้ RAII/modern C++ แต่ raw pointer และ lifetime ยังทำให้เกิดปัญหาได้ | มี bounds checking, ไม่มี pointer arithmetic และใช้ GC จึงลดปัญหา Memory บางประเภท |

### Rust Example

Slice ของ Vector
```rust
fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let part = &numbers[1..4];

    println!("{:?}", part);
}
```
ผลลัพธ์
```rust
[20, 30, 40]
```
ตรงนี้ `part` ไม่ได้สร้าง `Vec` ใหม่ แต่เป็น Slice ที่ ยืมข้อมูลจาก `numbers`

ดังนั้น `numbers` เป็น Owner ส่วน `part` เป็น Borrower

### Rust `&str` Example
```rust
fn main() {
    let text = "Hello Rust";

    let part: &str = &text[0..5];

    println!("{}", part);
}
```
ผลลัพธ์
```rust
Hello
```
`&str` เป็น String Slice ที่ ไม่ได้เป็นเจ้าของ String แต่เป็น reference ไปยังข้อมูล String ที่มีอยู่แล้ว

จุดที่ต้องระวังคือ Rust String ใช้ UTF-8 ดังนั้น `&str` ใช้ byte range ไม่ใช่ตำแหน่งตัวอักษรแบบที่มองเห็น

### Python Example

Python มี slicing โดยตรงและเขียนง่ายมาก
```python
numbers = [10, 20, 30, 40, 50]

part = numbers[1:4]

print(part)
```
ผลลัพธ์
```python
[20, 30, 40]
```
แต่พฤติกรรมต่างจาก Rust เพราะการ slice list แบบนี้โดยทั่วไปจะสร้าง list ใหม่

จึงสามารถมองได้ประมาณว่า
```
numbers → [10,20,30,40,50]

part    → [20,30,40] (object ใหม่)
```
Python จึงใช้งานง่ายกว่า แต่ไม่มีแนวคิด Ownership/Borrowing แบบ Rust ที่ Compiler ตรวจสอบความสัมพันธ์เหล่านี้

### C Example

C ไม่มี Slice เป็น Type โดยตรง ดังนั้นมักใช้ Pointer + Length
```c
#include <stdio.h>

int main() {
    int numbers[] = {10, 20, 30, 40, 50};

    int *part = &numbers[1];
    int length = 3;

    for (int i = 0; i < length; i++) {
        printf("%d ", part[i]);
    }

    return 0;
}
```
ผลลัพธ์
```c
20 30 40
```
ตรงนี้
```c
int *part = &numbers[1];
int length = 3;
```
ต้องใช้ Pointer บอกว่าเริ่มตรงไหน และต้องใช้ length บอกว่ามีข้อมูลกี่ตัว

C ไม่ได้ตรวจสอบให้ว่า Pointer ยังถูกต้องหรือข้อมูลยังมีชีวิตอยู่

จึงมีโอกาสเกิดปัญหา เช่น 

- Dangling Pointer คือ Reference ที่ยังชี้ไปยังข้อมูลเดิม แต่ข้อมูลนั้นถูกทำลายหรือหมดอายุไปแล้ว
- Use-After-Free คือการ นำ Memory ที่ถูกคืนหรือถูกปล่อยไปแล้วกลับมาใช้งานอีก
- Invalid Memory Access คือการ เข้าถึง Memory ในตำแหน่งที่ไม่ควรเข้าถึง


### C++ Example

C++ มี std::span ซึ่งมีแนวคิดใกล้กับ Rust Slice มาก
```cpp
#include <iostream>
#include <vector>
#include <span>

int main() {
    std::vector<int> numbers = {10, 20, 30, 40, 50};

    std::span<int> part(numbers.data() + 1, 3);

    for (int x : part) {
        std::cout << x << " ";
    }

    return 0;
}
```
ผลลัพธ์
```cpp
20 30 40
```
std::span เป็น non-owning view หมายความว่าไม่ได้เป็นเจ้าของข้อมูล เช่นเดียวกับ Slice ของ Rust ในแง่แนวคิด

แต่ความแตกต่างสำคัญคือ C++ ไม่ได้มี Ownership/Borrow Checker แบบ Rust ดังนั้นโปรแกรมเมอร์ยังต้องระวังว่า numbers ต้องมีอายุยาวพอที่จะให้ part ใช้งาน

C++ ยังมี std::string_view สำหรับมองบางส่วนของ String โดยไม่ Copy ข้อมูล

### Java Example

Java ไม่มี Slice Type สำหรับ Array โดยตรง

ตัวอย่าง
```java
import java.util.Arrays;

public class Main {
    public static void main(String[] args) {
        int[] numbers = {10, 20, 30, 40, 50};

        int[] part = Arrays.copyOfRange(numbers, 1, 4);

        System.out.println(Arrays.toString(part));
    }
}
```
ผลลัพธ์
```java
[20, 30, 40]
```
แต่ Arrays.copyOfRange() สร้าง Array ใหม่
```
numbers → [10,20,30,40,50]

part    → [20,30,40] (Array ใหม่)
```
จึงต่างจาก Rust
```rust
let part = &numbers[1..4];
```
ที่ part เป็น reference ไปยังข้อมูลเดิม

สำหรับ Collection อย่าง List Java สามารถใช้
```java
List<Integer> part = numbers.subList(1, 4);
```
ซึ่งมีลักษณะเป็น view ของ List เดิม มากกว่าการ Copy ทั้งชุด

### Analysis

ความแตกต่างที่สำคัญระหว่าง Rust กับภาษาอื่นอยู่ที่ แนวคิดในการจัดการ Memory และการออกแบบ Slice โดย Rust พยายามให้โปรแกรมสามารถเข้าถึงข้อมูลเดิมได้โดยไม่ต้อง Copy ข้อมูล แต่ยังคงตรวจสอบความปลอดภัยของการอ้างอิงตั้งแต่ Compile Time

Rust vs. Python

Python ใช้ Syntax ของ Slice ที่ง่าย เช่น `data[1:4]` และจัดการ Memory ให้อัตโนมัติ ทำให้เขียนโปรแกรมได้สะดวก แต่ไม่ได้บังคับให้โปรแกรมเมอร์ระบุ Ownership หรือความสัมพันธ์ของ Reference แบบ Rust การออกแบบของ Rust จึงเน้นให้ผู้เขียนโปรแกรมควบคุมการยืมข้อมูลได้ชัดเจนขึ้น เพื่อให้เกิด Memory Safety โดยไม่ต้องพึ่ง Garbage Collector

Rust vs. C

C ไม่มี Slice type โดยตรง และมักใช้ Pointer กับ Length แทน ทำให้ควบคุม Memory ได้ละเอียดและมี Overhead ต่ำ แต่โปรแกรมเมอร์ต้องรับผิดชอบเรื่อง Pointer, Lifetime และขอบเขตของข้อมูลเอง Rust ออกแบบมาเพื่อลดปัญหาเหล่านี้ โดยให้ Slice เช่น `&[T]` เป็น Borrowed Reference และใช้ Ownership กับ Borrow Checker ตรวจสอบว่าการอ้างอิงยังถูกต้องอยู่

Rust vs. C++

C++ มีแนวคิดที่ใกล้ Rust มากขึ้น เช่น `std::span` และ `std::string_view` ซึ่งใช้เป็นมุมมองไปยังข้อมูลเดิมโดยไม่ต้อง Copy แต่ C++ ยังเปิดให้ใช้ Pointer และจัดการ Lifetime ได้หลายรูปแบบ ขณะที่ Rust ออกแบบกฎ Ownership และ Lifetime ให้เป็นส่วนหนึ่งของ Type System และให้ Compiler ตรวจสอบความสัมพันธ์ของ Reference อย่างเข้มงวดกว่า

Rust vs. Java

Java เน้นการจัดการ Memory อัตโนมัติด้วย Garbage Collector และไม่มี Pointer Arithmetic แบบ C/C++ ทำให้การใช้งาน Memory ค่อนข้างง่าย แต่การออกแบบของ Rust เลือกใช้ Ownership และ Lifetime แทน Garbage Collector เพื่อให้สามารถควบคุมทรัพยากรได้ละเอียดและคาดเดาได้มากขึ้น โดยยังรักษา Memory Safety ไว้

เหตุผลด้านการออกแบบภาษา

แนวคิดของ Rust ในเรื่อง Slice ถูกออกแบบให้ตอบโจทย์ 2 อย่างพร้อมกัน คือ Performance และ Safety

ใน Rust Slice คือการอ้างอิงไปยังข้อมูลบางส่วนของข้อมูลเดิม โดยไม่จำเป็นต้องสร้างหรือ Copy ข้อมูลชุดใหม่ขึ้นมา ทำให้ประหยัด Memory และช่วยให้การทำงานมีประสิทธิภาพมากขึ้น อย่างไรก็ตาม Slice ไม่ได้เป็นเจ้าของข้อมูลนั้นเอง แต่เป็นเพียงการ Borrow ข้อมูลจากตัวแปรที่เป็น Owner ดังนั้น Rust จึงใช้แนวคิด Ownership, Borrowing และ Lifetime เพื่อควบคุมว่าข้อมูลสามารถถูกอ้างอิงและใช้งานได้นานแค่ไหน โดย Compiler จะตรวจสอบกฎเหล่านี้ตั้งแต่ Compile Time เพื่อป้องกันปัญหาที่เกี่ยวกับ Memory เช่น การอ้างอิงข้อมูลที่หมดอายุหรือการใช้ข้อมูลผิดช่วงเวลา

ดังนั้น จุดเด่นของ Rust ไม่ได้อยู่ที่การมี Slice เพียงอย่างเดียว แต่คือการออกแบบให้ การอ้างอิงข้อมูลที่มีประสิทธิภาพสามารถใช้งานร่วมกับระบบ Ownership ได้อย่างปลอดภัย ซึ่งเป็นแนวคิดที่แตกต่างจาก Python และ Java ที่เน้นการจัดการ Memory อัตโนมัติ และแตกต่างจาก C/C++ ที่ให้อิสระในการจัดการ Memory มากกว่า

---

## 11. Teach Your Topic

การนำเสนอมีสมาชิก **4 คน คนละประมาณ 5 นาที**

| Member | Responsibility | Time |
|---|---|---:|
| Member 1 | Concept + Short Code Illustration | 5 min |
| Member 2 | Detailed Code + Live Demo | 5 min |
| Member 3 | Rust vs Other Language + PPL Analysis | 5 min |
| Member 4 | Exercises + Common Mistakes + Challenge | 5 min |

### Individual Contribution

**Member 1**

`[สิ่งที่รับผิดชอบ]`

**Member 2**

`[สิ่งที่รับผิดชอบ]`

**Member 3**

`[สิ่งที่รับผิดชอบ]`

**Member 4**

`[สิ่งที่รับผิดชอบ]`

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. `[The Rust Programming Language — Rust Book]`
2. `[Rust by Example / Rust Reference]`
3. `[Official documentation ที่เกี่ยวข้องกับ Topic]`
4. `[แหล่งอ้างอิงเพิ่มเติม]`

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| `[เช่น ChatGPT]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |
| `[AI tool]` | `[ใช้เพื่ออะไร]` | `[ตรวจสอบอย่างไร]` |

### Declaration

- [ ] Code ทุกส่วนที่นำเสนอได้รับการ Compile และทดสอบแล้ว
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ที่นำเสนอได้
- [ ] ตรวจสอบข้อมูลจากแหล่งอ้างอิงที่น่าเชื่อถือแล้ว
- [ ] ระบุการใช้ AI อย่างโปร่งใส

**รายละเอียดการใช้ AI**

`[อธิบายว่าใช้ AI ในขั้นตอนใด และสมาชิกตรวจสอบผลลัพธ์อย่างไร]`

---

## 14. GitHub Contribution

| Member | Issues | Commits | Pull Requests | Code Reviews | Contribution |
|---|---:|---:|---:|---:|---|
| Member 1 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 2 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 3 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |
| Member 4 | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[จำนวน]` | `[รายละเอียด]` |

### Teamwork Reflection

**How did your team collaborate?**

`[อธิบายกระบวนการทำงานร่วมกัน]`

**Problems encountered**

`[ปัญหาที่พบ]`

**How did you solve them?**

`[วิธีแก้ปัญหา]`

---

## 15. Final Checklist

- [ ] Learning Objectives ครบ 3–4 ข้อ
- [ ] Key Concepts ครบถ้วน
- [ ] Syntax / Rules
- [ ] Runnable Code Examples
- [ ] Code Compile และ Run ได้จริง
- [ ] Common Mistakes
- [ ] Exercises 2 ข้อ พร้อม Solutions
- [ ] PPL Perspective
- [ ] Rust vs Other Language
- [ ] References อย่างน้อย 4 แหล่ง
- [ ] AI Usage Declaration
- [ ] GitHub Contribution
- [ ] สมาชิกทั้ง 4 คนมีส่วนร่วม
- [ ] สมาชิกทั้ง 4 คนพร้อมนำเสนอคนละ 5 นาที
- [ ] สมาชิกทุกคนสามารถอธิบาย Code ของกลุ่มได้

---

## Submission Information

**Repository:** `[GitHub repository URL]`

**Chapter Path:** `[เช่น chapters/01-introduction/]`

**Final PR:** `#[PR number]`

**Submitted by:** `[Group XX]`

**Date:** `[YYYY-MM-DD]`
