use std::io;


fn main() {
    //b1
    let m1 = [1, 2, 3, 5, 6, 8, 10, 11];
    let m2 = [8,6,10];

    let mut index = 0;
    let mut check = false;
    while index < m1.len() {
        let mut j=0;
        if m1[index]==m2[j]{
            let mut i = index;
            while i < m1.len()&&j<m2.len(){
                if m1[i]==m2[j] {
                    check = true;
                    i+=1;
                    j+=1;
                }else{
                    j=0;
                    check=false;
                    break;
                }
            }
        }
        if check {
            break;
        }
        index += 1;
    }
    if check == true{
        println!("có là mảng con");
    }else {
        println!("không phải là mảng con")
    }

    //b2
    //     let chuoi:&str="This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal";
    //     let mut input =String::new();
        
    //     println!("Nhập giá trị bất kì:");
    //     io::stdin().read_line(&mut input).unwrap();

    //    let mut a1=input.len()-2;
    //    let mut a2=chuoi.len();
    //    let mut dem=0;

    //    let mut chuoi_ver:Vec<String>=Vec::new();
    
    //    for i in chuoi.split(" ") {
    //        chuoi_ver.push(i.to_string());
        
    //    }
    //     input.pop();
    //     input.pop();

    //    for i in  chuoi_ver.iter(){
    //        if i==&input 
    //        {
    //         dem+=1;
    //        }
    //    }
    //        print!("Có {} từ {} trong chuỗi. ",dem,input);



}
