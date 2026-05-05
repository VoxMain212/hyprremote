use std::process::Command;
use tokio::task;


pub async fn shell(command: String) -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg(command).output().map_err(|e| e.to_string())?;
    task::spawn_blocking(move || {
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&out.stderr).to_string())
    }}).await.unwrap()
} 

pub async fn backlight_up() -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg("brightnessctl set +5%").output().map_err(|e| e.to_string())?;
    task::spawn_blocking(move || {
        if out.status.success() {
            return Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            return Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }).await.unwrap()
}

pub async fn backlight_down() -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg("brightnessctl set 5%-").output().map_err(|e| e.to_string())?;
    task::spawn_blocking(move || {
        if out.status.success() {
            return Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            return Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }).await.unwrap()
}

pub async fn change_workspace(workspace_num: String) -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg(format!("hyprctl dispatch workspace {workspace_num}")).output().map_err(|e| e.to_string())?;
    task::spawn_blocking(move || {
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }).await.unwrap()
}

pub async fn player_play_pause() -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg("playerctl play-pause").output().map_err(|e| e.to_string())?;
    task::spawn_blocking(move || {
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }).await.unwrap()
}

pub async fn player_next() -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg("playerctl next").output().map_err(|e| e.to_string())?;
    task::spawn_blocking(move || {
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }).await.unwrap()
}

pub async fn player_previous() -> Result<String, String> {
    let out = Command::new("sh").arg("-c").arg("playerctl previous").output().map_err(|e| e.to_string())?;
    task::spawn_blocking(move || {
        if out.status.success() {
            Ok(String::from_utf8_lossy(&out.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).to_string())
        }
    }).await.unwrap()
}
