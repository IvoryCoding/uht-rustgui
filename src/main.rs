#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]  // Hide terminal window on release

use std::{env, fs::File, string::String};
use eframe::{egui::CentralPanel, epi::App, NativeOptions, run_native, egui::Ui, egui::Vec2, egui};
use eframe::egui::{TopBottomPanel, Label, CtxRef, Layout, Button, Visuals, SidePanel, TextEdit};
use nfd::Response;
use serde::{Deserialize, Serialize};
use docx::{Docx, document::Paragraph};
use chrono::Local;

#[derive(PartialEq)]
enum State {
    MainPage,
    NewReport,
    OpenReport
}

#[derive(PartialEq)]
enum Enum {
    Automatic,
    Manual
}

#[derive(Serialize, Deserialize)]
struct OpenReportInfo {
    report: i32,
    location: String,
    filename: String,
    date: String
}

#[derive(Serialize, Deserialize)]
struct AppConfigs {
    darkmode: bool,
    language: String,
    recent_reports: String,
    auto_save: String,
    currently_open: String
}

struct StateMachine { state: State }

struct StartPage {
    recent_files: Vec<RecentFileCards>,
    darkmode: bool,
    state_machine: StateMachine,
    report_values: NewReportValues,
    open_report: String
}

struct NewReportValues {
    ip_sniffer_bool: bool,
    retrieve_banner_bool: bool,
    vuln_scanner_bool: bool,
    port_sniffer_bool: bool,
    ssh_brute_bool: bool,
    ftp_brute_bool: bool,
    all_bool: bool,
    auto_man_radio: Enum,
    report_name_string: String,
    save_location_string: String
}

struct RecentFileCards {
    report: String,
    location: String,
    filename: String,
    date: String
}

impl AppConfigs {
    /*
        fn for all main settings or variables in json that can be updated

        fn write_updates {
            takes all the variables and writes them to the files
            gets called after each update in each update
        }
    */
    fn new() -> Self {
        AppConfigs {
            darkmode: AppConfigs::parseInformation().darkmode,
            language: AppConfigs::parseInformation().language,
            recent_reports: AppConfigs::parseInformation().recent_reports,
            auto_save: AppConfigs::parseInformation().auto_save,
            currently_open: AppConfigs::parseInformation().currently_open
        }
    }

    // new calls the function to get the information
    // fn for getting the data from the file and returns AppConfigs (there for can be used during update to get what I need)

    fn parseInformation() -> AppConfigs {
        // Here is where it parses the information
        let mut config_path = "";

        if env::consts::OS == "windows" {
            config_path = "F:/ultimate-hacking-tool/src/app-configs.json";
        } else if env::consts::OS == "macos" {
            config_path = "/Volumes/HACKING/ultimate-hacking-tool/src/app-configs.json";
        }

        let config_file = File::open(config_path).unwrap();
        let config_json: serde_json::Value = serde_json::from_reader(config_file).unwrap();
        let darkmode = config_json.get("darkmode").unwrap().as_bool().unwrap();
        let language = config_json.get("language").unwrap().as_str().unwrap();
        let recent_reports = config_json.get("recent-reports").unwrap().to_string();
        let auto_save = config_json.get("auto-save").unwrap().to_string();
        let currently_open = config_json.get("currently-opened").unwrap().to_string();

        return AppConfigs {
            darkmode: darkmode,
            language: language.to_string(),
            recent_reports: recent_reports.to_string(),
            auto_save: auto_save.to_string(),
            currently_open: currently_open.to_string()
        }
    }

    fn update_recent_files(update_for_list: String) {
        println!("[data to add to app-configs.json] \n{}\n", update_for_list);

        let mut configsData = serde_json::json!(&AppConfigs::parseInformation());
        // let mut recentData = &configsData["recent_reports"];
        let mut recentData = serde_json::to_vec(&configsData["recent-reports"]).unwrap();

        let mut data = serde_json::to_string(&recentData);


        for item in recentData {
            println!("[test] \n{}\n", item.to_string());
        }

        // get each list entry
        /*
        for item in recentData.split(",") {
            println!("[item] \n{}\n", item.replace("\\", ""));
        }
        */

        // insert update for the list data
        // increase report number by 1
        // remove if number is equal to 6 ( num == 6)
        // call write function to write the new data
        // print for debugging purposes
        //println!("[testing changes] \n{}\n", recentData);
        println!("[update app-configs.json check - debug]\n{}\n", configsData);
    }

    fn write_out_updates() {
        // Where all updates to AppConfigs will be written to the file and saved
    }
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::MainPage
        }
    }
}

impl NewReportValues {
    fn new() -> Self {
        NewReportValues {
            ip_sniffer_bool: false,
            retrieve_banner_bool: false,
            vuln_scanner_bool: false,
            port_sniffer_bool: false,
            ssh_brute_bool: false,
            ftp_brute_bool: false,
            all_bool: false,
            auto_man_radio: Enum::Automatic,
            report_name_string: String::new(),
            save_location_string: String::new()
        }
    }
}

