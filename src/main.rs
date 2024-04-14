#[derive(Debug,Clone)]
struct O_person{
    s_name: String, 
    n_age: u8
}

// for initializing the struct without named attributes we need to 
impl O_person{
    fn f_o_create(
        s_name: String, 
        n_age: u8
    )
    ->O_person
    {
        return O_person{
            s_name,
            n_age
        }
    }
}
fn f_v(

)-> u8{
    let n1 = 2;
    let n2 = 49;
    // because this is the last line 
    // we can leave out the ';' at the end 
    // and it will be considered as the 
    // return value
    n1+n2
}

    fn f_n_u8_sum_wrap(
        n_u8: u8, 
        n_u8_idx_max: u8, 
        n_i8_summand : i8
    ) -> u8 {
        let n_res = n_u8 as i16 + n_i8_summand as i16;
        if(n_res < 0){
            return n_u8_idx_max-1;
        } 
        return (n_res % n_u8_idx_max as i16).try_into().unwrap()
    }

fn f_read_struct(
    o: &O_person
){
    println!("struct is:{:?}", o);
}
fn f_write_struct(
    o: &mut O_person
){
    o.s_name = "new name".to_string();
    println!("new written struct is:{:?}", o);
}

fn f_multiple_borrow_error(){
    // 

    let mut a_o_person = vec![
        O_person{
            s_name: String::from("test"),
            n_age: 20
        }
    ];


    let mut o_p_mut = &mut a_o_person[0];
    let mut o2 = &mut a_o_person[0];
    f_write_struct(o_p_mut);
    // 74 |     let mut o_p_mut = &mut a_o_person[0];
    // |                            ---------- first mutable borrow occurs here
    // 75 |     let mut o2 = &mut a_o_person[0];
    // |                       ^^^^^^^^^^ second mutable borrow occurs here
    // 76 |     f_write_struct(o_p_mut);
    // |                    ------- first borrow later used here
    
}
fn main() {

    // im-mutable and mutable
    // basically every value is const 
    let n = 2;
    // n = 3 , not working 
    // unless...
    let mut n_mut = 2;
    n_mut = 3; // working because defined as mut


    // construct with named properties/attributes
    let o_person_1 = O_person{
        s_name : String::from("Hans").to_string(),
        n_age : 84 
    };
    println!("person : {:?}", o_person_1);

    // construct without named props /atts

    let o_p2 = O_person::f_o_create(
        String::from("test"),
        100
    );
    println!("person : {:?}", o_p2);

    // a function without a 'return' statement
    println!("return value is {}", f_v());


    // an array 
    let a_o_person = vec![
        o_person_1, 
        o_p2,
        O_person::f_o_create(String::from("jan"), 20),
        O_person::f_o_create(String::from("fridola"), 1),
        O_person::f_o_create(String::from("hans"), 49),
        O_person::f_o_create(String::from("alberta"), 33),
    ];


    // array functions 

    // iterate 

    for o in a_o_person.iter(){
        println!("person is {:?}", o);
    }

    // 'find' a combination of iter
    //  
    // let o_p = a_o_person.find()

    // println!("{:?}", a_o_person)

    // 'wrapping' numbers

    let mut nw = 0;
    let n_idx_max = 4;
    nw = f_n_u8_sum_wrap(nw, n_idx_max,1);
    println!("nw {:?}", nw);
    nw = f_n_u8_sum_wrap(nw, n_idx_max,1);
    println!("nw {:?}", nw);
    nw = f_n_u8_sum_wrap(nw, n_idx_max,1);
    println!("nw {:?}", nw);
    nw = f_n_u8_sum_wrap(nw, n_idx_max,1);
    println!("nw {:?}", nw);
    nw = f_n_u8_sum_wrap(nw, n_idx_max,1);
    println!("nw {:?}", nw);


    nw = f_n_u8_sum_wrap(nw, n_idx_max,-1);
    println!("nw {:?}", nw);
    nw = f_n_u8_sum_wrap(nw, n_idx_max,-1);
    println!("nw {:?}", nw);
    nw = f_n_u8_sum_wrap(nw, n_idx_max,-1);
    println!("nw {:?}", nw);
    nw = f_n_u8_sum_wrap(nw, n_idx_max,-1);
    println!("nw {:?}", nw);
    nw = f_n_u8_sum_wrap(nw, n_idx_max,-1);
    println!("nw {:?}", nw);



    f_multiple_borrow_error();

}
