# Rust Tutorial Project — Principles of Programming Languages

> **กลุ่มที่:** 01
> **Topic No.:** 01
> **Topic Name:** Introduction to Rust & Programming Paradigms
> **ประเด็นหลักที่ควรครอบคลุม:** ความเป็นมาของ Rust, จุดเด่น, แนวคิดของภาษา, Rust กับ Imperative / OOP / Functional Programming

---

## 1. Members

| # | Name | Student ID | GitHub Username | Main Responsibility |
|---|---|---|---|---|
| 1 | นางสาวขวัญฐิตา การดี | 650710069 | `@[กรอก GitHub username]` | Concept + Short Code Illustration (สรุปแนวคิดหลัก + โค้ดตัวอย่างสั้น) |
| 2 | นายคีญภัสน์ แก้วผ่อง | 660710072 | `@[กรอก GitHub username]` | Detailed Code + Live Demo (โค้ดเชิงลึก + สาธิตสด) |
| 3 | นายจักรภพ ภูมิพัฒน์ | 660710073 | `@660710073` | Rust vs Other Language + PPL Analysis (เปรียบเทียบภาษา + วิเคราะห์เชิง PPL) |
| 4 | นายณทรรศน์ พรหมประดิษฐ์ | 660710079 | `@660710079` | Exercises + Common Mistakes + Challenge (แบบฝึกหัด + ข้อผิดพลาดที่พบบ่อย + คำถามท้าทาย) |

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

`[เขียนเนื้อหาที่นี่ — ใช้โครงสร้างเดียวกับ rust_tutorial_template.md ฉบับเต็มที่ผู้สอนแจกให้]`

---

## 6. Runnable Code Examples

> **ข้อกำหนด:** Code ทุกตัวต้อง Compile และ Run ได้จริงก่อนนำมาใส่ในเอกสาร

### Example 1 — โจทย์เดียว 3 แนวทาง (Imperative, Functional, OOP-style)

**Purpose:** แก้โจทย์ "หาผลรวมกำลังสองของเลขคู่" ด้วย 3 paradigm เพื่อแสดงว่า Rust เป็นภาษา multi-paradigm

```rust
// OOP style: struct + impl
struct NumberList {
    numbers: Vec<i32>,
}

impl NumberList {
    fn new(numbers: Vec<i32>) -> Self {
        NumberList { numbers }
    }

    fn sum_even_squares(&self) -> i32 {
        self.numbers
            .iter()
            .filter(|&&n| n % 2 == 0)
            .map(|&n| n * n)
            .sum()
    }
}

// Imperative style: ตัวแปร mutable + loop
fn imperative(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for &n in numbers {
        if n % 2 == 0 {
            total += n * n;
        }
    }
    total
}

// Functional style: iterator chain ไม่มีตัวแปรที่ถูกแก้ไข
fn functional(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| n * n)
        .sum()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6];

    println!("Imperative: {}", imperative(&data));
    println!("Functional: {}", functional(&data));

    let list = NumberList::new(data);
    println!("OOP-style:  {}", list.sum_even_squares());
}
```

**Expected Output**

```text
Imperative: 56
Functional: 56
OOP-style:  56
```

**Explanation**

- **Imperative:** ประกาศ `let mut total = 0` แล้ววน `for` เพื่อสะสมค่าทีละรอบ เป็นการบอกคอมพิวเตอร์ว่า "ทำอะไรทีละขั้น" โดยมีการเปลี่ยนค่าตัวแปร (mutable state)
- **Functional:** ใช้ iterator chain `filter` (เลือกเลขคู่) → `map` (ยกกำลังสอง) → `sum` (รวมค่า) โดยไม่มีตัวแปรที่ถูกแก้ไขเลย
- **OOP-style:** Rust ไม่มี `class` แต่ใช้ `struct` เก็บข้อมูล และ `impl` กำหนด method (`new`, `sum_even_squares`) เพื่อรวมข้อมูลกับพฤติกรรมไว้ด้วยกัน
- ทั้ง 3 แบบได้ผลลัพธ์เท่ากันคือ 4 + 16 + 36 = 56

---

### Example 2 — Immutable by default และ Ownership

