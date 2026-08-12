//! 屏保相关 Tauri 命令
//!
//! 提供前端控制屏保计时器的接口：
//! - 启动/停止屏保
//! - 暂停/继续/重置倒计时
//! - 退出屏保
//! - 更新屏保设置

use std::sync::Arc;

use crate::{
    error::AppError,
    service::screen_saver::{ScreenSaverService, TimerState},
};

/// 启动屏保计时器
#[tauri::command]
pub async fn ss_start(
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
    idle_timeout: u64,
    duration: u64,
) -> Result<(), AppError> {
    screen_saver.start(idle_timeout, duration).await;
    Ok(())
}

/// 停止屏保计时器（禁用）
#[tauri::command]
pub async fn ss_stop(
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
) -> Result<(), AppError> {
    screen_saver.stop().await;
    Ok(())
}

/// 暂停空闲倒计时
#[tauri::command]
pub async fn ss_pause(
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
) -> Result<(), AppError> {
    screen_saver.pause().await;
    Ok(())
}

/// 继续空闲倒计时
#[tauri::command]
pub async fn ss_resume(
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
) -> Result<(), AppError> {
    screen_saver.resume().await;
    Ok(())
}

/// 重置空闲倒计时
#[tauri::command]
pub async fn ss_reset(
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
) -> Result<(), AppError> {
    screen_saver.reset().await;
    Ok(())
}

/// 退出屏保，重新开始空闲倒计时
#[tauri::command]
pub async fn ss_exit(
    app_handle: tauri::AppHandle,
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
) -> Result<(), AppError> {
    screen_saver.exit_screen_saver(&app_handle).await;
    Ok(())
}

/// 更新屏保设置（空闲超时和持续时长，单位：秒）
#[tauri::command]
pub async fn ss_update_settings(
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
    idle_timeout: u64,
    duration: u64,
) -> Result<(), AppError> {
    screen_saver.update_settings(idle_timeout, duration).await;
    Ok(())
}

/// 获取屏保当前状态
#[tauri::command]
pub async fn ss_get_state(
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
) -> Result<crate::service::screen_saver::ScreenSaverState, AppError> {
    Ok(screen_saver.get_state().await)
}

/// 暂停或继续（托盘菜单切换用）
#[tauri::command]
pub async fn ss_toggle_pause(
    screen_saver: tauri::State<'_, Arc<ScreenSaverService>>,
) -> Result<bool, AppError> {
    let state = screen_saver.get_state().await;
    match state.timer_state {
        TimerState::Running => {
            screen_saver.pause().await;
            Ok(true) // 返回 true 表示已暂停
        }
        TimerState::Paused => {
            screen_saver.resume().await;
            Ok(false) // 返回 false 表示已继续
        }
        _ => Ok(false),
    }
}
