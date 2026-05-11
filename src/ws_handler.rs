use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};
use serde_json::{json, Value};
use crate::commands;

pub async fn handle_socket(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connection(socket))
}

async fn handle_connection(mut socket: WebSocket) {
    println!("🔌 Новое подключение");
    
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                println!("📥 Received: {}", text);
                
                let response = if let Ok(value) = serde_json::from_str::<Value>(&text) {
                    let action = value.get("action").and_then(|v| v.as_str()).unwrap_or("").trim();
                    let e_p = json!({});
                    let params = value.get("params").unwrap_or(&e_p);

                    let result = match action {
                        "backlight" => {
                            if let Some(op) = params.get("op").and_then(|v| v.as_str()) {
                                match op {
                                    "up" => commands::backlight_up().await,
                                    "down" => commands::backlight_down().await,
                                    _ => Err("Unknown backlight op".into())
                                }
                            } else { Err("Missing op param".into()) }
                        }
                        "volume" => {
                            if let Some(mode) = params.get("mode").and_then(|v| v.as_str()) {
                                match mode {
                                    "up" => commands::shell("amixer set Master 5%+".to_string()).await,
                                    "down" => commands::shell("amixer set Master 5%-".to_string()).await,
                                    "mute" => commands::shell("amixer set Master toggle".to_string()).await,
                                    s if s.starts_with("set ") => {
                                        let vol = s.replace("set ", "").replace("%", "").trim().to_string();
                                        commands::set_volume(vol).await
                                    }
                                    _ => Err("Unknown volume mode".into())
                                }
                            } else { Err("Missing mode param".into()) }
                        }
                        "media" => {
                            if let Some(cmd) = params.get("cmd").and_then(|v| v.as_str()) {
                                match cmd {
                                    "play-pause" => commands::player_play_pause().await,
                                    "next" => commands::player_next().await,
                                    "previous" => commands::player_previous().await,
                                    _ => Err("Unknown media cmd".into())
                                }
                            } else { Err("Missing cmd param".into()) }
                        }
                        "mouse_move" => {
                            let dx = params.get("dx").and_then(|v| v.as_i64()).unwrap_or(0);
                            let dy = params.get("dy").and_then(|v| v.as_i64()).unwrap_or(0);
                            commands::shell(format!("xdotool mousemove_relative -- {} {}", dx, dy)).await
                        }
                        "mouse_click" => {
                            let btn = params.get("button").and_then(|v| v.as_str()).unwrap_or("left");
                            let btn_num = match btn { "left" => "1", "middle" => "2", "right" => "3", _ => "1" };
                            commands::shell(format!("xdotool click {}", btn_num)).await
                        }
                        "key_combo" => {
                            if let Some(keys) = params.get("keys").and_then(|v| v.as_array()) {
                                let key_strs: Vec<String> = keys.iter().filter_map(|k| k.as_str().map(String::from)).collect();
                                if key_strs.len() == 2 && key_strs[0] == "SUPER" {
                                    commands::change_workspace(key_strs[1].clone()).await
                                } else {
                                    commands::shell(format!("wtype {}", key_strs.join("+"))).await
                                }
                            } else { Err("Missing keys param".into()) }
                        }
                        "exec" => {
                            if let Some(cmd) = params.get("command").and_then(|v| v.as_str()) {
                                commands::shell(cmd.to_string()).await
                            } else { Err("Missing command param".into()) }
                        }
                        _ => Err(format!("Unknown action: '{}'", action))
                    };

                    match result {
                        Ok(output) => json!({"status": "ok", "action": action, "output": output}),
                        Err(err) => json!({"status": "error", "action": action, "message": err})
                    }
                } else {
                    json!({"status": "error", "message": "Invalid JSON format"})
                };

                if socket.send(Message::Text(response.to_string())).await.is_err() {
                    println!("🔌 Клиент отключился");
                    break;
                }
            }
            Message::Close(_) => {
                println!("🔌 Соединение закрыто");
                break;
            }
            _ => {}
        }
    }
}