**Purpose:** แสดงจุดเด่นด้านความปลอดภัยของ Rust ได้แก่ ตัวแปรแก้ค่าไม่ได้โดย default และระบบ ownership

```rust
fn main() {
    // ตัวแปรเป็น immutable โดย default
    let x = 5;
    println!("x = {}", x);

    // ต้องประกาศ mut ถึงจะแก้ค่าได้
    let mut y = 5;
    y += 1;
    println!("y = {}", y);

    // ownership: การ "move" ค่า
    let s1 = String::from("Rust");
    let s2 = s1; // s1 ถูก move ไปให้ s2 แล้ว
    println!("s2 = {}", s2);

    // ถ้าต้องการใช้ทั้งสองตัว ต้อง clone
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);
}
```

**Expected Output**

```text
x = 5
y = 6
s2 = Rust
s2 = Rust, s3 = Rust
```

**Explanation**

- `let x = 5;` ตัวแปรเป็น immutable ถ้าพยายามกำหนดค่าซ้ำ compiler จะแจ้ง error (E0384)
- `let mut y = 5;` ใส่ `mut` เพื่อบอกว่าตั้งใจให้แก้ค่าได้
- `let s2 = s1;` เนื่องจาก `String` เก็บข้อมูลบน heap ค่าจึงถูก **move** ไปให้ `s2` และ `s1` ใช้งานต่อไม่ได้ ถ้าเรียกใช้ `s1` อีกจะเกิด error (E0382)
- `s2.clone()` สร้างสำเนาข้อมูลใหม่ ทำให้ใช้ได้ทั้ง `s2` และ `s3`

---

## 7. Common Mistakes

### Mistake 1 — ใส่ semicolon ท้ายค่าที่ต้องการ return

**Problem**

ใน Rust บรรทัดสุดท้ายของฟังก์ชันที่ไม่มี `;` คือ expression ที่เป็นค่า return ถ้าใส่ `;` จะกลายเป็น statement และฟังก์ชันคืนค่า `()` แทน

**Incorrect Code**

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b;
}

fn main() {
    println!("{}", add(2, 3));
}
```

**Correct Code**

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(2, 3));
}
```

**Why?**

Rust เป็นภาษา expression-oriented โดย block `{ ... }` ให้ค่าเป็น expression สุดท้าย การใส่ `;` ทำให้ค่าถูกทิ้ง compiler จึงฟ้อง mismatched types (E0308) เพราะประกาศว่าจะคืน `i32` แต่ได้ `()`

---

### Mistake 2 — บวกตัวเลขต่างชนิดกันโดยตรง

**Problem**

Rust ไม่แปลงชนิดตัวเลขให้อัตโนมัติ (no implicit conversion)

**Incorrect Code**

```rust
fn main() {
    let a: i32 = 5;
    let b: f64 = 2.5;
    let sum = a + b;
    println!("{}", sum);
}
```

**Correct Code**

```rust
fn main() {
    let a: i32 = 5;
    let b: f64 = 2.5;
    let sum = a as f64 + b;
    println!("{}", sum);
}
```

ผลลัพธ์: `7.5`

**Why?**

Rust เป็น statically typed และ strongly typed ต้องแปลงชนิดด้วย `as` อย่างชัดเจน เพื่อป้องกันข้อผิดพลาดจากการแปลงที่ผู้เขียนไม่ได้ตั้งใจ (error E0277)

---

## 8. Exercises

### Exercise 1 — ผลรวมกำลังสองแบบ Imperative และ Functional

**Problem**

เขียนโปรแกรมหาผลรวมของ 1² + 2² + 3² + 4² + 5² สองแบบ คือ Imperative (ใช้ loop) และ Functional (ใช้ iterator)

**Hint**

แบบ Imperative ใช้ `let mut` กับ `for n in 1..=5` แบบ Functional ใช้ `(1..=5).map(...).sum()`

**Solution**

```rust
fn main() {
    // Imperative
    let mut total = 0;
    for n in 1..=5 {
        total += n * n;
    }

    // Functional
    let total_f: i32 = (1..=5).map(|n| n * n).sum();

    println!("Imperative: {}", total);
    println!("Functional: {}", total_f);
}
```

ผลลัพธ์:

```text
Imperative: 55
Functional: 55
```

**Explanation**

