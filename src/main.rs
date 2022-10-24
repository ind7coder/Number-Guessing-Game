use std::io;
use ansi_term::Colour;
use rand::Rng;
use std::cmp::Ordering;
use std::string::String;

fn main() {

    println!("\x1B[2J\x1B[1;1H");

    println!("{}\n", Colour::Purple.bold().paint("====================================="));
    println!("{}", Colour::Cyan.bold().paint("Selamat Datang Di Game Tebak-Tebakan"));
    println!("{} {}", 
    Colour::Cyan.bold().paint("Ketik: PLAY"),
    Colour::Yellow.paint("\"Untuk Memulai\""));
    println!("\n{}", Colour::Purple.bold().paint("====================================="));

    println!("\n\n{}", Colour::Blue.bold().paint("Comment:"));

    let mut play = String::new();

    io::stdin().read_line(&mut play).expect("Failed read to line");

    if play.to_lowercase().trim() == "play"{
        println!("\x1B[2J\x1B[1;1H");
    loop{
    let acak = rand::thread_rng().gen_range(1..=4);
    let jawaban = rand::thread_rng().gen_range(1..=100);
    let secret_number1 = rand::thread_rng().gen_range(1..=100);
    let secret_number2 = rand::thread_rng().gen_range(1..=100);
    let secret_number3 = rand::thread_rng().gen_range(1..=100);

    println!("{}", Colour::Blue.bold().paint("Pilih Salah Satu!"));
    if acak == 1{
        println!("A. {}", jawaban);
        println!("B. {}", secret_number1);
        println!("C. {}", secret_number2);
        println!("D. {}", secret_number3);
    }else if acak == 2{
        println!("A. {}", secret_number1);
        println!("B. {}", jawaban);
        println!("C. {}", secret_number2);
        println!("D. {}", secret_number3);
    }else if acak == 3{
        println!("A. {}", secret_number1);
        println!("B. {}", secret_number2);
        println!("C. {}", jawaban);
        println!("D. {}", secret_number3);
    }else if acak == 4{
        println!("A. {}", secret_number1);
        println!("B. {}", secret_number2);
        println!("C. {}", secret_number3);
        println!("D. {}", jawaban);
    }

    println!("\n{}", Colour::Cyan.bold().paint("Jawaban:"));

    let mut nilai = String::new().to_lowercase();

    io::stdin().read_line(&mut nilai).expect("Failed to read line!");

    let jawaban1 = if nilai.trim() == "a"{
        jawaban
    }else if nilai.trim() == "b"{
        secret_number1
    }else if nilai.trim() == "c"{
        secret_number2
    }else if nilai.trim() == "d"{
        secret_number3
    }else{
        0
    };

    let jawaban2 = if nilai.trim() == "a"{
        secret_number1
    }else if nilai.trim() == "b"{
        jawaban
    }else if nilai.trim() == "c"{
        secret_number2
    }else if nilai.trim() == "d"{
        secret_number3
    }else{
        0
    };

    let jawaban3 = if nilai.trim() == "a"{
        secret_number1
    }else if nilai.trim() == "b"{
        secret_number2
    }else if nilai.trim() == "c"{
        jawaban
    }else if nilai.trim() == "d"{
        secret_number3
    }else{
        0
    };

    let jawaban4 = if nilai.trim() == "a"{
        secret_number1
    }else if nilai.trim() == "b"{
        secret_number2
    }else if nilai.trim() == "c"{
        secret_number3
    }else if nilai.trim() == "d"{
        jawaban
    }else{
        0
    };

let soal = if acak == 1{
    jawaban1
}else if acak == 2{
    jawaban2
}else if acak == 3{
    jawaban3
}else if acak == 4{
    jawaban4
}else{
    0
};

println!("\x1B[2J\x1B[1;1H");
println!("================");
println!("\nJawaban mu: {}", soal);

match soal.cmp(&jawaban){
    Ordering::Less => println!("{}", Colour::Yellow.paint("Terlalu rendah!")),
    Ordering::Greater => println!("{}", Colour::Cyan.bold().paint("Terlalu tinggi!")),
    Ordering::Equal => {
        println!("{}", Colour::Green.bold().paint("Win"));
        println!("\n================");
        break;
    }
};

println!("\n================");
}
}
    
}
