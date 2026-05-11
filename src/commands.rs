use tokio::process::Command;
use std::env;

// 🔧 Функция для выполнения команд с правильными переменными окружения
pub async fn shell(command: String) -> Result<String, String> {
    // Получаем переменные окружения текущей сессии
    let display = env::var("DISPLAY").unwrap_or_else(|_| ":0".to_string());
    let wayland_display = env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-1".to_string());
    let xdg_runtime = env::var("XDG_RUNTIME_DIR").unwrap_or_else(|_| format!("/run/user/{}", env::var("UID").unwrap_or_else(|_| "1000".to_string())));

    let output = Command::new("sh")
        .arg("-c")
        .arg(&command)
        .env("DISPLAY", &display)
        .env("WAYLAND_DISPLAY", &wayland_display)
        .env("XDG_RUNTIME_DIR", &xdg_runtime)
        .env("DBUS_SESSION_BUS_ADDRESS", format!("unix:path={}/bus", xdg_runtime))
        .output()
        .await
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        Err(if stderr.is_empty() { 
            "Command failed".to_string() 
        } else { 
            stderr 
        })
    }
}

pub async fn backlight_up() -> Result<String, String> {
    shell("brightnessctl set +5%".to_string()).await
}

pub async fn backlight_down() -> Result<String, String> {
    shell("brightnessctl set 5%-".to_string()).await
}

pub async fn change_workspace(workspace_num: String) -> Result<String, String> {
    shell(format!("hyprctl dispatch workspace {}", workspace_num)).await
}

pub async fn player_play_pause() -> Result<String, String> {
    shell("playerctl play-pause".to_string()).await
}

pub async fn player_next() -> Result<String, String> {
    shell("playerctl next".to_string()).await
}

pub async fn player_previous() -> Result<String, String> {
    shell("playerctl previous".to_string()).await
}

pub async fn set_volume(volume: String) -> Result<String, String> {
    shell(format!("amixer set Master {}%", volume)).await
}