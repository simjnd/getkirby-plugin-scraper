use regex::Regex;
use serde::{Serialize, Deserialize};
use reqwest::blocking::Client;
use std::fs;

#[derive(Serialize, Deserialize)]
struct Plugin {
  name: String,
  author: String,
  desc: String,
  gh_user: String,
  gh_repo: String,
  kirby_user: String,
  kirby_repo: String
}

fn main() {
  let response =
    Client::new()
      .get("https://getkirby.com/plugins/category:all")
      .send()
      .unwrap()
      .text()
      .unwrap();
  let plugins = Regex::new("https://getkirby.com/plugins/(?P<kirby_user>.*?)/(?P<kirby_repo>.*?)\">(.|\n)*?<h4 class=\"h5\">(?P<name>.*?)</h4>(.|\n)*?<span class=\"color-black\">(?P<author>.*?)</span>").unwrap();
  let plugin_details = Regex::new("<figcaption class=\"prose text-xl color-black\">(.|\n)*?<p>(?P<desc>.*)</p>(.|\n)*?https://api.github.com/repos/(?P<gh_user>.*)/(?P<gh_repo>.*)/zipball").unwrap();

  let mut parsed_plugins: Vec<Plugin> = Vec::new();

  for plugin in plugins.captures_iter(&response) {
    let details =
      Client::new()
        .get(format!(
          "https://getkirby.com/plugins/{}/{}",
          &plugin["kirby_user"],
          &plugin["kirby_repo"]))
        .send()
        .unwrap()
        .text()
        .unwrap();
    let details = plugin_details.captures(&details);
    if let Some(details) = details {
      let plug = Plugin {
        name: plugin["name"].to_owned(),
        author: plugin["author"].to_owned(),
        desc: details["desc"].to_owned(),
        gh_user: details["gh_user"].to_owned(),
        gh_repo: details["gh_repo"].to_owned(),
        kirby_user: plugin["kirby_user"].to_owned(),
        kirby_repo: plugin["kirby_repo"].to_owned()
      };
      parsed_plugins.push(plug);
    }
  }

  let mut json = serde_json::to_string(&parsed_plugins).unwrap();
  json = json.replace("&#8209;", "-");
  json = json.replace("&nbsp;", " ");
  json = json.replace("&amp;", "&");
  fs::write("plugins.json", &json).unwrap();
}