impl StartPage {
    fn new() -> StartPage {
        let mut config_path = "";

        if env::consts::OS == "windows" {
            config_path = "F:/ultimate-hacking-tool/src/app-configs.json";
        } else if env::consts::OS == "macos" {
            config_path = "/Volumes/HACKING/ultimate-hacking-tool/src/app-configs.json";
        }

        let config_file = File::open(config_path).unwrap();
        let config_json: serde_json::Value = serde_json::from_reader(config_file).unwrap();
        let darkmode = config_json.get("darkmode").unwrap().as_bool().unwrap();
        let recent_reports = config_json.get("recent-reports").unwrap().as_array().unwrap();

        let v_iter = recent_reports.iter().map(|a | RecentFileCards{
            report: format!("{}", a["report"]),
            location: format!("{}", a["location"]),
            filename: format!("{}", a["filename"]),
            date: format!("{}", a["date"])
        });

        StartPage {
            recent_files: Vec::from_iter(v_iter),
            darkmode: darkmode,
            state_machine: StateMachine::new(),
            report_values: NewReportValues::new(),
            open_report: String::new()
        }
    }
}

impl App for StartPage {
    fn update(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        //let mut state_machine = StateMachine::new(); // This has to be moved outside of update

        render_top_panel(ctx, frame, &mut self.darkmode, &mut self.state_machine); // possibly add a back arrow ←

        if self.state_machine.state == State::MainPage {
            render_main_panel(ctx, frame, &mut self.recent_files, &mut self.state_machine, &mut self.open_report);
        } else if self.state_machine.state == State::OpenReport {
            render_report_panel(ctx, frame);
        } else if self.state_machine.state == State::NewReport {
            render_new_report_panel(ctx, frame, &mut self.report_values, &mut self.state_machine);
        }
    }

    fn name(&self) -> &str {
        "Ultimate Hacking Tool"
    }
}

fn render_top_panel(ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>, darkmode: &mut bool, state_machine: &mut StateMachine) {
    TopBottomPanel::top("top_panel").show(ctx, |ui: &mut Ui| {
        ui.add_space(10.);
        egui::menu::bar(ui, |ui: &mut Ui| {
            // Logo
            ui.with_layout(Layout::left_to_right(), |ui: &mut Ui|{
                ui.add(Label::new("Ultimate Hacking Tool").monospace());
            });

            // Controls
            ui.with_layout(Layout::right_to_left(), |ui: &mut Ui|{
                let close_btn = ui.add(Button::new("✖"));
                let new_report_btn = ui.add(Button::new(
                    if state_machine.state != State::MainPage {"<"} else { "+" }));
                let dark_theme_btn = ui.add(Button::new(
                    if *darkmode { "🌙" } else { "🌞" }
                )); // "🌞"

                if close_btn.clicked() {
                    frame.quit();
                }

                if dark_theme_btn.clicked() {
                    *darkmode = !*darkmode;
                }

                if *darkmode {
                    ctx.set_visuals(Visuals::light());
                } else {
                    ctx.set_visuals(Visuals::dark());
                }

                if new_report_btn.clicked() {
                    if state_machine.state != State::MainPage {
                        state_machine.state = State::MainPage
                    } else {
                        state_machine.state = State::NewReport
                    }
                    //Sends data to the state manager to display the new report page
                }
            });
        });
        ui.add_space(10.);
    });
}

fn render_main_panel(ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>, recent_files: &mut Vec<RecentFileCards>, state_machine: &mut StateMachine, open_report: &mut String) {
    // Create a function for this block and check if report is true or false, false means display //
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            // open report label
            ui.vertical_centered(|ui: &mut Ui| {
                ui.add_space(10.);
                let open_report_btn = ui.add(Button::new("open report..."));
                ui.add_space(10.);

                if open_report_btn.clicked() {
                    let result = nfd::open_file_dialog(None, None).unwrap_or_else( |e| {
                        panic!("{}", e);
                    });

                    match result {
                        Response::Okay(file_path) => {
                            state_machine.state = State::OpenReport;

                            // split the file path so its filename and location to use for the report_info object
                            let location = file_path.split("/");
                            let mut date = Local::now();
                            let mut filename = "".to_string();
                            let mut path = "".to_string();

                            for jumps in location {
                                if jumps.contains(".docx") {
                                    filename = jumps.to_string();
                                } else {
                                    path += &*format!("{}/", jumps.to_string());
                                }
                            }

                            let report_info = OpenReportInfo {
                                report: 1.to_owned(),
                                location: path.to_owned(),
                                filename: filename.to_owned(),
                                date: date.format("%d/%m/%Y").to_string().to_owned()
                            };

                            // send this info above as pretty string to the impl of OpenReportInfo or app-configs update recent-report
                            let report_info_j = serde_json::to_string_pretty(&report_info).unwrap();
                            // AppConfigs::update_recent_files(&mut self::AppConfigs::new(), report_info_j); // Take all the data and update the json list
                            AppConfigs::update_recent_files(report_info_j);
                        },
                        Response::OkayMultiple(files) => println!("Cannot open {:?}, must select one file", files),
                        Response::Cancel => println!("easter egg"),
                    }
                }
            });

            ui.vertical_centered(|ui: &mut Ui| {
                ui.add_space(20.);
                for a in recent_files {
                    ui.add_space(10.);
                    ui.add(Label::new(format!("{}{}", a.location, a.filename)));
                    ui.add(Label::new(a.date.as_str()));
                    let open_btn = ui.add(Button::new("Open >"));
                    ui.add_space(10.);

                    if open_btn.clicked() {
                        state_machine.state = State::OpenReport;
                    }
                }
            });
            render_footer(ctx);
        });
}

