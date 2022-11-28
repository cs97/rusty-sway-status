// Swaywm status bar
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {

  //MHz
  let cpu = return_max_cpu_freq();

  //RAM
  let ram = get_ram_usage();

  //BAT
  let bat = get_bat();
 
  //date
  let date = get_date();

  //vol
  let volume = return_vol();

  //ip
  let ip = get_ip();
 
  //status
  let stat = format!("{} {} {} {} {} {}", cpu, ram, ip, volume, bat, date);
  println!("{}", stat);

}

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

fn get_date() -> String {
  let output = Command::new("date").args(["+%a %F %H:%M"]).output().expect("failed to execute process");
  let date = format!("{:?}", String::from_utf8_lossy(&output.stdout));
  let l = date.len();
  return format!("DATE:[{}]", &date[1..l-3]);
}

fn return_vol() -> String {
  let cmd = "amixer".to_string();
  let output = Command::new(cmd).output();
  let out = match output {
    Ok(out) => String::from_utf8(out.stdout).unwrap(),
    Err(_e) => { return "VOL:[no audio]".to_string(); }
  };

  let vol_vec: Vec<String> = out.split(&[' ', '\n'][..]).map(|s| s.to_string()).collect();
  let vol_left = vol_vec[35].to_string();
  let vol_rigth = vol_vec[43].to_string();
  let vol_status = vol_vec[36].to_string();
  return format!("VOL:{}{}{}", vol_left, vol_rigth, vol_status);
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
    //cur_freq = return_core_freq(n);
    let core = "/sys/devices/system/cpu/cpu".to_string() + &n.to_string() + "/cpufreq/scaling_cur_freq";
    let core_freq = return_string(core).to_string();
    let cur_freq = core_freq.parse::<usize>().unwrap();

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

fn get_ip() -> String {

  fn check_state(s: &str) -> bool {
    let v: Vec<&str> = s.split(' ').collect();
    if v.len() < 9 { return false }
    if v[8] == "UP" { return true } else { return false	}
  }

  let output = Command::new("ip").args(["a"]).output().expect("failed to execute process");
  let out = String::from_utf8_lossy(&output.stdout);
  let ip_a: Vec<&str> = out.split('\n').collect();

  for n in 0..ip_a.len() {
    if check_state(ip_a[n]) {
      let link: Vec<&str> = ip_a[n].split(' ').collect();
      let ip: Vec<&str> = ip_a[n + 2].split(' ').collect();
      return format!("{}[{}]", link[1], ip[5]);
    }
  }
  return format!("lo:{}","[127.0.0.1/8]")
}