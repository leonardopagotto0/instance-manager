use std::{fs, io};
use uuid::Uuid;

#[derive(Debug)]
struct ServerProperties<'a> {
    motd: &'a str,
    seed: &'a str,
    difficulty: &'a str,
    gamemode: &'a str,
    max_players: &'a str,
}

fn main() {

    
    let path = "../server/server.properties";
    let log_path = "../logs/".to_string();

    let server_config = ServerProperties {
        difficulty: "difficulty=normal",
        gamemode: "gamemode=survival",
        seed: "level-seed=",
        max_players: "max-players=10",
        motd: "motd=hello word",
    };

    
    let text = set_config(path, server_config);
    if let Err(err) = &text {
        write_log(&log_path, err.kind());
    }
    
    let result = fs::write(path, text.unwrap());
    if let Err(err) = result {
        write_log(&log_path, err.kind());
    }
    
}

fn set_config(path: &str, server_config: ServerProperties ) -> Result<String, io::Error> {

    let file_data = fs::read_to_string(path)?;
    let mut lines:Vec<String> = vec![];

    for mut line in file_data.lines() {
        
        let final_line:Vec<&str> = line.split('=').collect();
        
        if final_line[0] == "level-seed" {
            line = server_config.seed;
        }
        else if final_line[0] == "difficulty" {
            line = server_config.difficulty;
        }
        else if final_line[0] == "max-players" {
            line = server_config.max_players;
        }
        else if final_line[0] == "gamemode" {
            line = server_config.gamemode;
        }
        else if final_line[0] == "motd" {
            line = server_config.motd;
        }

        lines.push(line.to_string() + "\r\n");
    }

    let text:String = lines.into_iter().collect();
    Ok(text)

}

fn write_log(path: &str, err_content: io::ErrorKind) {
    let file_name: String = build_id() + ".txt";

    let result = fs::write(path.to_string() + &file_name, err_content.to_string());

    if let Err(err) = result {
        panic!("error ao criar um log de error, response: {:?}", err);
    }
}

fn build_id() -> String {
    let id:Uuid = Uuid::new_v4();
    id.to_string()
}