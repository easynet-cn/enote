//! 屏保服务模块
//!
//! 提供护眼屏保的计时器管理功能，包括：
//! - 空闲倒计时（默认 3600 秒 = 60 分钟）
//! - 屏保持续倒计时（默认 300 秒 = 5 分钟）
//! - 暂停、继续、重置、退出屏保
//! - 通过 Tauri 事件通知前端状态变更
//! - 更新系统托盘 tooltip 显示倒计时

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tauri::{AppHandle, Emitter};
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::i18n::t_simple;

/// 计时器状态
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TimerState {
    /// 空闲倒计时中
    Running,
    /// 已暂停
    Paused,
    /// 屏保激活中
    ScreenSaver,
    /// 已禁用
    Disabled,
}

/// 屏保状态（序列化给前端）
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScreenSaverState {
    /// 当前计时器状态
    pub timer_state: TimerState,
    /// 空闲剩余秒数
    pub idle_remaining: u64,
    /// 空闲超时（秒）
    pub idle_timeout: u64,
    /// 屏保持续时长（秒），0 = 无限
    pub duration: u64,
    /// 屏保剩余秒数
    pub duration_remaining: u64,
}

/// 屏保服务
pub struct ScreenSaverService {
    state: Arc<RwLock<ScreenSaverState>>,
}

impl ScreenSaverService {
    /// 创建新服务实例（默认禁用状态）
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(ScreenSaverState {
                timer_state: TimerState::Disabled,
                idle_remaining: 3600,
                idle_timeout: 3600,
                duration: 300,
                duration_remaining: 0,
            })),
        }
    }

    /// 启动屏保计时器
    pub async fn start(&self, idle_timeout: u64, duration: u64) {
        let mut state = self.state.write().await;
        state.timer_state = TimerState::Running;
        state.idle_timeout = idle_timeout;
        state.idle_remaining = idle_timeout;
        state.duration = duration;
        state.duration_remaining = 0;
        info!(
            "Screen saver started: idle_timeout={}s, duration={}s",
            idle_timeout, duration
        );
    }

    /// 停止屏保计时器（禁用）
    pub async fn stop(&self) {
        let mut state = self.state.write().await;
        state.timer_state = TimerState::Disabled;
        info!("Screen saver stopped");
    }

    /// 暂停空闲倒计时
    pub async fn pause(&self) {
        let mut state = self.state.write().await;
        if state.timer_state == TimerState::Running {
            state.timer_state = TimerState::Paused;
            info!("Screen saver paused");
        }
    }

    /// 继续空闲倒计时
    pub async fn resume(&self) {
        let mut state = self.state.write().await;
        if state.timer_state == TimerState::Paused {
            state.timer_state = TimerState::Running;
            info!("Screen saver resumed");
        }
    }

    /// 重置空闲倒计时
    pub async fn reset(&self) {
        let mut state = self.state.write().await;
        state.timer_state = TimerState::Running;
        state.idle_remaining = state.idle_timeout;
        state.duration_remaining = 0;
        info!("Screen saver reset");
    }

    /// 退出屏保，重新开始空闲倒计时
    pub async fn exit_screen_saver(&self) {
        let mut state = self.state.write().await;
        state.timer_state = TimerState::Running;
        state.idle_remaining = state.idle_timeout;
        state.duration_remaining = 0;
        info!("Screen saver exited, idle countdown restarted");
    }

    /// 更新设置（空闲超时和持续时长，单位：秒）
    pub async fn update_settings(&self, idle_timeout: u64, duration: u64) {
        let mut state = self.state.write().await;
        state.idle_timeout = idle_timeout;
        state.duration = duration;
        // 如果正在运行，重置空闲倒计时
        if state.timer_state == TimerState::Running {
            state.idle_remaining = idle_timeout;
        }
        info!(
            "Screen saver settings updated: idle_timeout={}s, duration={}s",
            idle_timeout, duration
        );
    }

    /// 获取当前状态
    pub async fn get_state(&self) -> ScreenSaverState {
        self.state.read().await.clone()
    }

    /// 启动计时器循环（每秒 tick）
    ///
    /// 在后台 tokio 任务中运行，每秒：
    /// - 递减倒计时
    /// - 到时自动激活/退出屏保
    /// - 发送 ss-tick 事件给前端
    /// - 更新系统托盘 tooltip 和 title
    pub fn start_timer_loop(&self, app_handle: AppHandle) {
        let state = self.state.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            let mut tick: u64 = 0;
            loop {
                interval.tick().await;
                tick += 1;
                let mut s = state.write().await;

                match s.timer_state {
                    TimerState::Running => {
                        if s.idle_remaining > 0 {
                            s.idle_remaining -= 1;
                        }
                        if s.idle_remaining == 0 {
                            // 激活屏保
                            s.timer_state = TimerState::ScreenSaver;
                            s.duration_remaining = s.duration;
                            let _ = app_handle.emit("ss-activate", ());
                            info!("Screen saver activated");
                        }
                    }
                    TimerState::ScreenSaver => {
                        if s.duration > 0 {
                            if s.duration_remaining > 0 {
                                s.duration_remaining -= 1;
                            }
                            if s.duration_remaining == 0 {
                                // 自动退出屏保
                                s.timer_state = TimerState::Running;
                                s.idle_remaining = s.idle_timeout;
                                let _ = app_handle.emit("ss-deactivate", ());
                                info!("Screen saver auto-deactivated");
                            }
                        }
                        // duration == 0 表示无限屏保，不自动退出
                    }
                    TimerState::Paused | TimerState::Disabled => {}
                }

                // 发送 tick 事件给前端
                let _ = app_handle.emit("ss-tick", s.clone());

                // 更新托盘 tooltip 和 title
                update_tray(&app_handle, &s, tick);
            }
        });
    }
}

