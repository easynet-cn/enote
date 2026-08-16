//! 屏保服务模块
//!
//! 提供护眼屏保的计时器管理功能，包括：
//! - 空闲倒计时（默认 3600 秒 = 60 分钟）
//! - 屏保持续倒计时（默认 300 秒 = 5 分钟）
//! - 暂停、继续、重置、退出屏保
//! - 通过 Tauri 事件通知前端状态变更
//! - 更新系统托盘 tooltip 显示倒计时
//! - 多显示器窗口管理（close 退出 + visible(false) 创建 + 延迟 show 激活）
//!
//! 窗口生命周期：
//! - 退出屏保：close() 确定性销毁 → Destroyed 事件自动清理缓存
//! - 激活屏保：visible(false) 创建 → set_fullscreen(true) → 等加载 → show()
//! - 缓存管理：CachedWindow 跟踪显示器指纹，Destroyed 事件保持缓存同步
//! - double check：get_webview_window() + Tauri webview_windows() 验证窗口实际存在

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use tauri::{
    AppHandle, Emitter, Manager, Monitor, WebviewUrl, WebviewWindowBuilder, WindowEvent,
};
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

/// 缓存的屏保窗口信息
///
/// 跟踪屏保窗口 label，用于退出时 close 和 Destroyed 事件清理。
#[derive(Debug, Clone)]
struct CachedWindow {
    /// 窗口 label（如 "screen-saver-0"）
    label: String,
}

