// use std::fs::File;
// use std::io::prelude::*;
// use rppal::i2c::I2c;

// use std::thread;
// use std::time::Duration;

mod device;
use device::{DeviceBuffer, KeyboardBuf, RelaMouseBuf, AbslMouseBuf, HIDBuffer, Device};


fn main() {



    let t = HIDBuffer::Keyboard(KeyboardBuf{
        keys : [4,0,0,0,0,0],
        ..Default::default()
    });
    
    let mut d = Device::new();

    println!("{:?}", t);
    d.send(t);
}







// mod device; // 声明模块

// enum Device {
//     Keyboard,
//     RelaMouse,
//     AbslMouse,
// }

// enum Buffer {
//     KeyboardBuf {
//         modifier: u8, // 修饰键
//         reserved: u8, // 保留字节
//         keys: [u8; 6], // 普通按键按下的键值
//     },
//     RelaMouseBuf {
//         button_status: u8, // 按键状态
//         x_movement: i8,    // X 坐标变化量
//         y_movement: i8,    // Y 坐标变化量
//         v_wheel: i8,       // 垂直滚轮变化量
//         h_wheel: i8,       // 水平滚轮变化量
//     },
//     AbslMouseBuf([i8; 7]), // 7fff 小端
// }


// struct UsbDriver{
//     device : Device,
//     file : File,
// }

// impl UsbDriver {
//     fn path(dev: &Device) -> &'static str{
//         match dev {
//             Device::Keyboard => "/dev/hidg0",
//             Device::RelaMouse => "/dev/hidg1",
//             Device::AbslMouse => "/dev/hidg2",
//         }
//     }

//     fn new(dev:Device) -> Result<UsbDriver, std::io::Error>{
//         let path = UsbDriver::path(&dev);
//         let file = File::create(path)?;
//         Ok(UsbDriver {device: dev, file})
//     }

//     fn send(&mut self, buf: &Buffer) -> Result<(), std::io::Error> {

//         let data_u8:Vec<u8> = match buf {
//             Buffer::AbslBuf(arr) => arr.iter().map(|&x| x as u8).collect(),
//             Buffer::RelaBuf(arr) => arr.iter().map(|&x| x as u8).collect(),
//         };
//         self.file.write_all(&data_u8)?;
//         Ok(())
//     }
// }



// fn main() -> Result<(), std::io::Error> {

//     let mut usb = UsbDriver::new(Device::RelaMouse)?;
//     let mut usb2 = UsbDriver::new(Device::AbslMouse)?;
//     let mut count = 0;
//     loop {
//         if count > 1000000 {
//             break;
//         }

//         println!("r: {}", count);

//         match count%4 {
//             1 => usb2.send(&Buffer::AbslBuf([0,5,5,5,5,0,0]))?,
//             2 => usb.send(&Buffer::RelaBuf([0,0,-120,0,0]))?,
//             3 => usb.send(&Buffer::RelaBuf([0,120,0,0,0]))?,
//             0 => usb.send(&Buffer::RelaBuf([0,0,120,0,0]))?,
//             _ => unreachable!(),
//         }
//         count = count + 1;

//         thread::sleep(Duration::from_millis(30));
//     }

//     Ok(())
// }
