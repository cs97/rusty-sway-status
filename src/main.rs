// Swaywm status bar
use std::str;
use std::fs;
use std::process::Command;

fn main() {

  let cpu_cores = 7;
  
  //MHz
  let cpu = return_max_cpu_freq(cpu_cores);

  //BAT
  let bat = get_bat();
 
  //date
  let date = get_date();

  //vol
  let volume = return_vol();
 
  //status
  let stat = format!("{} {} {} {}", cpu, volume, bat, date);
  println!("{}", stat);

}

fn get_bat() -> String {
  let bat_cap = return_string("/sys/class/power_supply/BAT0/capacity".to_string());
  let bat_stat = return_string("/sys/class/power_supply/BAT0/status".to_string());
  return format!("BAT:[{}% {}]", bat_cap, bat_stat);
}

fn get_date() -> String {
  let output = Command::new("date").args(["+%a %F %H:%M"]).output().expect("failed to execute process");
  let date = format!("{:?}", String::from_utf8_lossy(&output.stdout));
  return format!("DATE:[{}]", &date[1..20]);
}

fn return_vol() -> String {
  let cmd = "amixer".to_string();
  let output = Command::new(cmd).output();
  let out = match output {
    Ok(out) => out.stdout,
    Err(_e) => { return "VOL:[no audio]".to_string(); }
  };
  let vol_str = str::from_utf8(&out).unwrap();
  if vol_str.len() < 2 {
    return "[failed to execute amixer]".to_string();
  }
  let vol_vec: Vec<String> = vol_str.split(&[' ', '\n'][..]).map(|s| s.to_string()).collect();
  let vol_left = vol_vec[35].to_string();
  let vol_rigth = vol_vec[43].to_string();
  let vol_status = vol_vec[36].to_string();
  let vol = format!("VOL:{}{}{}", vol_left, vol_rigth, vol_status);
  return vol
}

fn return_max_cpu_freq(cores: usize) -> String {
  let mut max_freq = 0;
  let mut core_num = 0;
  let mut cur_freq;

  for n in 0..=cores {
    cur_freq = return_core_freq(n);

    if cur_freq > max_freq {
      max_freq = cur_freq;
      core_num = n;  
    }
  }

  let cpu_khz = max_freq.to_string();
  let cpu_mhz = cpu_khz.split_at(cpu_khz.len() - 3).0;

  return format!("CPU{}:[{}MHz]", core_num.to_string(), cpu_mhz);
}

fn return_core_freq(core: usize) -> usize {
  let core = "/sys/devices/system/cpu/cpu".to_string() + &core.to_string() + "/cpufreq/scaling_cur_freq";
  let core_freq = return_string(core).to_string();
  let u64freq = core_freq.parse::<usize>().unwrap();
  return u64freq
}

fn return_string(filename: String) -> String {
  let mut s = fs::read_to_string(filename).expect("File not found");
  s.pop();
  return s
}
