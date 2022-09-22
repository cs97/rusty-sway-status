// Swaywm status bar
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {

  //MHz
  let cpu = return_max_cpu_freq();

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
    Ok(out) => String::from_utf8(out.stdout).unwrap(),
    Err(_e) => { return "VOL:[no audio]".to_string(); }
  };

  let vol_vec: Vec<String> = out.split(&[' ', '\n'][..]).map(|s| s.to_string()).collect();
  let vol_left = vol_vec[35].to_string();
  let vol_rigth = vol_vec[43].to_string();
  let vol_status = vol_vec[36].to_string();
  return format!("VOL:{}{}{}", vol_left, vol_rigth, vol_status);
}

fn return_max_cpu_freq() -> String {
  let mut core_num = get_amount_of_cores;
  let mut max_freq = 0;
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

fn get_amount_of_cores() -> usize {
  let mut n = 0;
  loop {
    let s = format!("/sys/devices/system/cpu/cpu"{}, n + 1)
    if Path::new(&s).is_dir() {
      n = n + 1;
      continue;
    } else {
      break;
    };
  };
  return n;
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