impl Default for ScreenSaverService {
    fn default() -> Self {
        Self::new()
    }
}

/// 标记托盘 tooltip 是否已首次设置成功（仅日志一次）
static TRAY_TOOLTIP_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// 更新系统托盘 tooltip（鼠标悬停显示）和 title（macOS 菜单栏图标旁显示）
///
/// - tooltip: 所有平台均支持，鼠标悬停时显示完整信息
/// - title: macOS/Linux 在菜单栏图标旁显示文字，Windows 不支持
fn update_tray(app_handle: &AppHandle, state: &ScreenSaverState, tick: u64) {
    let tray_id = tauri::tray::TrayIconId::new("main");
    let Some(tray) = app_handle.tray_by_id(&tray_id) else {
        if tick % 60 == 0 {
            warn!("Tray icon 'main' not found (tick={})", tick);
        }
        return;
    };

    // 根据状态生成 tooltip（完整描述）和 title（简短，用于菜单栏显示）
    let (tooltip, title) = match state.timer_state {
        TimerState::Running => {
            let time_str = format_time(state.idle_remaining);
            (
                format!("ENote - {}", time_str),
                time_str,
            )
        }
        TimerState::Paused => {
            let label = t_simple("tray.ssPaused");
            (format!("ENote - {}", label), label)
        }
        TimerState::ScreenSaver => {
            if state.duration > 0 {
                let time_str = format_time(state.duration_remaining);
                let label = t_simple("tray.ssResting");
                (
                    format!("ENote - {} {}", label, time_str),
                    time_str,
                )
            } else {
                let label = t_simple("tray.ssResting");
                (format!("ENote - {}", label), label)
            }
        }
        TimerState::Disabled => ("ENote".to_string(), String::new()),
    };

    // 设置 tooltip（所有平台）
    match tray.set_tooltip(Some(&tooltip)) {
        Ok(()) => {
            if !TRAY_TOOLTIP_INITIALIZED.swap(true, Ordering::Relaxed) {
                info!("Tray tooltip first set successfully: {}", tooltip);
            }
            // 每 60 秒记录一次，确认 tooltip 正在持续更新
            if tick % 60 == 0 {
                info!("Tray tooltip update confirmed (tick={}): {}", tick, tooltip);
            }
        }
        Err(e) => {
            warn!("Failed to set tray tooltip: {}", e);
        }
    }

    // 设置 title（macOS 菜单栏图标旁显示文字，Windows 不支持）
    let title_result = if title.is_empty() {
        tray.set_title(None::<&str>)
    } else {
        tray.set_title(Some(&title))
    };
    if let Err(e) = title_result {
        // Windows 上 set_title 不支持，仅记录 debug 日志
        tracing::debug!("Failed to set tray title: {}", e);
    }
}

/// 将秒数格式化为时间字符串
///
/// - < 1 小时：`MM:SS`（如 `59:59`）
/// - >= 1 小时：`H:MM:SS`（如 `1:00:00`、`2:30:15`）
fn format_time(total_secs: u64) -> String {
    let hours = total_secs / 3600;
    let mins = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{:02}:{:02}", mins, secs)
    }
}
