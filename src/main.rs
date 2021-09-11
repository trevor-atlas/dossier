use sysinfo::{System, SystemExt, ComponentExt, Component};
use tokio::time::{self, Duration};


#[tokio::main]
async fn main() {
    let mut sys = System::new_all();

    if System::IS_SUPPORTED {
        println!("This OS is supported!");
        let five_seconds = Duration::new(5, 0);
        let mut interval = time::interval(five_seconds);
        loop  {
            interval.tick().await;
            get_sys_info(&mut sys);
        }
    } else {
        println!("This OS isn't supported (yet?).");
    }
}

fn get_sys_info(sys: &mut System) {
    sys.refresh_all();
    println!("############################################");
    println!("");
    // Components temperature: note that this does not work as expected in VMs
    println!("=> components:");
    for component in sys.components() {
        println!("{}: {}°C", component.label(), component.temperature());println!("{:?}", component);
    }

    println!("=> system:");
    // RAM and swap information:
    println!("  total memory: {:.3} GB", sys.total_memory() / 1024 / 1024 );
    println!("  used memory : {:.3} GB", sys.used_memory() / 1024 / 1024 );
    println!("  total swap  : {:.3} GB", sys.total_swap() / 1024 / 1024 );
    println!("  used swap   : {:.3} GB", sys.used_swap() / 1024 / 1024 );

    // Display system information:
    println!("  System name:             {:?}", sys.name().unwrap());
    println!("  System kernel version:   {:?}", sys.kernel_version().unwrap());
    println!("  System OS version:       {:?}", sys.os_version().unwrap());
    println!("  System host name:        {:?}", sys.host_name().unwrap());

    // Number of processors:
    println!("  Number of logical cores: {}", sys.processors().len());
}