แบบแรกสะสมค่าในตัวแปร mutable ทีละรอบ แบบที่สองสร้างช่วง 1..=5 แล้วแปลงค่าและรวมด้วย iterator โดยไม่มีตัวแปรที่ถูกแก้ไข ผลลัพธ์เท่ากัน

---

### Exercise 2 — struct และ method

**Problem**

สร้าง `struct Rectangle` ที่มี `width` และ `height` พร้อม method `area()` และ `is_square()` แล้วทดสอบกับสี่เหลี่ยม 2 รูป

**Hint**

ใช้ `impl Rectangle { ... }` และ method รับ `&self`

**Solution**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let r = Rectangle::new(4, 5);
    let s = Rectangle::new(3, 3);
    println!("r: area = {}, square? {}", r.area(), r.is_square());
    println!("s: area = {}, square? {}", s.area(), s.is_square());
}
```

ผลลัพธ์:

```text
r: area = 20, square? false
s: area = 9, square? true
```

**Explanation**

`struct` เก็บข้อมูล ส่วน `impl` กำหนดพฤติกรรม `&self` คือการยืมค่ามาอ่านโดยไม่ย้าย ownership ซึ่งเป็นรูปแบบ OOP-style ของ Rust
---

## 9. PPL Perspective

> **ส่วนนี้เป็นหัวใจของรายวิชา Principles of Programming Languages**

วิเคราะห์ Topic นี้ในมุมมองของ Programming Languages

### 9.1 Syntax

Rust ใช้ C-style syntax (มีการใช้วงเล็บปีกกา {} และเครื่องหมายอัฒภาค ;) ทำให้ผู้ที่คุ้นเคยกับ C/C++ หรือ Java สามารถอ่านทำความเข้าใจได้ง่าย แต่จุดเด่นคือ Rust เป็น Expression-oriented language คำสั่งส่วนใหญ่ในภาษา (รวมถึง if, match, หรือ block scope) จะประเมินค่าและส่งคืนค่า (Return value) เสมอ ทำให้สามารถเขียนโค้ดที่กระชับและลดการประกาศตัวแปรชั่วคราวได้

### 9.2 Semantics

Rust มี Semantics หรือพฤติกรรมพื้นฐานที่เน้นความปลอดภัยเป็นหลัก (Safety-first)
- Immutability by default: ตัวแปรทุกตัวใน Rust จะไม่สามารถแก้ไขค่าได้หลังจากการประกาศ เว้นแต่จะระบุ Keyword mut อย่างชัดเจน
- Move Semantics: เมื่อมีการกำหนดค่า (Assign) ตัวแปรที่เป็น Complex type ให้กับตัวแปรใหม่ สิทธิ์การครอบครองข้อมูลนั้นจะถูกย้าย (Move) ไปยังตัวแปรใหม่ทันที ไม่ใช่การคัดลอก (Copy) แบบในภาษาอื่น ซึ่งช่วยป้องกันข้อผิดพลาดทางหน่วยความจำ

### 9.3 Type System

Rust ใช้ระบบชนิดข้อมูลแบบ Statically Typed (ตรวจสอบชนิดข้อมูลตอน Compile) และ Strongly Typed (ไม่มีการแปลงชนิดข้อมูลข้ามประเภทแบบ Implicit ที่ไม่ปลอดภัย) นอกจากนี้ยังมี Type Inference ที่ตัวคอมไพเลอร์สามารถอนุมานชนิดของตัวแปรได้เองจากค่าที่กำหนดให้ โดยที่โปรแกรมเมอร์ไม่ต้องระบุ Type ให้ยุ่งยากในทุกๆ ครั้ง

### 9.4 Memory / Resource Management

นี่คือนวัตกรรมที่สำคัญที่สุดของ Rust ภาษาไม่มี Garbage Collector (GC) แบบ Java หรือ Python และไม่ต้องให้โปรแกรมเมอร์จอง/คืนหน่วยความจำเอง (Manual Memory Management แบบ malloc/free ใน C) แต่ Rust ใช้ระบบ Ownership, Borrowing และ Lifetimes ซึ่งคอมไพเลอร์จะทำการตรวจสอบกฎเหล่านี้ตั้งแต่ตอน Compile (Compile-time) ทำให้ได้ประสิทธิภาพความเร็วเทียบเท่า C/C++ แต่การันตีความปลอดภัย (Memory Safety) และไม่มีปัญหาเรื่อง Memory Leak หรือ Dangling Pointers

### 9.5 Abstraction / Other PPL Concepts

Rust เป็นภาษาแบบ Multi-paradigm ที่ดึงข้อดีของหลายกระบวนทัศน์มารวมกัน:
- Imperative: รองรับการเขียนโปรแกรมแบบลำดับขั้น การเปลี่ยนสถานะตัวแปร (mut) และโครงสร้างควบคุมแบบดั้งเดิม (Loop, if-else)
- Functional Programming (FP): ได้รับอิทธิพลจากภาษา ML และ Haskell รองรับ First-class functions, Closures, Iterators, Pattern Matching (match) และ Algebraic Data Types (ผ่าน enum ที่เก็บค่าได้)
- Object-Oriented Programming (OOP): Rust ไม่มี Class และ Inheritance แบบดั้งเดิม แต่จัดการข้อมูลผ่าน struct และพฤติกรรมผ่าน impl การทำ Polymorphism และ Interface จะใช้ระบบ Traits (คล้าย Interface ใน Java) ซึ่งเป็นการออกแบบที่เน้น  Composition over Inheritance ช่วยลดปัญหาซับซ้อนอย่าง Diamond Problem

### 9.6 Why Rust?

Rust ถูกออกแบบมาเพื่อแก้ปัญหา "ความเร็ว vs ความปลอดภัย" (Performance vs Safety) ที่ภาษาในอดีตต้องเลือกอย่างใดอย่างหนึ่ง Rust นำเสนอ Memory Safety และ "Fearless Concurrency" (การเขียนโปรแกรมแบบทำงานพร้อมกันโดยไม่เกิด Data Races) โดยไม่ต้องพึ่งพา Garbage Collector ทำให้ Rust เหมาะอย่างยิ่งสำหรับการเขียนระบบ (Systems Programming), WebAssembly, และโปรแกรมที่ต้องการประสิทธิภาพขั้นสูงสุด

---

## 10. Rust vs. Other Language

**Comparison Language:** C++

| Aspect | Rust | Other Language |
|---|---|---|
| Syntax | C-style, เน้น Expression-oriented | C-style, เน้น Statement-oriented |
| Semantics / Behavior | ตัวแปรเป็น Immutable โดยค่าเริ่มต้น (ต้องระบุ mut เพื่อให้แก้ค่าได้) | ตัวแปรเป็น Mutable โดยค่าเริ่มต้น (ต้องระบุ const เพื่อไม่ให้แก้ค่าได้) |
| Type System | Static & Strong, มี Type inference และไม่มี Implicit conversion ที่เสี่ยงต่อข้อมูลหาย | Static & Strong, อนุญาตให้ทำ Implicit conversion ได้หลายกรณี (เช่น int เป็น float) |
| Memory Management | จัดการผ่านระบบ Ownership และ Borrow Checker ในตอน Compile-time (ไม่มี GC) | จัดการด้วยตนเอง (new/delete) หรือใช้ Smart Pointers (RAII) เพื่อช่วยจัดการ |
| Safety | การันตี Memory Safety และ Thread Safety ตั้งแต่ตอน Compile จะไม่เกิด Segfaults ถ้าไม่ใช้ unsafe block | ไม่มีระบบป้องกันที่เข้มงวด โปรแกรมเมอร์ต้องรับผิดชอบเรื่อง Memory Leak, Dangling Pointers และ Data Races เอง |

### Rust Example

```rust
trait Drawable {
    fn draw(&self);
}
struct Circle {
    radius: f64,
}
impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius: {}", self.radius);
    }
}

