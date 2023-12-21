use anyhow::Error;
use serde::Deserialize;
use serde_xml_rs::from_str;
use std::thread::sleep;
use std::time::Duration;
use toml::Table;
use ureq::Agent;

pub struct Nation {
    pub nation: String,
    pub password: String,
}

pub struct Config {
    pub main_nation: String,
    pub nations: Vec<Nation>,
}

fn canonicalize(string: &str) -> String {
    let mut output = String::from(string);
    output.make_ascii_lowercase();
    return str::replace(output.as_str(), " ", "_");
}

pub fn load_config(config_file: &str) -> Result<Config, Error> {
    let config_contents = std::fs::read_to_string(config_file)?;
    let raw_config: Table = toml::from_str(&config_contents)?;
    let mut nations = Vec::new();

    let main_nation = raw_config
        .get("config")
        .expect("Did not find [config] block in config.toml!")
        .get("main_nation")
        .expect("Did not find main_nation in config.toml!")
        .as_str()
        .expect("Could not unwrap main_nation as string!")
        .to_string();

    let puppets = raw_config
        .get("puppets")
        .expect("Did not find [puppets] block in config.toml!")
        .as_table()
        .expect("Could not parse puppets as table!");

    for puppet in puppets.keys() {
        let nation = puppet;
        let password = puppets.get(puppet).unwrap().as_str().unwrap();

        let new_nation: Nation = Nation {
            nation: canonicalize(&nation.to_string()),
            password: password.to_string(),
        };

        nations.push(new_nation);
    }

    let config: Config = Config {
        main_nation,
        nations,
    };

    Ok(config)
}

#[derive(Deserialize)]
struct Issue {
    id: u32,
}

#[derive(Deserialize)]
struct Issues {
    #[serde(alias = "ISSUE")]
    issue: Vec<Issue>,
}

#[derive(Deserialize)]
struct NationResponse {
    #[serde(alias = "ISSUES")]
    issues: Issues,
}

#[derive(Deserialize)]
struct Packs {
    #[serde(alias = "PACKS")]
    packs: u32,
}

pub fn get_issues(agent: &Agent, nation: &str, password: &str) -> Result<Option<Vec<u32>>, Error> {
    let mut issue_ids = Vec::new();

    let url = format!(
        "https://www.nationstates.net/cgi-bin/api.cgi?q=issues&nation={}",
        nation
    );

    let response = agent
        .get(&url)
        .set("X-Password", password)
        .call()?
        .into_string()?;

    let response = from_str(&response);

    if response.is_ok() {
        let response: NationResponse = response.unwrap();

        for issue in response.issues.issue {
            issue_ids.push(issue.id);
        }
        // Rate limit
        sleep(Duration::from_millis(750));

        Ok(Some(issue_ids))
    } else {
        sleep(Duration::from_millis(750));
        Ok(None)
    }
}

pub fn get_packs(agent: &Agent, nation: &str, password: &str) -> Result<u32, Error> {
    let url = format!(
        "https://www.nationstates.net/cgi-bin/api.cgi?q=packs&nation={}",
        nation
    );

    let response = agent
        .get(&url)
        .set("X-Password", password)
        .call()?
        .into_string()?;

    let response: Packs = from_str(&response)?;

    // Rate limit
    sleep(Duration::from_millis(750));

    Ok(response.packs)
}
