use anyhow::Error;
use inquire::Select;
use progress_bar::*;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use magician::{Config, get_issues, get_packs, load_config};
use ureq::{Agent, AgentBuilder};

fn do_issues(config: Config, agent: Agent) -> Result<String, Error> { 
    let filename = "issues.html";
    let mut file = File::create(filename)?;

    init_progress_bar(config.nations.len());
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);

    for nation in config.nations { 
        let issues = get_issues(&agent, &nation.nation, &nation.password)?;
        if issues.is_some() { 
            let issues = issues.unwrap();
            let progress_message = format!("Fetched {} issues for {}", 
                issues.len(), 
                &nation.nation
            );

            print_progress_bar_info("Success", &progress_message, Color::Green, Style::Bold);
            for issue in issues { 
                let issue_link = format!(
                    "https://www.nationstates.net/container={}/template-overall=none/page=show_dilemma/dilemma={}",
                    nation.nation,
                    issue
                );

                let issue_link = format!("<div class=issue><a class=\"login_link\" onclick=\"document.getElementsByClassName('issue')[0].remove();\" target=\"_blank\" href={}>{} ({})</a><br></div>",
                    issue_link,
                    nation.nation,
                    issue
                );

                writeln!(file, "{}", issue_link)?;
            }
        }
        inc_progress_bar();
    }

    finalize_progress_bar();

    Ok(filename.to_string())
}

fn do_packs(config: Config, agent: Agent) -> Result<String, Error> { 
    let filename = "packs.html";
    let mut file = File::create(filename)?;
    init_progress_bar(config.nations.len());
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);
    for nation in config.nations { 
        let packs = get_packs(&agent, &nation.nation, &nation.password)?;
        if packs > 0 { 
            let progress_message = format!("Fetched {} issues for {}", 
                packs,
                &nation.nation
            );

            print_progress_bar_info("Success", &progress_message, Color::Green, Style::Bold);

            let pack_link = format!(
                        "https://www.nationstates.net/container={}/template-overall=none/page=deck",
                        nation.nation,
                    );

            let pack_link = format!("<div class=packs><a class=\"login_link\" onclick=\"document.getElementsByClassName('packs')[0].remove();\" target=\"_blank\" href={}>{} ({})</a><br></div>",
                pack_link,
                nation.nation,
                packs
            );

            let _ = writeln!(file, "{}", pack_link);
        }
//        println!("Fetched {0} packs for {1}", packs, nation.nation);
        inc_progress_bar();
    }

    finalize_progress_bar();
    Ok(filename.to_string())
}

fn do_logins(config: Config) -> Result<String, Error> { 
    let filename = "logins.html";
    let containers = "containerize.txt";
    let mut file = File::create(filename)?;
    let mut containers = File::create(containers)?;

    for nation in config.nations { 
        let container_rule = format!(
            "@^.*\\.nationstates\\.net/(.*/)?container={}(/.*)?$ , {}",
            nation.nation, 
            nation.nation
        );

        let login_link = format!(
            "https://www.nationstates.net/container={}/template-overall=none/page=login",
            nation.nation
        );

        let nation_line = format!("<div class=puppet><a class=\"login_link\" onclick=\"document.getElementsByClassName('puppet')[0].remove();\" target=\"_blank\" href={}>{}</a><br></div>",
            login_link,
            nation.nation
        );

        writeln!(file, "{}", nation_line)?;
        writeln!(containers, "{}", container_rule)?;
    }

    println!("Generated containerize rules at containerize.txt");
    Ok(filename.to_string())
}

fn main() {
    let Ok(config) = load_config("config.toml") else { 
        panic!("Could not load configuration file!")
    };

    println!("Welcome, {}", config.main_nation);

    let options: Vec<&str> = vec!["Answer Issues", "Open Packs", "Log In", "Exit"];
    let Ok(mode) = Select::new("Operating mode?", options).prompt() else { 
        panic!("Unrecognized option!");
    };

    let user_agent = format!(
        "Magician/{0} (Developed by nation=Volstrostia; In use by nation={1})", 
        env!("CARGO_PKG_VERSION"),
        config.main_nation
    );

    let api_agent: Agent = AgentBuilder::new()
        .user_agent(&user_agent)
        .timeout(Duration::from_secs(15))
        .build();

    let outfile = match mode { 
        "Answer Issues" => do_issues(config, api_agent), 
        "Open Packs" => do_packs(config, api_agent), 
        "Log In" => do_logins(config),  
        "Exit" => { 
            println!("Goodbye");
            return;
        }
        _ => panic!("Unrecognized option!")
    };

    match outfile { 
        Ok(outfile) => println!("Generated sheet at {outfile}"),
        Err(reason) => panic!("Failed to generate sheet: {reason}")
    };
}
