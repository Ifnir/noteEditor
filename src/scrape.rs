fn _get_ascii_code() {
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?} \r", b);
                } else {
                    println!("{:?} ({}) \r", b, c);
                }

                if b == _to_ctrl_byte('q') {
                    break;
                }
            }
            Err(err) => die(err),
        }
    } 
}

fn _get_binary() {
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:#b} \r", b);
        } else {
            println!("{:#b} ({})\r", b, c);
        } 
        if b == _to_ctrl_byte('q') {
            break;
        }
    } 
}

fn _to_ctrl_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn _die(e: std::io::Error) {
    panic!("{}", e);
}