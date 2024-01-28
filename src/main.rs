use anyhow::Error;
use inquire::Select;
use magician::{get_issues, get_packs, load_config, Config};
use progress_bar::*;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

fn do_issues(config: Config, agent: Agent, user_agent: &str) -> Result<String, Error> {
    let filename = "issues.html";
    let mut file = File::create(filename)?;

    init_progress_bar(config.nations.len());
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);

    for nation in config.nations {
        if let Ok(issues) = get_issues(&agent, &nation.nation, &nation.password) { 
            if issues.is_some() {
                let issues = issues.unwrap();
                for issue in issues.iter() { 
                    let issue_link = format!(
                        "https://www.nationstates.net/container={}/template-overall=none/page=show_dilemma/dilemma={}/script={}",
                        nation.nation,
                        issue, 
                        user_agent
                    );

                    let issue_link = format!("<div class=issue><a class=\"issue_link\" onauxclick=\"document.getElementsByClassName('issue')[0].remove();\" onclick=\"document.getElementsByClassName('issue')[0].remove();\" target=\"_blank\" href={}>{} ({})</a><br></div>",
                        issue_link,
                        nation.nation,
                        issue
                    );

                    writeln!(file, "{}", issue_link)?;
                }
                let status_msg = format!("fetched {} issues for {}", 
                    issues.len(), 
                    nation.nation
                );

                print_progress_bar_info("Success", &status_msg, Color::Green, Style::Bold);
            // Issues == None
            } else { 
                let status_msg = format!("fetched 0 issues for {}", 
                    nation.nation
                );

                print_progress_bar_info("Success", &status_msg, Color::Green, Style::Bold);
            }
        // Issues failed to fetch
        } else {
            let status_msg = format!("Could not fetch issues for {}",
                nation.nation
            );

            print_progress_bar_info("Failed", &status_msg, Color::Red, Style::Normal);
        }

        inc_progress_bar();
    }

    finalize_progress_bar();

    Ok(filename.to_string())
}

fn do_packs(config: Config, agent: Agent, user_agent: &str) -> Result<String, Error> {
    let filename = "packs.html";
    let mut file = File::create(filename)?;
    init_progress_bar(config.nations.len());
    set_progress_bar_action("Loading", Color::Blue, Style::Bold);
    for nation in config.nations {
        if let Ok(packs) = get_packs(&agent, &nation.nation, &nation.password) {
            if packs > 0 {
                let pack_link = format!(
                    "https://www.nationstates.net/container={}/template-overall=none/page=deck/script={}",
                    nation.nation,
                    user_agent,
                );

                let pack_link = format!("<div class=packs><a class=\"pack_link\" onauxclick=\"document.getElementsByClassName('packs')[0].remove();\" onclick=\"document.getElementsByClassName('packs')[0].remove();\" target=\"_blank\" href={}>{} ({})</a><br></div>",
                    pack_link,
                    nation.nation,
                    packs
                );

                for _ in 0..packs {
                    let _ = writeln!(file, "{}", pack_link);
                }

                let status_msg = format!("Fetched {} packs for {}", 
                    packs,
                    nation.nation
                );

                print_progress_bar_info("Success", &status_msg, Color::Green, Style::Bold);
            } else { 
                let status_msg = format!("Fetched 0 packs for {}", 
                    nation.nation
                );

                print_progress_bar_info("Success", &status_msg, Color::Green, Style::Bold);
            }
        } else { 
            let status_msg = format!("Failed to fetch packs for {}",
                nation.nation
            );

            print_progress_bar_info("Failed", &status_msg, Color::Red, Style::Normal);
        }
        inc_progress_bar();
    }

    finalize_progress_bar();
    Ok(filename.to_string())
}

fn do_logins(config: Config, user_agent: &str) -> Result<String, Error> {
    let filename = "logins.html";
    let containers = "containerize.txt";
    let mut file = File::create(filename)?;
    let mut containers = File::create(containers)?;

    for nation in config.nations {
        let container_rule = format!(
            "@^.*\\.nationstates\\.net/(.*/)?container={}(/.*)?$ , {}",
            nation.nation, nation.nation
        );

        let login_link = format!(
            "https://www.nationstates.net/container={}/template-overall=none/page=login/script={}",
            nation.nation, 
            user_agent,
        );

        let nation_line = format!("<div class=puppet><a class=\"login_link\" onauxclick=\"document.getElementsByClassName('packs')[0].remove();\" onclick=\"document.getElementsByClassName('puppet')[0].remove();\" target=\"_blank\" href={}>{}</a><br></div>",
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
    let url_user_agent = format!("magician_version_{0}__developed_by_Volstrostia__used_by_{1}",
        env!("CARGO_PKG_VERSION"),
        config.main_nation,
    );

    let api_agent: Agent = AgentBuilder::new()
        .user_agent(&user_agent)
        .timeout(Duration::from_secs(15))
        .build();

    let outfile = match mode {
        "Answer Issues" => do_issues(config, api_agent, &url_user_agent),
        "Open Packs" => do_packs(config, api_agent, &url_user_agent),
        "Log In" => do_logins(config, &url_user_agent),
        "Exit" => {
            println!("Goodbye");
            return;
        }
        _ => panic!("Unrecognized option!"),
    };

    match outfile {
        Ok(outfile) => println!("Generated sheet at {outfile}"),
        Err(reason) => panic!("Failed to generate sheet: {reason}"),
    };
}
