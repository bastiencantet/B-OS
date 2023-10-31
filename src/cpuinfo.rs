use raw_cpuid::CpuId;
use crate::println;

pub fn print_cpu_blok_info() {
    let cpuid = CpuId::new();

    println!("-----------------CPU INFO--------------------");
    println!(
        "Vendor is: {}",
        cpuid
            .get_vendor_info()
            .as_ref()
            .map_or_else(|| "unknown", |vf| vf.as_str(),)
    );

    println!(
        "CPU Model is: {}",
        cpuid
            .get_processor_brand_string()
            .as_ref()
            .map_or_else(|| "n/a", |pbs| pbs.as_str())
    );
    println!("---------------------------------------------")
}