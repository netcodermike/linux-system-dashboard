use std::{collections::VecDeque,fs,io::{self,Write},thread,time::{Duration,Instant}};
fn cpu(text:&str)->Result<(u64,u64),String>{
 let line=text.lines().next().ok_or("empty /proc/stat")?;
 let v:Vec<u64>=line.split_whitespace().skip(1).map(|s|s.parse::<u64>().map_err(|e|e.to_string())).collect::<Result<_,_>>()?;
 if v.len()<4{return Err("short CPU row".into())}
 // guest and guest_nice are already included in user/nice.
 Ok((v.iter().take(8).sum(),v[3]+v.get(4).unwrap_or(&0)))
}
fn memory(text:&str)->(u64,u64){
 let get=|key:&str|text.lines().find(|s|s.starts_with(key)).and_then(|s|s.split_whitespace().nth(1)).and_then(|s|s.parse::<u64>().ok()).unwrap_or(0);
 let total=get("MemTotal:");(total.saturating_sub(get("MemAvailable:")),total)
}
fn network(text:&str)->(u64,u64){
 text.lines().skip(2).filter_map(|l|l.split_once(':')).filter(|(n,_)|n.trim()!="lo").fold((0,0),|a,(_,row)|{
 let v:Vec<u64>=row.split_whitespace().filter_map(|s|s.parse().ok()).collect();
 (a.0+v.first().unwrap_or(&0),a.1+v.get(8).unwrap_or(&0))})
}
fn disk(text:&str)->(u64,u64){
 text.lines().fold((0,0),|a,l|{let v:Vec<&str>=l.split_whitespace().collect();
 if v.len()<10 || v[2].starts_with("loop") || v[2].starts_with("ram"){return a;}
 // Include whole block devices only; /sys exposes a partition marker.
 if std::path::Path::new(&format!("/sys/class/block/{}/partition",v[2])).exists(){return a;}
 (a.0+v[5].parse::<u64>().unwrap_or(0)*512,a.1+v[9].parse::<u64>().unwrap_or(0)*512)})
}
fn bar(value:f64)->String{let n=(value.clamp(0.,100.)/5.) as usize;format!("{}{}","#".repeat(n),"-".repeat(20-n))}
fn read(path:&str)->String{fs::read_to_string(path).unwrap_or_default()}
fn main()->Result<(),Box<dyn std::error::Error>>{
 if !cfg!(target_os="linux"){return Err("This dashboard requires Linux /proc and /sys".into())}
 let args:Vec<String>=std::env::args().collect();let once=args.iter().any(|s|s=="--once");
 let threshold=args.iter().position(|s|s=="--alert").and_then(|i|args.get(i+1)).and_then(|s|s.parse::<f64>().ok()).unwrap_or(90.);
 let mut prev=cpu(&read("/proc/stat"))?;let mut net=network(&read("/proc/net/dev"));let mut io=disk(&read("/proc/diskstats"));let mut last=Instant::now();let mut history=VecDeque::new();
 loop {thread::sleep(Duration::from_secs(1));let now=Instant::now();let seconds=now.duration_since(last).as_secs_f64();last=now;
 let cur=cpu(&read("/proc/stat"))?;let total=cur.0.saturating_sub(prev.0);let idle=cur.1.saturating_sub(prev.1);prev=cur;
 let usage=if total==0{0.}else{100.*(total.saturating_sub(idle)) as f64/total as f64};
 let (used,mem)=memory(&read("/proc/meminfo"));let mem_pct=if mem==0{0.}else{100.*used as f64/mem as f64};
 let next_net=network(&read("/proc/net/dev"));let next_io=disk(&read("/proc/diskstats"));
 history.push_back((usage/12.5).clamp(0.,7.) as usize);if history.len()>50{history.pop_front();}
 print!("\x1b[2J\x1b[HLINUX SYSTEM DASHBOARD  |  Ctrl+C to quit\n\nCPU [{}] {:5.1}%\nRAM [{}] {:5.1}%  {} / {} MiB\n",bar(usage),usage,bar(mem_pct),mem_pct,used/1024,mem/1024);
 println!("Network RX {:.1} KiB/s  TX {:.1} KiB/s",next_net.0.saturating_sub(net.0) as f64/1024./seconds,next_net.1.saturating_sub(net.1) as f64/1024./seconds);
 println!("Disk read {:.1} KiB/s  write {:.1} KiB/s",next_io.0.saturating_sub(io.0) as f64/1024./seconds,next_io.1.saturating_sub(io.1) as f64/1024./seconds);net=next_net;io=next_io;
 let chars=['▁','▂','▃','▄','▅','▆','▇','█'];println!("CPU history {}",history.iter().map(|n|chars[*n]).collect::<String>());
 if let Ok(zones)=fs::read_dir("/sys/class/thermal"){for z in zones.flatten(){if let Ok(t)=fs::read_to_string(z.path().join("temp")){if let Ok(t)=t.trim().parse::<f64>(){println!("{} {:.1}°C",z.file_name().to_string_lossy(),t/1000.);}}}}
 if usage>=threshold || mem_pct>=threshold{println!("ALERT: CPU or memory crossed {threshold}%");}
 io::stdout().flush()?;if once{break}
 } Ok(())
}
#[cfg(test)] mod tests{use super::*;
 #[test]fn parses_cpu_without_double_counting_guest(){assert_eq!(cpu("cpu 10 2 3 40 5 1 1 1 9 9").unwrap(),(63,45));}
 #[test]fn computes_available_memory(){assert_eq!(memory("MemTotal: 100 kB\nMemAvailable: 30 kB"),(70,100));}
 #[test]fn excludes_loopback(){assert_eq!(network("header\nheader\nlo: 99 0 0 0 0 0 0 0 99\neth0: 50 0 0 0 0 0 0 0 70"),(50,70));}
}
