mod utils;

use core::arch::asm;
use std::ops::Add;

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}
fn main() {
    begin_code();
    println!("{}", "----".repeat(20));
    loops();
    println!("{}", "----".repeat(20));
    tvector();
    println!("{}", "----".repeat(20));
    operating_sys();

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    let f1_name = &f1.name;
    let f1_length = &f1.data.len();
    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
    // let array = vec![1, 2, 3, 4];
    let array: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", array);
    for i in array.iter() {
        println!("i:{}", i)
    }
    ifcondition();
    statement();
    t_str();
    t_tup();
    t_struct();
}

fn t_str() {
    let s = String::from("asdfgh");
    let mut s1 = &s[0..3];
    println!("{}", s1);
    s1 = &s[0..s.len()];
    println!("{}", s1);
    println!("{}", s.len());
    t_str_func();
}

fn ifcondition() {
    println!("----------------call ifcondition----------------");
    let o = Some(3);
    let v = match o {
        Some(x) => x,
        _ => 0,
    };
    println!("v:{}", v);

    let o = Some(3);
    // if是表达式，有返回值，默认情况下只会返回`()`
    let p = if let Some(x) = o { x } else { 0 };
    println!("{}", p);
    println!("----------------call ifcondition----------------");
}

fn begin_code() {
    let s = String::from("hello");
    println!("{}", s);
    take_ownership(s);
    // println!("{}", s);
    // 将会报错，因为所有权被转移了
    let s1 = gives_ownership();

    println!("{}", s1);

    let s2 = String::from("hello");
    let (s3, len) = takes_and_gives_back(s2);
    println!("{} {}", s3, len);

    println!("{}", "-----".repeat(5));

    // reference and borrowing
    let x = 5;
    let y = &x;
    let z = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);

    println!("{}", calc_length(&s3));

    let mut s4 = String::from("hello");
    let ps5 = &mut s4;
    println!("*ps5 {}", *ps5);
    println!("ps5 {}", ps5);
    let ps = &mut s4;
    println!("ps {}", ps);

    change_str(&mut s4);
    println!("s4 {}", s4);

    println!("------replacen-------");
    let sn = "hello world, my world";
    let mut ss = sn.to_string();
    println!("{}", sn.replacen("world", "WORLD", 2));
    println!("{}", sn.replacen("world", "WORLD", 1));
    ss.replace_range(0..12, "w");
    println!("replace_range:{:?}", ss);
    println!("{}", "-----".repeat(5));

    // 复合类型

    let s = String::from("hello world");
    let sslice = &s[0..s.len()];
    println!("sslice:{}", sslice);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    println!("{}", "-----".repeat(5));

    let mut name_buf = [0_u8; 12];
    if cfg!(target_arch = "x86") {
        unsafe {
            asm!(
                "push ebx",
                "cpuid",
                "mov [{0}], ebx",
                "mov [{0} + 4], edx",
                "mov [{0} + 8], ecx",
                "pop ebx",
                in(reg) name_buf.as_mut_ptr(),
                inout("eax") 0 => _,
                out("ecx") _,
                out("edx") _,
            );
        }
        let name = core::str::from_utf8(&name_buf).unwrap();
        println!("CPU Manufacturer ID: {}", name);
    }

    if cfg!(target_arch = "x86_64") {
        unsafe {
            asm!(
                "push rbx",
                "cpuid",
                "mov [{0}], ebx",
                "mov [{0} + 4], edx",
                "mov [{0} + 8], ecx",
                "pop rbx",
                in(reg) name_buf.as_mut_ptr(),
                inout("eax") 0 => _,
                out("ecx") _,
                out("edx") _,
            );
        }
        let name = core::str::from_utf8(&name_buf).unwrap();
        println!("CPU Manufacturer ID: {}", name);
    }
    if cfg!(target_arch = "x86_64") {
        let x: i32;
        unsafe {
            asm!("mov {:r}, 5",out(reg) x);
        }
        assert_eq!(x, 5);
    }
    if cfg!(target_arch = "x86") {
        let x: i32;
        unsafe {
            asm!("mov {:e}, 5",out(reg) x);
        }
        assert_eq!(x, 5);
    }

    let mut maintime: u32;
    let mut subtime: u32;
    let mut newsubtime: u32;
    let mut newmaintime: u32;
    // 循环10次
    for _ in 0..10 {
        unsafe {
            asm!(
                "cpuid",
                "rdtsc",
                out("eax") subtime ,
                out("edx") maintime,
            );
            asm!(
                "cpuid",
                "rdtsc",
                out("eax") newsubtime,
                out("edx") newmaintime,
            );
        }
        println!("{} {}", newmaintime - maintime, newsubtime - subtime);
    }
}

fn take_ownership(some_string: String) {
    println!("{}", "-----------------take_ownership---------");
    println!("s2.len(): {}", some_string.len());
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> (String, usize) {
    let length = a_string.len();
    (a_string, length)
}

fn calc_length(s: &String) -> usize {
    println!("calc: {}", s);
    s.len()
}

fn change_str(s: &mut String) {
    s.push_str(", world");
}

fn loops() {
    let mut i = 0;
    loop {
        println!("loop");
        i += 1;
        if i == 10 {
            break;
        }
    }

    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 {
                continue 'outer;
            } // Continues the loop over `x`.
            if y % 2 == 0 {
                continue 'inner;
            } // Continues the loop over `y`.
            println!("x: {}, y: {}", x, y);
        }
    }

    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }
}

fn tvector() {
    let v = vec![1, 2, 3, 4, 5];
    println!("{:?}", v);
    let s = match v.get(10) {
        Some(x) => println!("{}", x),
        None => println!("None"),
    };
    println!("{:?}", s);

    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(31) = age {
        println!("匹配出来的age是");
    }

    println!("在匹配后，age是{:?}", age);
}

fn operating_sys() {
    if cfg!(target_os = "windows") {
        println!("windows");
    } else if cfg!(target_os = "linux") {
        println!("linux");
    } else if cfg!(target_os = "macos") {
        println!("macos");
    } else {
        println!("other");
    }
}

fn statement() {
    println!("--------------call statement------------");
    assert_eq!(t_statement(), ());
    let a = 3;
    let b = 1 + 2;
    assert_eq!(a, b);
    println!("--------------end call statement------------");
}

fn t_statement() {
    let x = 1;
    let _y = x;
}

fn t_str_func() {
    let mut origin_str = String::from("hello world!");
    // pop 改变会改变原来的字符串
    let p1 = origin_str.pop();
    dbg!(p1);
    // let s = dbg!(origin_str);
    let p2 = origin_str.remove(0);
    // dbg!(p2);
    println!("p2:{}", p2);
    println!("origin:{}", origin_str);

    let mut origin_str2 = String::from("test value");
    origin_str2.truncate(3);
    println!("origin str 2:{}", origin_str2);

    let add_res = origin_str + &origin_str2;
    println!("add result:{}", add_res);

    let s1 = String::from("hello ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3:{}", s3);
    // 这里不会报错
    assert_eq!(s3, "hello world!");
    let s4 = s2.add(" add test");
    println!("{}", s4);
    let s5 = format!("{} {}", "hello", "world");
    println!("s5:{}", s5);
}

fn t_tup() {
    let tup = (12, 1.2, -12);
    let tup2 = (12, 1.2, -12);

    println!("tup:{}", tup.0);
    assert_eq!(tup, tup2);

    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
}

fn t_struct() {}