fn render_report_panel(ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
    // Create a function for this block and check if report is true or false, false means display //
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            ui.with_layout(Layout::left_to_right(), |ui: &mut Ui|{
                // Add a panel to hold all of the runnable scripts stuff
                SidePanel::left("runnable_scripts").show(ctx, |ui: &mut Ui|{
                    ui.add(Label::new("Scripts to run!").monospace());
                    // List of vector information for each possible test that can run, have fun implementing lol
                });
                // Add a panel to hold the report / report stats depending on size
                CentralPanel::default().show(ctx, |ui: &mut Ui|{
                    ui.add(Label::new("View report stats!").monospace());
                });
                // Add a panel to hold all of the company information
                SidePanel::right("company_information").show(ctx, |ui: &mut Ui|{
                    ui.add(Label::new("Company information!").monospace());
                    // This information gets set when
                });
            });
        });
}

fn render_new_report_panel(ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>, report_values: &mut NewReportValues, state_machine: &mut StateMachine) {
    // Create panel for report
    CentralPanel::default().show(ctx, |ui: &mut Ui| {
        ui.vertical_centered(|ui: &mut Ui| {
            ui.add_space(40.);
            ui.add(Label::new("New Report Name:").monospace());
            let report_name = ui.add(TextEdit::singleline(&mut report_values.report_name_string));

            ui.add_space(10.);
            ui.add(Label::new("New Report Save Location:").monospace());
            let save_location = ui.add(TextEdit::singleline(&mut report_values.save_location_string));

            ui.add_space(10.);
            ui.add(Label::new("Choose to automate or manually run tests:").monospace());

            ui.radio_value(&mut report_values.auto_man_radio, Enum::Automatic, "Automatic");
            ui.radio_value(&mut report_values.auto_man_radio, Enum::Manual, "Manual");

            ui.add_space(10.);
            ui.add(Label::new("List all test you want to run:").monospace());

            ui.checkbox(&mut report_values.ip_sniffer_bool, "Ip Sniffer");
            ui.checkbox(&mut report_values.port_sniffer_bool, "Port Sniffer");
            ui.checkbox(&mut report_values.retrieve_banner_bool, "Retrieve Banners");
            ui.checkbox(&mut report_values.vuln_scanner_bool, "Scan Vulnerabilities");
            ui.checkbox(&mut report_values.ssh_brute_bool, "Brute Force SSH");
            ui.checkbox(&mut report_values.ftp_brute_bool, "Brute Force FTP");
            let all_check = ui.checkbox(&mut report_values.all_bool, "All tests");

            if all_check.clicked() {
                report_values.ip_sniffer_bool = !report_values.ip_sniffer_bool;
                report_values.port_sniffer_bool = !report_values.port_sniffer_bool;
                report_values.retrieve_banner_bool = !report_values.retrieve_banner_bool;
                report_values.vuln_scanner_bool = !report_values.vuln_scanner_bool;
                report_values.ssh_brute_bool = !report_values.ssh_brute_bool;
                report_values.ftp_brute_bool = !report_values.ftp_brute_bool;
            }

            ui.add_space(10.);

            let create_report_btn = ui.add(Button::new("Create"));

            if create_report_btn.clicked() {
                // Create the report
                // Load the report and save it right away
                // Check if text fields have data and that one checkbox is clicked at least before creating the new report
                if !report_values.report_name_string.is_empty() || !report_values.save_location_string.is_empty() {
                    // save the newly created document (as a docx file), (path should be more dynamic then this lol. Maybe use the input data from user
                    let mut docx = Docx::default();

                    let para = Paragraph::default().push_text(format!("Document: \n{}", report_values.report_name_string));
                    docx.document.push(para);

                    docx.write_file(format!("{}{}", report_values.save_location_string, report_values.report_name_string)).unwrap();

                    state_machine.state = State::OpenReport;
                } else {
                    if report_values.report_name_string.is_empty() {
                        ui.label("Please enter a report name to save");
                    } else if report_values.save_location_string.is_empty() {
                        ui.label("Please enter a location to save to.");
                    } else {
                        ui.label("Please enter a report name and location to save to.");
                    }
                }
            }
        });
    });
}

fn render_footer(ctx: &CtxRef) {
    TopBottomPanel::bottom("footer").show(ctx, |ui: &mut Ui| {
        ui.vertical_centered(|ui: &mut Ui| {
            ui.add_space(10.);
            ui.add(Label::new("Ivory Coding © 2020").monospace());
            ui.add_space(10.);
        });
    });
}

fn main() {
    let app = StartPage::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(950., 580.));
    run_native(Box::new(app), win_option);
}
