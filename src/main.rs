static CONFIG_FILE: &str = "config.yml";

fn main() -> std::io::Result<()> {
    events_finder::run(CONFIG_FILE)
}

