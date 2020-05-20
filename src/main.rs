// use async_std::prelude::*;
use console::Term;
use std::io::Write;
use std::path::Path;
use serde::{Serialize, Deserialize};
use std::error::Error;
use string_error;

async fn read_ans(term: &mut console::Term, msg: &str) -> std::io::Result<String> {
    term.write(format!("{}: ", msg).as_bytes())?;
    term.read_line()
}

fn menu_banner(term: &Term) -> std::io::Result<()> {
    if let Err(e) = term.write_line(
        "                                                                                     ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                dddddddd                                             ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                d::::::d  iiii                                       ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                d::::::d i::::i                                      ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                d::::::d  iiii                                       ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                d:::::d                                              ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "rrrr   rrrrrrrrr       ddddddddd:::::d iiiiiii     ssssssssss   ppppp   ppppppppp    ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "::::rrr:::::::::r    dd::::::::::::::d i:::::i   ss::::::::::s  p::::ppp:::::::::p   ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        ":::::::::::::::::r  d::::::::::::::::d  i::::i ss:::::::::::::s p:::::::::::::::::p  ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "r::::::rrrrr::::::rd:::::::ddddd:::::d  i::::i s::::::ssss:::::spp::::::ppppp::::::p ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        " r:::::r     r:::::rd::::::d    d:::::d  i::::i  s:::::s  ssssss  p:::::p     p:::::p",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        " r:::::r     rrrrrrrd:::::d     d:::::d  i::::i    s::::::s       p:::::p     p:::::p",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        " r:::::r            d:::::d     d:::::d  i::::i       s::::::s    p:::::p     p:::::p",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        " r:::::r            d:::::d     d:::::d  i::::i ssssss   s:::::s  p:::::p    p::::::p",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        " r:::::r            d::::::ddddd::::::ddi::::::is:::::ssss::::::s p:::::ppppp:::::::p",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        " r:::::r             d:::::::::::::::::di::::::is::::::::::::::s  p::::::::::::::::p ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        " r:::::r              d:::::::::ddd::::di::::::i s:::::::::::ss   p::::::::::::::pp  ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        " rrrrrrr               ddddddddd   dddddiiiiiiii  sssssssssss     p::::::pppppppp    ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                                                  p:::::p            ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                                                  p:::::p            ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                                                 p:::::::p           ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                                                 p:::::::p           ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                                                 p:::::::p           ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                                                 ppppppppp           ",
    ) { return Err(e); }
    if let Err(e) = term.write_line(
        "                                                                                     ",
    ) { return Err(e); }
    Ok(())
}

async fn menu_vnc_displays() -> std::io::Result<()> {
    let mut term = Term::stdout();
    term.write_line("VNC displays")?;
    term.write_line("------------")?;
    term.clear_line()
}

async fn menu_config() -> std::io::Result<()> {
    let mut term = Term::stdout();
    term.write_line("Configuration")?;
    term.write_line("------------")?;
    term.clear_line()
}

async fn menu_main() -> std::io::Result<()> {
    let mut term = Term::stdout();
    let mut redisplay = true;
    let ident = "  ";
    while redisplay {
        if let Err(e) = menu_banner(&term) { return Err(e); }
        term.write_line("")?;
        term.write_line(format!("{}1. VNC displays:", ident).as_str())?;
        term.write_line(format!("{}2. Configuration", ident).as_str())?;
        term.write_line(format!("{}3. Exit", ident).as_str())?;
        let sans = read_ans(&mut term, format!("{}Select", ident).as_str()).await?;
        let ans = sans.as_str();
        if ans == "1" {
            menu_vnc_displays().await?;
        } else if ans == "2" {
            menu_config().await?;
        } else if ans == "3" {
            redisplay = false;
        } else {
            eprintln!("An invalid menu option was selected.")
        }
    }
    term.clear_line()
}

#[async_std::main]
async fn main() {
    let rv = menu_main().await;
    if let Err(e) = rv {
        eprintln!("Error: {}", e);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Configuration {
    pub(crate) version: String,
    pub(crate) vnc_profiles: Vec<VNCProfile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct VncCommand {
    pub(crate) cmd: String,
    pub(crate) args: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct VNCProfile {
    pub(crate) version: String,
    pub(crate) server: String,
    pub(crate) display: String,
}

async fn read_configuration(path: &Path) -> Result<Configuration, Box<Error>> {
    let res_buf = std::fs::read(path);
    if let Err(e) = res_buf {
        return Err(string_error::into_err(e.to_string()));
    }
    let buf = res_buf.unwrap();
    let res_sbuf = String::from_utf8(buf);
    if let Err(e) = res_sbuf {
        return Err(Box::new(e));
    }
    let sbuf = res_sbuf.unwrap();
    let res_json : Result<Configuration, serde_json::Error> = serde_json::from_str(&sbuf);
    if let Err(e) = res_json {
        return Err(Box::new(e));
    }
    return Ok(res_json.unwrap());
}
