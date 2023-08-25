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
    match serialport::new(name, 0).timeout(Duration::from_millis(100)).open() {
        Ok(mut port) => {
            let start: [u8; 1] = [2];
            let _ = port.write(&start);
            thread::sleep(Duration::from_millis(500));
    
            match port.write(&data) {
                Ok(_) => {
                    thread::sleep(Duration::from_millis(500));
                    let end: [u8; 1] = [3];
                    let _ = port.write(&end);

                },
                Err(_) => {
                    false;
                }
            }
        },
        Err(_) => {
            false;
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
                            if write_data(portanme.clone(), &buffer) {
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

