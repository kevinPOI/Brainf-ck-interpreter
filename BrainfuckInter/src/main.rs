fn main(){
    let v = vec!['>','<','+','-','[',']',',','.'];
    let mut orig_bf_code = String::new();
    orig_bf_code = String::from("-[------->+<]>-.-[->+++++<]>++.+++++++..+++.[--->+<]>----
    -.---[->+++<]>.-[--->+<]>---.+++.------.--------.");
    let mut bf_code:Vec<char> = Vec::new();//vector for isolated code
    let mut bf_vec:Vec<u8> = Vec::new(); //vector for virtual vector used by brain f*ck
    bf_vec.push(0);//make bf_vec[0];
    let mut pointer_index = 0;// set the pointer to [0];
    let mut count = 0;
    let mut loop_index = 0;
    ////////////////initializing done/////////////////////////////
    bf_code = isolater(&mut orig_bf_code);//isolate code
    bf_code.push('e');//terminate program
    println!("debug:isolated code = {:?} vec is{:?}", bf_code, bf_vec);
    
    //main loop starts～
    while bf_code[count] != 'e'{
        
        if (bf_code[count]== '>'){
            pointer_index +=1;
            bf_vec.push(0);
        }else if (bf_code[count]== '<'){
            pointer_index -=1;
            
        }else if((bf_code[count]== '+') & (bf_vec[pointer_index]== 255)){
            bf_vec[pointer_index] = 0;
        }
        else if (bf_code[count]== '+'){
            bf_vec[pointer_index]= element_p(bf_vec[pointer_index]);
        }else if ((bf_code[count] == '-') & (bf_vec[pointer_index]== 0)){
            bf_vec[pointer_index]= 255;
        }else if (bf_code[count]== '-'){
            bf_vec[pointer_index]= element_m(bf_vec[pointer_index]);   
        }else if (bf_code[count]== ','){
            bf_vec[pointer_index] = bf_write(pointer_index);
        }else if (bf_code[count]== '.'){
            print!("{}",bf_vec[pointer_index] as char);
        }else if(bf_code[count]== '['){
            loop_index = count;
        }else if((bf_code[count]== ']') & (bf_vec[pointer_index]!=0)){
            count = loop_index;
            
        }
        
        count += 1;

        
    }
    

}
//————————————————————————————————————————————————————————————————————————————————————————————————
fn isolater(code:&mut String)->Vec<char>{
    let mut v:Vec<char> = Vec::new();
    for i in code.chars() {
        v.push(i);
    }
    v
}
//fn pointer_forward() pointer_index +=1;
//fn pointer_back()pointer_index -=1;
fn element_p(value:u8)->u8{
    let value = value + 1;
    value
}
fn element_m(value:u8)->u8{
    let value = value - 1;
    value
}
//fn print: println(bf_vec[pointer_index])
fn bf_write(index:usize)->u8{
    println!("Enter value for element[{}]",index);
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)
                .expect("Failed to read line");
    let value: u8 = line.trim().parse()
        .expect("Please type a number!");
    value
}



