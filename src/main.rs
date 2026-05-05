mod commands;


struct Query {
    id: u64,
    action: String,
    params: String
}

async fn handle_command(query: Query) {
    let action = query.action;
    if action == "brightness_up" {
        let _ = commands::backlight_up().await;
    } else if action == "brightness_down" {
        let _ = commands::backlight_down().await;
    } else if action == "change_workspace" {
        let _ = commands::change_workspace(query.params).await;
    } else if action == "player_next" {
        let _ = commands::player_next().await;
    } else if action == "player_previous" {
        let _ = commands::player_previous().await;
    } else if action == "player_pp" {
        let _ = commands::player_play_pause().await;
    } else {
        println!("Error");
    }
}


#[tokio::main]
async fn main() {
    let res = commands::change_workspace(5).await;
    match res {
        Ok(res) => {
            println!("{res}")
        },
        Err(e) => {
            println!("{e}")
        }
    }
}
