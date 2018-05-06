use std;
use std::fmt;
use std::str::FromStr;

pub struct NvidiaSmiMemResult {
    pub gpus: Vec<GpuMemUsage>,
}

impl fmt::Display for NvidiaSmiMemResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for gpu in self.gpus.iter() {
            write!(f, "{}\n", &gpu)?
        }
        Ok(())
    }
}

impl FromStr for NvidiaSmiMemResult {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<&str>>();
        let mut gpus = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("GPU ") {
                gpus.push(lines[i..(i + 10)].join("\n").parse::<GpuMemUsage>()?)
            }
        }
        Ok(Self { gpus })
    }
}

pub struct GpuMemUsage {
    pub fb: MemUsage,
    pub bar1: MemUsage,
}

impl fmt::Display for GpuMemUsage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fb:\t{}\nbar1:\t{}\n", self.fb, self.bar1)
    }
}

impl FromStr for GpuMemUsage {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<&str>>();
        Ok(Self {
            fb: lines[2..5].join("\n").parse::<MemUsage>()?,
            bar1: lines[6..9].join("\n").parse::<MemUsage>()?,
        })
    }
}

pub struct MemUsage {
    pub total: MemSize,
    pub used: MemSize,
    pub free: MemSize,
}

impl fmt::Display for MemUsage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Total: {}, Used: {}, Free: {}",
            self.total, self.used, self.free
        )
    }
}

impl FromStr for MemUsage {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.split("\n").collect::<Vec<&str>>();
        Ok(Self {
            total: lines[0].parse()?,
            used: lines[1].parse()?,
            free: lines[2].parse()?,
        })
    }
}

pub struct MemSize {
    pub size: usize,
}

impl fmt::Display for MemSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5} MiB", self.size)
    }
}

impl FromStr for MemSize {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(":").map(str::trim).collect::<Vec<&str>>();
        let sizes = parts[1].split(" ").collect::<Vec<&str>>();
        Ok(Self {
            size: sizes[0].trim().parse::<usize>()?,
        })
    }
}
