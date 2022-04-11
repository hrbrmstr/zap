// use serde::Deserialize;
use std::net::UdpSocket;
// use chrono::{ TimeZone, Utc };

// #[derive(Debug, Deserialize)]
// struct Wind {
//     serial_number: String,
//     r#type: String,
//     hub_sn: String,
//     evt: Vec<f64>
// }

fn listen() -> std::io::Result<()> {
     
    let mut buf = [0; 1024];
    let s = UdpSocket::bind("0.0.0.0:50222").expect(r#"{"message":"Could not bind to address/port."}"#);

    loop {
        
        let (n, _) = s.recv_from(&mut buf).expect(r#"{"message":"No broadcasts received."}"#);
        
        let rec: String = String::from_utf8(buf[..n].to_vec()).unwrap();
        
        if rec.contains("evt_strike") {
            println!("{}", rec);
            // let wind: Wind = serde_json::from_str(&rec).expect("JSON was not well-formatted");
            // eprintln!(
            //     "{:?}\n{}\n{}\n{}\n{}\n{}\n{}", 
            //     wind, wind.serial_number, wind.hub_sn, wind.r#type, Utc.timestamp(wind.ob[0] as i64, 0), wind.evt[1], wind.ob[2]
            // )
        }
        
    }

}

fn main() {
    
    ctrlc::set_handler(move || {
        eprintln!("Ctrl-c pressed. Exiting");
        std::process::exit(0x100);
    }).expect("Error setting Ctrl-C handler");
    
    std::process::exit(match listen() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
    
}

// {"serial_number":"ST-00055227","type":"rapid_wind","hub_sn":"HB-00069665","ob":[1649584886,0.00,0]}
// {"serial_number":"ST-00055227","type":"evt_strike","hub_sn":"HB-00069665","evt":[1645548480,63,17825791]}