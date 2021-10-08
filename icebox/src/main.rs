use icebox::*;

fn main() {
    env_logger::init();

    // let mut args = std::env::args();
    // let pid = args.nth(1).expect("missing pid");
    // let pid: i32 = pid.parse().unwrap();
    /*
    let len = args.next().expect("missing len");
    let len = len.parse().unwrap();
    let addr = args.next().expect("missing phys_addr");
    let addr = if addr.starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    let addr = u64::from_str_radix(addr, 16).unwrap();
    */

    //let vm = backend::Kvm::connect(pid, 2 << 30).unwrap();
    let vm = backend::DumbDump::read("linux.dump").unwrap();
    //vm::dump_kvm(&vm).unwrap().write("grub.dump").unwrap();

    //let addr = virtual_to_physical(&vm, GuestVirtAddr(vm.get_regs().rip)).unwrap();
    let _ = dbg!(os::Linux::quick_check(&vm));
    //println!("0x{:x}", addr);

    let mut syms = ibc::SymbolsIndexer::new();
    let kallsyms = std::io::BufReader::new(std::fs::File::open("../kallsyms").unwrap());
    icebox_os_linux::profile::parse_kallsyms(kallsyms, &mut syms).unwrap();

    let mut profile = icebox_os_linux::Profile::new(syms);
    profile.read_object_file("../elf");

    let linux = os::Linux::create(profile);
    linux.read_current_task(&vm).unwrap();
}