fn main() {
    let shape = Circle { radius: 5.0 };
    shape.draw();
}
```

### `[C++]` Example

```C++
#include <iostream>

class Drawable {
public:
    virtual void draw() const = 0;
};
class Circle : public Drawable {
private:
    double radius;
public:
    Circle(double r) : radius(r) {}
    void draw() const override {
        std::cout << "Drawing a circle with radius: " << radius << std::endl;
    }
};

int main() {
    Circle shape(5.0);
    shape.draw();
    return 0;
}
```

### Analysis

จากมุมมองของ PPL การออกแบบสถาปัตยกรรมโปรแกรมในสองภาษานี้มีความแตกต่างกันอย่างชัดเจน C++ ใช้กระบวนทัศน์ OOP แบบดั้งเดิมที่ผูกพันกับ Inheritance (การสืบทอด) ซึ่งทำให้เกิดลำดับชั้น (Class Hierarchy) ที่ซับซ้อนและอาจนำไปสู่ปัญหาความสัมพันธ์ที่แน่นเกินไป (Tight Coupling)

ในทางตรงกันข้าม Rust ปฏิเสธแนวคิด Inheritance แต่เลือกใช้กระบวนทัศน์ที่เน้น Composition และ Traits (คล้าย Type Classes ในภาษา Haskell) Rust แยกส่วนของ "ข้อมูล" (struct) ออกจาก "พฤติกรรม" (trait) อย่างเด็ดขาด การออกแบบนี้ทำให้โค้ดมีความยืดหยุ่นกว่า ปลอดภัยกว่า และเอื้อต่อการทำ Data-oriented design ซึ่งส่งผลดีต่อประสิทธิภาพของ Cache ในระดับ Hardware นอกจากนี้ แม้ทั้งสองภาษาจะจัดการหน่วยความจำโดยไม่ใช้ GC แต่ระบบ Ownership ของ Rust จะบังคับเช็คความถูกต้องตั้งแต่ก่อนรันโปรแกรม ทำให้ช่วยลดภาระทางความคิด (Cognitive load) ของนักพัฒนาเมื่อเทียบกับ C++ ที่มักเกิดปัญหา Memory Corruption จากความผิดพลาดของคนได้ง่ายกว่า

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

Concept + Short Code Illustration

**Member 2**

Detailed Code + Live Demo

**Member 3**

Githup และ PowerPoint ในส่วนของ Rust vs Other Language + PPL

**Member 4**

Exercies, Common Mistakes, Challenge

> สมาชิกทุกคนต้องสามารถอธิบาย Code ของกลุ่มได้ ไม่ใช่เฉพาะส่วนที่ตนเองเขียน

---

## 12. References

> แนะนำให้มีอย่างน้อย **4 แหล่งอ้างอิง** และควรใช้เอกสารทางการเป็นหลัก

1. https://doc.rust-lang.org/book/ch01-00-getting-started.html
2. https://doc.rust-lang.org/book/ch17-00-oop.html
3. https://doc.rust-lang.org/rust-by-example/trait.html
4. https://isocpp.github.io/CppCoreGuidelines/CppCoreGuidelines

---

## 13. AI Usage Declaration

สามารถใช้ AI เป็นเครื่องมือช่วยเรียนรู้และพัฒนาได้ แต่สมาชิกทุกคนต้องเข้าใจและสามารถอธิบายผลงานของกลุ่มได้

| AI Tool | Purpose | How the Result Was Verified |
|---|---|---|
| Gemini | ใช้เพื่อหาแหล่งข้อมูลและวิเคราะห์แนวคิดเชิง PPL | ตรวจสอบความถูกต้องโดยเทียบเคียงกับ Official Documentation และทดสอบรันโค้ดตัวอย่าง |
| Claude | เขียนโค้ดเปรียบเทียบ Rust/C++ | คอมไพล์และรันโค้ด Rust ด้วย `cargo run` และโค้ด C++ ด้วย `g++ -Wall` แล้วเทียบ output กับที่ระบุในเอกสาร, เปิดอ่านบทที่อ้างใน Rust Book (ch.4, 13, 18) และเอกสาร C++ เพื่อยืนยันข้อความ, ตรวจลิงก์อ้างอิงทุกลิงก์ว่าเปิดได้และตรงกับเนื้อหา |

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

*โครงสร้างเอกสารฉบับเต็ม (Key Concepts, Runnable Code Examples, Common Mistakes, Exercises, PPL Perspective, Rust vs Other Language, References, AI Usage Declaration, GitHub Contribution, Final Checklist) ให้ทำต่อจากจุดนี้ตาม Template หลักของวิชา (`rust_tutorial_template.md`) ที่แนบมากับใบมอบหมายงาน*
