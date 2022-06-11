use std::fs::File;
use eframe::{egui::CentralPanel, epi::App, NativeOptions, run_native, egui::Ui, egui::Vec2, egui};
use eframe::egui::{TopBottomPanel, Label, CtxRef, Layout, Button, Visuals};

#[derive(PartialEq)]
enum State {
    MainPage,
    NewReport,
    OpenReport
}

struct StateMachine { state: State }

struct StartPage {
    recent_files: Vec<RecentFileCards>,
    darkmode: bool
}

struct RecentFileCards {
    report: String,
    location: String,
    filename: String,
    date: String
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::MainPage
        }
    }

    fn open_report(&mut self) {
        self.state = State::OpenReport
    }

    fn state_main(&mut self) {
        self.state = State::MainPage
    }
}

impl StartPage {
    fn new() -> StartPage {
        let config_path = "F:/ultimate-hacking-tool/src/app-configs.json";
        let config_file = File::open(config_path).unwrap();
        let config_json: serde_json::Value = serde_json::from_reader(config_file).unwrap();
        let darkmode = config_json.get("darkmode");
        let recent_reports = config_json.get("recent-reports");

        // iter is 1 - whatever is recent (couple weeks old or a month old haven't decided)
        let iter = (1..6).map(|a: i32| RecentFileCards {
            report: format!("{}", a),
            location: format!("File{}", a),
            filename: format!("{}.txt", a),
            date: format!("05/0{}/2022", a),
        });

        StartPage {
            recent_files: Vec::from_iter(iter),
            darkmode: darkmode.unwrap().as_bool().unwrap()
        }
    }
}

impl App for StartPage {
    fn update(&mut self, ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
        let mut state_machine = StateMachine::new(); // This has to be moved outside of update

        render_top_panel(ctx, frame, &mut self.darkmode, &mut state_machine); // possibly add a back arrow ←

        if state_machine.state == State::MainPage {
            render_main_panel(ctx, frame, &mut self.recent_files, &mut state_machine);
        } else if state_machine.state == State::OpenReport {
            render_report_panel(ctx, frame);
        } else if state_machine.state == State::NewReport {
            render_newreport_panel(ctx, frame);
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
                let new_report_btn = ui.add(Button::new("+"));
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
                    state_machine.state = State::NewReport
                    //Sends data to the state manager to display the new report page
                }
            });
        });
        ui.add_space(10.);
    });
}

fn render_main_panel(ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>, recent_files: &mut Vec<RecentFileCards>, state_machine: &mut StateMachine) {
    // Create a function for this block and check if report is true or false, false means display //
        CentralPanel::default().show(ctx, |ui: &mut Ui| {
            // open report label
            ui.vertical_centered(|ui: &mut Ui| {
                ui.add_space(10.);
                let open_report_btn = ui.add(Button::new("open report..."));
                ui.add_space(10.);

                if open_report_btn.clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        // Once we have the path, we can open the report
                        // Send info to the state manager to display the report
                        state_machine.state = State::OpenReport;
                    }
                }
            });

            // Card display example for recent reports (one for each in the cards)
            ui.vertical_centered(|ui: &mut Ui| {
                ui.add_space(20.);
                for a in recent_files {
                    ui.add_space(10.);
                    ui.add(Label::new(format!("{}/{}", a.location, a.filename)));
                    ui.add(Label::new(a.date.as_str()));
                    let open_btn = ui.add(Button::new("Open >"));
                    ui.add_space(10.);

                    if open_btn.clicked() {
                        println!("Report: {} opening {}/{}", a.report.as_str(), a.location.as_str(), a.filename.as_str());
                        // Use a.location.as_str() and a.filename.as_str() to open the file at the location
                        // Sends info to the state manager to display the report
                    }
                }
            });
            ////////////////////////////////////////////////////////////////////////////////
            render_footer(ctx);
        });
}

fn render_report_panel(ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
    // Create a function for this block and check if report is true or false, false means display //
        CentralPanel::default().show(ctx, |ui: &mut Ui| {

        });
}

// Will open up its own window and data will be passed back to the main startpage
fn render_newreport_panel(ctx: &CtxRef, frame: &mut eframe::epi::Frame<'_>) {
    // Create panel for report
    CentralPanel::default().show(ctx, |ui: &mut Ui| {
        ui.vertical_centered(|ui: &mut Ui| {
            ui.add_space(10.);
            ui.add(Label::new("-----New Report-----").monospace());
            ui.add_space(10.);
            ui.add(Label::new("New Report Name:").monospace());
            // Text input for report name
            ui.add_space(10.);
            ui.add(Label::new("New Report Location:").monospace());
            // Text input for report location
            ui.add_space(10.);
            ui.add(Label::new("Choose to automate or manually run tests:").monospace());
            // Add radio buttons for automation or manual
            ui.add_space(10.);
            ui.add(Label::new("List all test you want to run:").monospace());
            // text edit for list of tests to run (can be hard coded for now)
            // Button to create report
            ui.add_space(10.);
            let create_report_btn = ui.add(Button::new("Start Report"));
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
