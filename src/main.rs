use std::env;
use std::fs::File;
use std::io::{Read};
use std::time::Duration;
use std::{thread};

fn select_serialport() -> Option<String> {
    match serialport::available_ports() {
        Ok(ports) => {
            if ports.len() < 1 {
                return None
            }
            
            for (index, port) in ports.iter().enumerate() {
                println!("{}: {}", index + 1, port.port_name);
            }
            println!("シリアルポートを選択してください(番号を選んてEnterキーを押す)");
        
            let mut word = String::new();
            std::io::stdin().read_line(&mut word).ok();
            match word.trim().to_string().parse::<usize>() {
                Ok(index) => {
                    if index < 1 || index > ports.len() {
                        return None
                    }
                    Some(ports[index - 1].port_name.clone())
                },
                Err(_) => None
            }
        },
        Err(_) => {
            None
        }
    }
}

fn write_data(name: String, data: &Vec<u8>) -> bool {
    match serialport::new(name, 115200)
    .stop_bits(serialport::StopBits::One)
    .data_bits(serialport::DataBits::Eight)
    .parity(serialport::Parity::None)
    .timeout(Duration::from_millis(10000)).open() {
        Ok(mut port) => {
            let start = 2;
            port.write(&[start]).expect("ポートに書き込めません");
            thread::sleep(Duration::from_millis(500));
    
            for &byte in data.iter() {
                port.write(&[byte]).expect("ポートに書き込めません");
                thread::sleep(Duration::from_millis(10));
            }

            thread::sleep(Duration::from_millis(500));
            let end = 3;
            port.write(&[end]).expect("ポートに書き込めません");
        },
        Err(_) => {
            println!("ポートを開けません");
            return false;
        }
    }
    true
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("設定ファイルを指定してください");
        return
    }
    println!("filename: {}", args[1]);
    let ref filepath = args[1];

    match select_serialport() {
        Some(portanme) => {
            println!("serialport: {}", portanme);
        
            match File::open(filepath) {
                Ok(mut file) => {
                    let mut buffer = Vec::new();
                    match file.read_to_end(&mut buffer) {
                        Ok(_) => {
                            println!("{}", String::from_utf8(buffer.clone()).unwrap());
                            if write_data(portanme, &buffer) {
                                println!("設定送信完了しました");

                            } else {
                                println!("設定送信に失敗しました");

                            }
                        },
                        Err(_) => {
                            println!("ファイルが読み込めませんでした");

                        }
                    }
                }
                Err(_) => {
                    println!("ファイルが開けませんでした");

                }
            }
        },
        None => {
            println!("正しいシリアルポートを選択してください");
        }
    }
}

