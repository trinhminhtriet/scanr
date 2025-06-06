use scanner::{host_info, AllPortStatus};
use terminal::{print, spinner::Spinner};

mod parse_arg;
mod port_descriptions;
mod scanner;
mod terminal;

fn exit(code: i32) {
    Spinner::show_cursor();
    std::process::exit(code);
}

fn tokio_signal() {
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        exit(1);
    });
}

#[tokio::main]
async fn main() {
    let cli_args = parse_arg::CliArgs::new();
    terminal::text_color(&cli_args);
    let exit_error = || print::address_error(&cli_args);

    tokio_signal();

    let Ok(host_info) = host_info::HostInfo::try_from(&cli_args.address).await else {
        return exit_error();
    };
    let Some(ip) = host_info.get_ip(&cli_args) else {
        return exit_error();
    };

    print::name_and_target(&cli_args, ip);
    print::extra_ips(&host_info, ip);

    let spinner = Spinner::new();
    spinner.start();
    let now = std::time::Instant::now();
    let scan_output = AllPortStatus::scan_ports(&cli_args, ip).await;
    spinner.stop();
    let done = now.elapsed();

    print::scan_time(&scan_output, done);
    print::result_table(&scan_output);
}
