// use async_std::prelude::*;
use console::Term;
use std::io::Write;

async fn read_ans(term: &mut console::Term, msg: &str) -> std::io::Result<String> {
    term.write(format!("{}: ", msg).as_bytes())?;
    term.read_line()
}

fn menu_banner(term: &Term) {
    term.write_line(
        "                                                                                     ",
    );
    term.write_line(
        "                                dddddddd                                             ",
    );
    term.write_line(
        "                                d::::::d  iiii                                       ",
    );
    term.write_line(
        "                                d::::::d i::::i                                      ",
    );
    term.write_line(
        "                                d::::::d  iiii                                       ",
    );
    term.write_line(
        "                                d:::::d                                              ",
    );
    term.write_line(
        "rrrr   rrrrrrrrr       ddddddddd:::::d iiiiiii     ssssssssss   ppppp   ppppppppp    ",
    );
    term.write_line(
        "::::rrr:::::::::r    dd::::::::::::::d i:::::i   ss::::::::::s  p::::ppp:::::::::p   ",
    );
    term.write_line(
        ":::::::::::::::::r  d::::::::::::::::d  i::::i ss:::::::::::::s p:::::::::::::::::p  ",
    );
    term.write_line(
        "r::::::rrrrr::::::rd:::::::ddddd:::::d  i::::i s::::::ssss:::::spp::::::ppppp::::::p ",
    );
    term.write_line(
        " r:::::r     r:::::rd::::::d    d:::::d  i::::i  s:::::s  ssssss  p:::::p     p:::::p",
    );
    term.write_line(
        " r:::::r     rrrrrrrd:::::d     d:::::d  i::::i    s::::::s       p:::::p     p:::::p",
    );
    term.write_line(
        " r:::::r            d:::::d     d:::::d  i::::i       s::::::s    p:::::p     p:::::p",
    );
    term.write_line(
        " r:::::r            d:::::d     d:::::d  i::::i ssssss   s:::::s  p:::::p    p::::::p",
    );
    term.write_line(
        " r:::::r            d::::::ddddd::::::ddi::::::is:::::ssss::::::s p:::::ppppp:::::::p",
    );
    term.write_line(
        " r:::::r             d:::::::::::::::::di::::::is::::::::::::::s  p::::::::::::::::p ",
    );
    term.write_line(
        " r:::::r              d:::::::::ddd::::di::::::i s:::::::::::ss   p::::::::::::::pp  ",
    );
    term.write_line(
        " rrrrrrr               ddddddddd   dddddiiiiiiii  sssssssssss     p::::::pppppppp    ",
    );
    term.write_line(
        "                                                                  p:::::p            ",
    );
    term.write_line(
        "                                                                  p:::::p            ",
    );
    term.write_line(
        "                                                                 p:::::::p           ",
    );
    term.write_line(
        "                                                                 p:::::::p           ",
    );
    term.write_line(
        "                                                                 p:::::::p           ",
    );
    term.write_line(
        "                                                                 ppppppppp           ",
    );
    term.write_line(
        "                                                                                     ",
    );
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
        // term.write_line("/----------\\")?;
        // term.write_line("|  rdisp   |")?;
        // term.write_line("\\----------/")?;
        menu_banner(&term);
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
