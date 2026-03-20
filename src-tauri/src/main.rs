// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(desktop)]
use clap::Parser;

/// ENote - 跨平台桌面笔记应用
#[cfg(desktop)]
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// 指定配置文件路径
    #[arg(short, long)]
    config: Option<String>,
}

fn main() {
    #[cfg(desktop)]
    {
        let args = Args::parse();
        enote_lib::run_with_config(args.config);
    }
    #[cfg(not(desktop))]
    {
        enote_lib::run();
    }
}
