use std::env;
use std::fs::File;
use std::io::{Read};
use std::time::Duration;
use std::{thread};

fn select_serialport() -> Result<String, String> {
    let ports = serialport::available_ports().map_err(|_e| {"ポートが見つかりません"})?;
    if ports.len() < 1 {
        return Err("ポートが見つかりません".to_string());
    }
    for (index, port) in ports.iter().enumerate() {
        println!("{}: {}", index + 1, port.port_name);
    }
    println!("シリアルポートを選択してください(番号を選んてEnterキーを押す)");

    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();

    let index = word.trim().to_string().parse::<usize>().map_err(|_e| {"不正なインデックスです"})?;
    if index < 1 || index > ports.len() {
        return Err("不正なインデックスです".to_string());
    }

    Ok(ports[index - 1].port_name.clone())
}


fn write_data(name: String, data: &Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let mut port = serialport::new(name, 115200)
        .stop_bits(serialport::StopBits::One)
        .data_bits(serialport::DataBits::Eight)
        .parity(serialport::Parity::None)
        .timeout(Duration::from_millis(10000))
        .open().map_err(|e| {format!("ポートを開けません({error})", error = e.to_string())})?;
    
    let start = 2;
    port.write(&[start]).map_err(|e| {format!("ポートに書き込めません({error})", error = e.to_string())})?;
    thread::sleep(Duration::from_millis(500));

    for &byte in data.iter() {
        port.write(&[byte]).map_err(|e| {format!("ポートに書き込めません({error})", error = e.to_string())})?;
        thread::sleep(Duration::from_millis(10));
    }

    thread::sleep(Duration::from_millis(500));
    let end = 3;
    port.write(&[end]).map_err(|e| {format!("ポートに書き込めません({error})", error = e.to_string())})?;

    Ok(())
}

fn process_file_and_serial(filepath: String) -> Result<(), Box<dyn std::error::Error>> {
    let portname = select_serialport().map_err(|e| {format!("Error: 正しいシリアルポートを選択してください({error})", error = e.to_string())})?;
    println!("serialport: {}", portname);

    let mut buffer = Vec::new();
    let mut file = File::open(filepath).map_err(|e| {format!("Error: ファイルが開けませんでした({error})", error = e.to_string())})?;

    file.read_to_end(&mut buffer).map_err(|e| {format!("Error: ファイルが読み込めませんでした({error})", error = e.to_string())})?;
    
    println!("{}", String::from_utf8(buffer.clone()).unwrap());

    write_data(portname, &buffer).map_err(|e| {format!("Error: 設定送信に失敗しました({error})", error = e.to_string())})?;

    println!("設定送信完了しました");

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("設定ファイルを指定してください");
        return
    }
    println!("filename: {}", args[1]);
    let ref filepath = args[1];

    if let Err(e) = process_file_and_serial(filepath.to_string()) {
        println!("{}", e);
    }
}

