// Swaywm status bar
use std::fs;
use std::path::Path;
use std::process::Command;
//use chrono::Utc;
use chrono::Local;
use chrono_tz::Europe::Berlin;
use sysctl::Sysctl;

fn main() {
 
  //status
  #[cfg(not(feature = "battery-status"))]
  #[cfg(any(target_os = "linux"))]
  let stat = format!("{} {} {} {} {}", return_max_cpu_freq(), get_ram_usage(), get_ip(), return_vol(), get_date());
  
  #[cfg(feature = "battery-status")]
  #[cfg(any(target_os = "linux"))]
  let stat = format!("{} {} {} {} {} {}", return_max_cpu_freq(), get_ram_usage(), get_ip(), return_vol(), get_bat(), get_date());


  #[cfg(not(feature = "battery-status"))]
  #[cfg(any(target_os = "freebsd"))]
  let stat = format!("{} {}", return_max_cpu_freq(), get_date());
	
  #[cfg(feature = "battery-status")]
  #[cfg(any(target_os = "freebsd"))]
  let stat = format!("{} {} {}", return_max_cpu_freq(), get_bat(), get_date());

	
  println!("{}", stat);

}

#[cfg(any(target_os = "linux"))]
fn get_bat() -> String {
  if Path::new("/sys/class/power_supply/BAT0").is_dir() {
    let bat_cap = return_string("/sys/class/power_supply/BAT0/capacity".to_string());
    let bat_stat = return_string("/sys/class/power_supply/BAT0/status".to_string());
    return format!("BAT:[{}% {}]", bat_cap, bat_stat);
  } else {
    let bat_cap = return_string("/sys/class/power_supply/BAT1/capacity".to_string());
    let bat_stat = return_string("/sys/class/power_supply/BAT1/status".to_string());
    return format!("BAT:[{}% {}]", bat_cap, bat_stat);
  }
}
#[cfg(any(target_os = "freebsd"))]
fn get_bat() -> String {
	let bat = sysctl::Ctl::new("hw.acpi.battery.life").unwrap().value_string().unwrap();
	let ac: &str = &sysctl::Ctl::new("hw.acpi.battery.state").unwrap().value_string().unwrap();
	
	let power = match ac {
		"1" => "Discharging",
		_ => "Charging",
	};
	return format!("BAT:[{}% {}]", bat, power)
}

fn get_date() -> String {
  //let now = Utc::now();
  let now = Local::now();
  return format!("[{}]", now.format("%a %F %H:%M"));
  //return format!("[{}]", utc.with_timezone(&Berlin).format("%a %F %H:%M"));
}

fn return_vol() -> String {
  let output = Command::new("amixer").output();
  let out = match output {
    Ok(out) => String::from_utf8(out.stdout).unwrap(),
    Err(_e) => { return "VOL:[no audio]".to_string(); }
  };

	let vol_vec: Vec<_> = out.split('\n').collect();
	let vol_left: Vec<_> = vol_vec[5].split(' ').collect();
	let vol_rigth: Vec<_> = vol_vec[6].split(' ').collect();

	let mic_mute: Vec<_> = vol_vec[11].split(' ').collect();
	
  return format!("MIC:{} VOL:{}{}{}", mic_mute[6].to_string(), vol_left[6].to_string(), vol_rigth[6].to_string(), vol_rigth[7].to_string());
}

fn get_ram_usage() -> String {
  let ram = return_string("/proc/meminfo".to_string());
  let mem_lines: Vec<&str> = ram.split('\n').collect();

  fn get_value(v: &Vec<&str>, n: usize) -> usize {
    let s: Vec<&str> = v[n].split(' ').collect();
    let v = s[s.len() -2].parse::<usize>().unwrap();
    return v
  }

  let mem_total = get_value(&mem_lines, 0);
  let mem_free = get_value(&mem_lines, 1);
  let mem_buffer = get_value(&mem_lines, 3);
  let mem_cached = get_value(&mem_lines, 4);

  let mem_use = mem_total - (mem_free + mem_buffer + mem_cached);
  //let mem_use = mem_total - mem_free;
  let mem_use_percent = mem_use / (mem_total / 100);
  return format!("RAM:[{}%]", mem_use_percent);
}

#[cfg(any(target_os = "linux"))]
fn return_max_cpu_freq() -> String {
  let mut cores = 0;
  loop {
    let s = format!("/sys/devices/system/cpu/cpu{}", cores + 1);
    if Path::new(&s).is_dir() {
      cores += 1;
      continue;
    } else {
      break;
    };
  };

  let mut core_num = 0;
  let mut max_freq = 0;
  for n in 0..=cores {
    let core = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", &n);
    let cur_freq = return_string(core).to_string().parse::<usize>().unwrap();

    if cur_freq > max_freq {
      max_freq = cur_freq;
      core_num = n;  
    }
  }
  
  let cpu_khz = max_freq.to_string();
  let cpu_mhz = cpu_khz.split_at(cpu_khz.len() - 3).0;
  return format!("CPU{}:[{}MHz]", core_num.to_string(), cpu_mhz);
}

fn return_string(filename: String) -> String {
  let mut s = fs::read_to_string(filename).expect("File not found");
  s.pop();
  return s
}
#[cfg(any(target_os = "freebsd"))]
fn return_max_cpu_freq() -> String {
	let cpu = sysctl::Ctl::new("dev.cpu.0.freq").unwrap().value().unwrap();
	return format!("CPU:[{}MHz]", cpu)
}
/*
fn get_ip() -> String {
  let output = Command::new("ip").args(["a"]).output().expect("failed to execute process");
  let out = String::from_utf8_lossy(&output.stdout);
  let ip_a: Vec<&str> = out.split('\n').collect();

  for n in 0..ip_a.len() {
    if ip_a[n].contains("state UP") {
      let link: Vec<&str> = ip_a[n].split(' ').collect();
      let ip: Vec<&str> = ip_a[n + 2].split(' ').collect();
      return format!("{}[{}]", link[1], ip[5]);
    }
  }
  return format!("lo:{}","[127.0.0.1/8]")
}
*/
fn get_ip() -> String {
	let myip_s = return_string("/proc/net/fib_trie".to_string());
	let myip: Vec<_> = myip_s.split('\n').collect();
    for n in 0..myip.len() {
        if myip[n].contains("link UNICAST") {
			if myip[n+1].contains("|--") {
            	return format!("IP:[{}]", myip[n+1].to_string()[15..].to_string());
			} else if myip[n+2].contains("|--") {
				return format!("IP:[{}]", myip[n+2].to_string()[15..].to_string());
			}
        }
    }
	return format!("lo:{}","[127.0.0.1/8]");
}
