use socketcan::{CANSocket, CANFrame};
use clap::{Arg, App};

const DEFAULT_CAN_IFACE: &str = "can0";

struct RadarConfig {
    radar_id: u32,
    thrs: u8,
    min_azimuth: i8,
    max_azimuth: i8,
    min_elevation: i8,
    max_elevation: i8,
    save_on_radar: u8,
}

fn main(){
    print!("{}[2J", 27 as char);

    let matches = App::new("mmMaveRadar cfg")
        .version("0.1.0")
        .author("Matvei Klimov <klimatt.gu@gmail.com>")
        .about("Configure the mmWaveRadar via CAN-BUS")
        .arg(Arg::new("can_bus_iface")
            .short("i".parse().unwrap())
            .long("canIface")
            .takes_value(true))
        .get_matches();

    let cIface = matches.value_of("can_bus_iface").unwrap_or_else(|| DEFAULT_CAN_IFACE.into());

    use terminal_menu::*;

    //create the menu
    let menu = menu(vec![

        //run the example and try these out
        label("(mmWaveRadar cfg)"),
        scroll("Radar_Id", vec!["1234", "1235", "1236", "1237", "1238", "1239"]),
        numeric("Thrs", 6.0, Some(1.0), Some(0.0), Some(12.0)),
        numeric("Min_Azimuth", -45.0, Some(1.0), Some(-90.0), Some(0.0)),
        numeric("Max_Azimuth", 45.0, Some(1.0), Some(0.0), Some(90.0)),
        numeric("Min_Elevation", -45.0, Some(1.0), Some(-90.0), Some(0.0)),
        numeric("Max_Elevation", 45.0, Some(1.0), Some(0.0), Some(90.0)),
        list("Save_as_default", vec!["No", "Yes"]),
        button("Send!")
    ]);

    //open the menu
    activate(&menu);

    //other work can be done here
    //wait for the menu to exit
    wait_for_exit(&menu);

    let newCfg : RadarConfig = RadarConfig{
        radar_id: selection_value(&menu, "Radar_Id").parse::<u32>().expect("Error Parse"),
        thrs: numeric_value(&menu, "Thrs") as u8,
        min_azimuth: numeric_value(&menu, "Min_Azimuth") as i8,
        max_azimuth: numeric_value(&menu, "Max_Azimuth") as i8,
        min_elevation: numeric_value(&menu, "Min_Elevation") as i8,
        max_elevation: numeric_value(&menu, "Max_Elevation") as i8,
        save_on_radar: if &selection_value(&menu, "Save_as_default") == "No" {0x00} else {0xFF},
    };

    let payload : [u8; 6] = [newCfg.min_azimuth as u8, newCfg.max_azimuth as u8, newCfg.min_elevation as u8, newCfg.max_elevation as u8, newCfg.thrs, newCfg.save_on_radar];

    let cs = CANSocket::open(cIface).unwrap();

    let mut newCfg_frame = CANFrame::new(newCfg.radar_id, &payload, false, false).unwrap();
    newCfg_frame.force_extended();

    println!("Sending new config: {:x?} to radar: {:x} via iface: {2}", newCfg_frame.data(), newCfg_frame.id(), cIface);

    cs.write_frame(&newCfg_frame).unwrap();
    println!("Done!");

}