/// 屏保服务
pub struct ScreenSaverService {
    state: Arc<RwLock<ScreenSaverState>>,
    /// 屏保窗口缓存（跟踪显示器指纹，Destroyed 事件自动清理）
    cached_windows: Arc<Mutex<Vec<CachedWindow>>>,
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
            cached_windows: Arc::new(Mutex::new(Vec::new())),
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
    ///
    /// 由前端屏保窗口的退出按钮或 Esc 键调用。
    /// 先 close() 销毁窗口，再 emit 通知前端。
    /// close() 是确定性销毁，不会有黑屏残留。
    pub async fn exit_screen_saver(&self, app_handle: &AppHandle) {
        let mut state = self.state.write().await;
        state.timer_state = TimerState::Running;
        state.idle_remaining = state.idle_timeout;
        state.duration_remaining = 0;
        drop(state); // 释放锁后再操作窗口，避免回调中死锁
        // 先 close 窗口（确定性销毁，无黑屏），再 emit 通知前端
        close_screen_saver_windows(app_handle, &self.cached_windows);
        let _ = app_handle.emit("ss-deactivate", ());
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
        let cached_windows = self.cached_windows.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
            let mut tick: u64 = 0;
            loop {
                interval.tick().await;
                tick += 1;

                // 在锁内更新状态，收集需要在锁外执行的操作
                // 避免 emit/窗口操作触发回调时死锁
                let (tick_state, do_activate, do_deactivate) = {
                    let mut s = state.write().await;
                    let mut activate = false;
                    let mut deactivate = false;

                    match s.timer_state {
                        TimerState::Running => {
                            if s.idle_remaining > 0 {
                                s.idle_remaining -= 1;
                            }
                            if s.idle_remaining == 0 {
                                s.timer_state = TimerState::ScreenSaver;
                                s.duration_remaining = s.duration;
                                activate = true;
                            }
                        }
                        TimerState::ScreenSaver => {
                            if s.duration > 0 {
                                if s.duration_remaining > 0 {
                                    s.duration_remaining -= 1;
                                }
                                if s.duration_remaining == 0 {
                                    s.timer_state = TimerState::Running;
                                    s.idle_remaining = s.idle_timeout;
                                    deactivate = true;
                                }
                            }
                            // duration == 0 表示无限屏保，不自动退出
                        }
                        TimerState::Paused | TimerState::Disabled => {}
                    }

                    (s.clone(), activate, deactivate)
                }; // 写锁在此释放

                // 在锁外执行事件发射和窗口操作（可能触发回调，避免死锁）
                if do_activate {
                    // 先检查显示器是否可用，不可用时不 emit ss-activate
                    // 计时器已进入 ScreenSaver 状态，duration 照常倒数
                    // 显示器休眠/系统锁定时用户无法看到屏保，无需创建窗口
                    let has_monitors = match app_handle.available_monitors() {
                        Ok(m) => {
                            if m.is_empty() {
                                warn!("Screen saver activated but no monitors detected, skipping window creation");
                            }
                            !m.is_empty()
                        }
                        Err(e) => {
                            warn!("Screen saver activated but failed to list monitors: {}, skipping", e);
                            false
                        }
                    };

                    if has_monitors {
                        let _ = app_handle.emit("ss-activate", ());
                        show_screen_saver_window(&app_handle, &cached_windows).await;
                    }
                    info!("Screen saver activated");
                }
                if do_deactivate {
                    // 先 close 窗口，再 emit 通知前端
                    close_screen_saver_windows(&app_handle, &cached_windows);
                    let _ = app_handle.emit("ss-deactivate", ());
                    info!("Screen saver auto-deactivated");
                }

                // 发送 tick 事件给前端
                let _ = app_handle.emit("ss-tick", &tick_state);

                // 更新托盘 tooltip 和 title
                update_tray(&app_handle, &tick_state, tick);
            }
        });
    }

    /// 应用退出时清理所有屏保窗口
    ///
    /// 在 RunEvent::ExitRequested 中调用，此时进程即将终止，
    /// 不存在 displayLink 崩溃风险（进程直接退出）。
    pub fn cleanup_windows(&self, app_handle: &AppHandle) {
        let mut cached = self.cached_windows.lock().unwrap();
        for cw in cached.drain(..) {
            if let Some(window) = app_handle.get_webview_window(&cw.label) {
                let _ = window.close();
                info!("Screen saver window '{}' closed on app exit", cw.label);
            }
        }

        // double check: 使用 Tauri 窗口管理器查找残留的屏保窗口
        let orphaned: Vec<String> = app_handle
            .webview_windows()
            .keys()
            .filter(|label| label.starts_with(SS_WINDOW_PREFIX))
            .cloned()
            .collect();
        for label in orphaned {
            if let Some(window) = app_handle.get_webview_window(&label) {
                let _ = window.close();
                warn!(
                    "Orphaned screen saver window '{}' found and closed during cleanup",
                    label
                );
            }
        }
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

/// 屏保窗口 label 前缀
const SS_WINDOW_PREFIX: &str = "screen-saver-";

/// WebView 加载延迟（毫秒），等 index.html 内联脚本设置深色背景后再 show
const SS_SHOW_DELAY_MS: u64 = 300;

/// 显示屏保窗口（visible(false) 创建 → 等加载 → show()）
///
/// 策略：
/// 1. 先用 Tauri 窗口管理器清理残留的屏保窗口（double check）
/// 2. 对每个显示器创建新窗口：visible(false) → set_fullscreen(true) → 延迟 → show()
/// 3. Destroyed 事件会自动清理缓存，此处不需要处理缓存命中
///
/// 之所以每次都新建而非缓存复用：
/// - close() 是确定性销毁，不会黑屏残留
/// - visible(false) 创建 + 延迟 show() 消除 WebView 加载期间的空窗
/// - 避免了 hide()/show() 在 macOS 全屏窗口上的各种不可靠行为
async fn show_screen_saver_window(
    app_handle: &AppHandle,
    cached_windows: &Arc<Mutex<Vec<CachedWindow>>>,
) {
    let monitors = match app_handle.available_monitors() {
        Ok(monitors) if !monitors.is_empty() => monitors,
        Ok(_) => {
            warn!("No monitors detected, cannot create screen saver windows");
            return;
        }
        Err(e) => {
            warn!("Failed to list monitors: {}", e);
            return;
        }
    };

    // double check: 使用 Tauri 窗口管理器查找残留的屏保窗口并清理
    // 正常情况下 close() + Destroyed 事件已清理缓存，此处是兜底
    {
        let orphaned: Vec<String> = app_handle
            .webview_windows()
            .keys()
            .filter(|label| label.starts_with(SS_WINDOW_PREFIX))
            .cloned()
            .collect();
        if !orphaned.is_empty() {
            warn!(
                "Found {} orphaned screen saver window(s) before activation: {:?}",
                orphaned.len(),
                orphaned
            );
            for label in &orphaned {
                if let Some(window) = app_handle.get_webview_window(label) {
                    let _ = window.close();
                    info!("Orphaned screen saver window '{}' closed before activation", label);
                }
            }
            // 清理缓存中对应的条目
            let mut cached = cached_windows.lock().unwrap();
            cached.retain(|cw| !orphaned.contains(&cw.label));
        }
    }

    // 清理缓存中所有条目（close 时 Destroyed 事件应该已清理，此处为兜底）
    {
        let mut cached = cached_windows.lock().unwrap();
        if !cached.is_empty() {
            info!(
                "Clearing {} stale cache entries before activation (expected 0 if Destroyed fired)",
                cached.len()
            );
            cached.clear();
        }
    }

    // 对每个显示器创建新窗口（顺序创建，避免 monitor 引用生命周期问题）
    for (i, monitor) in monitors.iter().enumerate() {
        create_and_cache_window(app_handle, i, monitor, cached_windows).await;
    }
}

/// 创建新的屏保窗口并加入缓存
///
/// 流程：visible(false) 创建 → set_fullscreen(true) → 延迟等 WebView 加载 → show()
/// 延迟期间 index.html 的内联脚本同步设置 #1a1a2e 深色背景，
/// show() 时用户只看到深色背景而非空白/黑色。
async fn create_and_cache_window(
    app_handle: &AppHandle,
    monitor_index: usize,
    monitor: &Monitor,
    cached_windows: &Arc<Mutex<Vec<CachedWindow>>>,
) {
    let label = format!("{}{}", SS_WINDOW_PREFIX, monitor_index);

    // 如果窗口已存在（理论上不应该），先销毁
    if let Some(existing) = app_handle.get_webview_window(&label) {
        let _ = existing.destroy();
        info!(
            "Screen saver window '{}' destroyed before recreation",
            label
        );
    }

    let url = if monitor_index == 0 {
        // 主显示器：完整屏保 UI
        WebviewUrl::App("index.html?mode=screensaver".into())
    } else {
        // 其他显示器：纯色背景（通过 URL 参数区分）
        WebviewUrl::App(
            format!("index.html?mode=screensaver&monitor={}", monitor_index).into(),
        )
    };

    // 物理像素 → 逻辑像素
    let scale = monitor.scale_factor();
    let pos = monitor.position();
    let size = monitor.size();
    let logical_x = pos.x as f64 / scale;
    let logical_y = pos.y as f64 / scale;
    let logical_w = size.width as f64 / scale;
    let logical_h = size.height as f64 / scale;

    match WebviewWindowBuilder::new(app_handle, &label, url)
        .title("")
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .resizable(false)
        .position(logical_x, logical_y)
        .inner_size(logical_w, logical_h)
        .visible(false) // 隐藏创建，等 WebView 加载后再 show
        .build()
    {
        Ok(window) => {
            // 在不可见状态下进入全屏（用户看不到 Space 过渡动画）
            let _ = window.set_fullscreen(true);

            // 监听窗口事件：失焦拉回 + DPI 变化 + 销毁
            let ss_label = label.clone();
            let cw_arc = cached_windows.clone();
            let app_h = app_handle.clone();
            window.on_window_event(move |event: &WindowEvent| {
                match event {
                    WindowEvent::Focused(false) => {
                        // 屏保激活期间，窗口失焦时立即拉回焦点
                        // 阻止用户通过 Cmd+Tab 等方式切换到其他应用
                        // 注意：无法完全拦截 macOS 系统快捷键，但能保证切换后立即拉回
                        if let Some(w) = app_h.get_webview_window(&ss_label) {
                            let _ = w.set_focus();
                            tracing::debug!(
                                "Screen saver window '{}' lost focus, re-focusing",
                                ss_label
                            );
                        }
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        // 显示器配置变化（拔掉/插入显示器、DPI 变化）
                        // 重新查询当前显示器，关闭已移除显示器上的窗口
                        // 新增显示器暂不处理（下次屏保激活时自然创建）
                        info!(
                            "Scale factor changed on '{}', checking monitor topology",
                            ss_label
                        );

                        // 获取当前仍存在的显示器
                        let current_monitors = app_h.available_monitors().unwrap_or_default();
                        let current_count = current_monitors.len();

                        // 检查缓存中的窗口，关闭对应显示器已不存在的
                        let mut cached = cw_arc.lock().unwrap();
                        let before = cached.len();
                        let to_close: Vec<String> = cached
                            .iter()
                            .filter_map(|cw| {
                                // 从 label 提取显示器序号（如 "screen-saver-1" → 1）
                                let idx: usize = match cw
                                    .label
                                    .strip_prefix(SS_WINDOW_PREFIX)
                                    .and_then(|s| s.parse::<usize>().ok())
                                {
                                    Some(i) => i,
                                    None => return None,
                                };
                                if idx < current_count {
                                    None // 显示器仍存在
                                } else {
                                    Some(cw.label.clone()) // 显示器已移除
                                }
                            })
                            .collect();

                        if !to_close.is_empty() {
                            // 在锁外执行 close（避免回调中死锁）
                            let labels_str = to_close.clone();
                            // 先从缓存移除，再 close
                            cached.retain(|cw| !to_close.contains(&cw.label));
                            let after = cached.len();
                            drop(cached);

                            for label in &labels_str {
                                if let Some(w) = app_h.get_webview_window(label) {
                                    match w.close() {
                                        Ok(()) => info!(
                                            "Screen saver window '{}' closed (monitor removed, cache {} → {})",
                                            label, before, after
                                        ),
                                        Err(e) => warn!(
                                            "Failed to close screen saver window '{}' on monitor removal: {}",
                                            label, e
                                        ),
                                    }
                                }
                            }
                        } else {
                            info!(
                                "Monitor topology changed on '{}' but no windows need closing (current monitors: {})",
                                ss_label, current_count
                            );
                        }
                    }
                    WindowEvent::Destroyed => {
                        // 窗口被 close() 或系统回收时触发
                        // 主动从缓存中移除，保持缓存与实际状态一致
                        let mut cached = cw_arc.lock().unwrap();
                        let before = cached.len();
                        cached.retain(|c| c.label != ss_label);
                        let after = cached.len();
                        if before != after {
                            info!(
                                "Screen saver window '{}' destroyed, removed from cache ({} → {} entries)",
                                ss_label, before, after
                            );
                        } else {
                            info!(
                                "Screen saver window '{}' destroyed but not in cache (already removed)",
                                ss_label
                            );
                        }
                    }
                    _ => {}
                }
            });

            // 加入缓存
            {
                let mut cached = cached_windows.lock().unwrap();
                cached.push(CachedWindow {
                    label: label.clone(),
                });
            }

            info!(
                "Screen saver window '{}' created (hidden) on monitor {} (logical {:.0}x{:.0} at {:.0},{:.0}, scale={:.1}), waiting {}ms before show",
                label, monitor_index, logical_w, logical_h, logical_x, logical_y, scale, SS_SHOW_DELAY_MS
            );

            // 等待 WebView 加载 index.html（内联脚本同步设置 #1a1a2e 深色背景）
            tokio::time::sleep(tokio::time::Duration::from_millis(SS_SHOW_DELAY_MS)).await;

            // WebView 已加载深色背景，现在显示
            let _ = window.show();
            let _ = window.set_focus();

            info!(
                "Screen saver window '{}' shown after {}ms delay (monitor {})",
                label, SS_SHOW_DELAY_MS, monitor_index
            );
        }
        Err(e) => {
            warn!("Failed to create screen saver window '{}': {}", label, e);
        }
    }
}

/// 关闭所有屏保窗口（退出屏保时调用）
///
/// close() 是确定性销毁：
/// - 窗口立即消失，不可能黑屏残留
/// - Destroyed 事件自动触发，清理缓存
/// - 不涉及 Space 过渡动画
///
/// 退出时不需要延迟：close() 直接销毁窗口，用户看不到任何过渡。
fn close_screen_saver_windows(
    app_handle: &AppHandle,
    cached_windows: &Arc<Mutex<Vec<CachedWindow>>>,
) {
    let labels: Vec<String> = {
        let cached = cached_windows.lock().unwrap();
        cached.iter().map(|cw| cw.label.clone()).collect()
    };

    // 已 close 的窗口集合，用于 orphaned 检查时跳过
    let mut closed_labels: Vec<String> = Vec::new();

    for label in &labels {
        if let Some(window) = app_handle.get_webview_window(label) {
            match window.close() {
                Ok(()) => {
                    info!("Screen saver window '{}' closed", label);
                    closed_labels.push(label.clone());
                }
                Err(e) => warn!("Failed to close screen saver window '{}': {}", label, e),
            }
        } else {
            // double check: 窗口已不存在（可能已被系统回收）
            info!(
                "Screen saver window '{}' not found during close (already gone), cleaning cache",
                label
            );
        }
    }

    // 兜底：使用 Tauri 窗口管理器检查是否有残留的屏保窗口
    // 跳过已 close 的窗口（close() 是异步的，窗口可能仍在 webview_windows() 中）
    let orphaned: Vec<String> = app_handle
        .webview_windows()
        .keys()
        .filter(|label| {
            label.starts_with(SS_WINDOW_PREFIX) && !closed_labels.contains(label)
        })
        .cloned()
        .collect();
    for label in orphaned {
        if let Some(window) = app_handle.get_webview_window(&label) {
            let _ = window.close();
            warn!(
                "Orphaned screen saver window '{}' found and closed during exit",
                label
            );
        }
    }

    // 注意：不在这里手动清空缓存
    // Destroyed 事件会自动清理对应的缓存条目
    // 如果在这里也清空，Destroyed 事件触发时会找不到条目，产生多余日志
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
