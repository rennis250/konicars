extern crate serial;

use std::io;
use std::time::Duration;
use std::str;
use std::{thread, time};
use std::io::prelude::*;
use serial::prelude::*;

#[derive(Default)]
struct ColorData {
    Le: f64,
    Lv: f64,
    X: f64,
    Y: f64,
    Z: f64,
    x: f64,
    y: f64,
    u: f64,
    v: f64,
    T: f64,
    delta_uv: f64,
    lambda_d: f64,
    Pe: f64,
    X10: f64,
    Y10: f64,
    Z10: f64,
    x10: f64,
    y10: f64,
    u10: f64,
    v10: f64,
    T10: f64,
    delta_uv10: f64,
    lambda_d10: f64,
    Pe10: f64,
    spectralData: Vec<f64>,
}

fn init_connection<T: SerialPort>(arg: &str) -> io::Result<()> {
    let mut port = serial::open(arg).unwrap();

    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.set_timeout(Duration::from_secs(1)));

    let mut buf: Vec<u8> = vec![0; 6];

    try!(port.write("RMTS,1\r\n".as_bytes()));
    try!(port.read(&mut buf[..]));

    let resp = err_message(unsafe { str::from_utf8_unchecked(&buf) });
    println!("{:?}", unsafe { str::from_utf8_unchecked(&buf) });
    println!("{:?}", resp);

    let mut buf: Vec<u8> = vec![0; 6];

    try!(port.write("MSWE,0\r\n".as_bytes()));
    try!(port.read(&mut buf[..]));

    let b = buf.clone();
    let resp = err_message(unsafe { str::from_utf8_unchecked(&buf) });
    println!("{:?}", unsafe { str::from_utf8_unchecked(&buf) });
    println!("{:?}", resp);

    Ok(())
}

fn measure(port: &mut SerialPort) -> io::Result<ColorData> {
    let mut buf: Vec<u8> = vec![0; 10];

    try!(port.write("MEAS,1\r\n".as_bytes()));
    try!(port.read(&mut buf[..]));

    let ans = unsafe { str::from_utf8_unchecked(&buf) };
    let resp = err_message(ans.split(',').nth(0).unwrap());
    println!("{:?}", ans);
    println!("{:?}", resp);

    let to = ans.split(',').nth(1).unwrap().parse::<u64>().unwrap();

    thread::sleep(time::Duration::from_secs(to));

    let mut buf: Vec<u8> = vec![0; 6];

    try!(port.read(&mut buf[..]));
    let ans = unsafe { str::from_utf8_unchecked(&buf) };
    println!("{:?}", ans);

    let mut colordata: ColorData = Default::default();
    match resp {
        None => {
            // read spectral data 380...780nm from instrument
            let mut p = 0;
            let mut spectralData: Vec<f64> = vec![0f64; 401];
            for n in 0..4 {
                let mut ne;
                if n != 4 {
                    ne = 1006;
                } else {
                    ne = 1008;
                }
                let mut buf: Vec<u8> = vec![0; ne];
                try!(port.write(format!("MEDR,1,0,{}\r\n", n).as_bytes()));
                try!(port.read(&mut buf[..]));
                let ans = unsafe { str::from_utf8_unchecked(&buf) };
                let resp = err_message(ans.split(',').nth(0).unwrap());
                println!("{:?}", resp);
                println!("{:?}", ans);
                let spectrum = ans.split(',').skip(1).collect::<Vec<&str>>();
                let mut l;
                if n == 4 {
                    l = 101;
                } else {
                    l = 100;
                }
                for m in 0..l {
                    spectralData[p + m] = spectrum[m].parse::<f64>().unwrap();
                }
                p += 100;
            }

            let mut buf: Vec<u8> = vec![0; 195];
            try!(port.write("MEDR,2,0,00\r\n".as_bytes()));
            try!(port.read(&mut buf[..]));
            let ans = unsafe { str::from_utf8_unchecked(&buf) };
            let resp = err_message(ans.split(',').nth(0).unwrap());
            println!("{:?}", resp);
            println!("{:?}", ans);

            let color = ans.split(',').skip(1).collect::<Vec<&str>>();

            colordata = ColorData {
                Le: color[0].parse::<f64>().unwrap(),
                Lv: color[1].parse::<f64>().unwrap(),
                X: color[2].parse::<f64>().unwrap(),
                Y: color[3].parse::<f64>().unwrap(),
                Z: color[4].parse::<f64>().unwrap(),
                x: color[5].parse::<f64>().unwrap(),
                y: color[6].parse::<f64>().unwrap(),
                u: color[7].parse::<f64>().unwrap(),
                v: color[8].parse::<f64>().unwrap(),
                T: color[9].parse::<f64>().unwrap(),
                delta_uv: color[10].parse::<f64>().unwrap(),
                lambda_d: color[11].parse::<f64>().unwrap(),
                Pe: color[12].parse::<f64>().unwrap(),
                X10: color[13].parse::<f64>().unwrap(),
                Y10: color[14].parse::<f64>().unwrap(),
                Z10: color[15].parse::<f64>().unwrap(),
                x10: color[16].parse::<f64>().unwrap(),
                y10: color[17].parse::<f64>().unwrap(),
                u10: color[18].parse::<f64>().unwrap(),
                v10: color[19].parse::<f64>().unwrap(),
                T10: color[20].parse::<f64>().unwrap(),
                delta_uv10: color[21].parse::<f64>().unwrap(),
                lambda_d10: color[22].parse::<f64>().unwrap(),
                Pe10: color[23].parse::<f64>().unwrap(),
                spectralData: spectralData,
            };
        }
        Some(_) => (),
    }
    Ok(colordata)
}

fn err_message(error_check_code: &str) -> Option<&str> {
    // if "OK00" == error_check_code {
    // return None;
    // }

    if "ER00" == error_check_code {
        return Some("Invalid command string or number of parameters received.");
    }

    if "ER02" == error_check_code {
        return Some("Measurement error.");
    }

    if "ER05" == error_check_code {
        return Some("No user calibration values.");
    }

    if "ER10" == error_check_code {
        return Some("Over measurement range.");
    }

    if "ER17" == error_check_code {
        return Some("Parameter error.");
    }

    if "ER20" == error_check_code {
        return Some("No data.");
    }

    if "ER51" == error_check_code {
        return Some("CCD Peltier abnormality.");
    }

    if "ER52" == error_check_code {
        return Some("Temperatur count abnormality.");
    }

    if "ER71" == error_check_code {
        return Some("Outside synchronization signal range.");
    }

    if "ER81" == error_check_code {
        return Some("Shutter operation abnormality.");
    }

    if "ER82" == error_check_code {
        return Some("Internal ND filter operation malfunction.");
    }

    if "ER83" == error_check_code {
        return Some("Measurement angle abnormality.");
    }

    if "ER99" == error_check_code {
        return Some("Program abnormality.");
    }

    None
}

fn set_nd_filter(port: &mut SerialPort, filter: u8) -> io::Result<()> {
    let mut buf: Vec<u8> = vec![0; 195];
    try!(port.write(format!("NDFS,{}\r\n", filter).as_bytes()));
    try!(port.read(&mut buf[..]));
    let ans = unsafe { str::from_utf8_unchecked(&buf) };
    let resp = err_message(ans.split(',').nth(0).unwrap());
    println!("{:?}", resp);
    println!("{:?}", ans);
    println!("Filter , {},  has been set.", filter);
    Ok(())
}
