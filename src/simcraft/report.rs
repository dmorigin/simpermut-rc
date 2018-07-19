

use configuration::Configuration;
use std::fs::File;
use serde_json::{from_reader as read_json, Value};
use std::cell::RefCell;
use template::Template;


pub struct Report
{
    pub html: String,
    pub dps: f32
}


pub struct Generator
{
    config: Configuration,
    report_dir: String,
    reports: RefCell<Vec<Report>>,
    tpl_report: Template,
    tpl_list_entry: Template
}

impl Generator {
    pub fn new(configuration: &Configuration, reports: &str) -> Generator {
        // load templates
        let report = Template::load(&format!("{}/{}", &configuration.template_dir, "report.html")).unwrap();
        let list_entry = Template::load(&format!("{}/{}", &configuration.template_dir, "report_list_entry.html")).unwrap();

        report.set_var("best_of", &configuration.simcraft.best_of.to_string()).unwrap();
        report.set_var("report_dir", reports).unwrap();
        report.set_var("version", ::VERSION).unwrap();

        Generator {
            config: configuration.clone(),
            report_dir: String::from(reports),
            reports: RefCell::new(Vec::new()),
            tpl_report: report,
            tpl_list_entry: list_entry
        }
    }

    pub fn push(&self, json_report: &str, html_report: &str) {
        // read json report
        let fin = File::open(&json_report).unwrap();
        let json: Value = read_json(&fin).unwrap();

        //println!("Push a new report: {}", &html_report);

        // extract dps value
        let dps: f32 = json["sim"]["players"][0]["collected_data"]["dps"]["mean"]
            .to_string()
            .parse::<f32>()
            .unwrap();

        // add to list
        if self.reports.borrow().is_empty() {
            self.reports.borrow_mut().push(Report {
                html: String::from(html_report),
                dps: dps
            });
        } else {
            // borrow checker sucks :/
            let mut at: usize = 0;
            for i in self.reports.borrow().iter() {
                if dps > i.dps {
                    break;
                }

                at += 1;
            }

            self.reports.borrow_mut().insert(at, Report {
                html: String::from(html_report),
                dps: dps
            });

            // limit the number of reports
            if self.reports.borrow().len() > self.config.simcraft.best_of {
                self.reports.borrow_mut().pop();
            }
        }
    }

    pub fn compile(&self) {

        // entry container
        let mut entries: String = String::new();

        println!("Try to compile the report");

        // max dps
        let max_dps: f32 = match self.reports.borrow().first() {
            Some(v) => v.dps,
            None => 0.0
        };

        // min dps
        let min_dps: f32 = match self.reports.borrow().last() {
            Some(v) => v.dps,
            None => 0.0
        };

        println!("Min DPS: {} / Max DPS: {}", min_dps, max_dps);
        self.tpl_report.set_var("min_dps", &min_dps.to_string()).unwrap();
        self.tpl_report.set_var("max_dps", &max_dps.to_string()).unwrap();

        // list all reports
        for r in self.reports.borrow().iter() {
            // fill template
            self.tpl_list_entry.set_var("dps", &(r.dps.round() as i32).to_string()).unwrap();
            self.tpl_list_entry.set_var("val_now", &(((r.dps / max_dps) * 100.0).round() as i32).to_string()).unwrap();
            self.tpl_list_entry.set_var("html_report_file", &self._get_report_file(&r.html)).unwrap();
            self.tpl_list_entry.set_var("html_report_name", &self._get_report_name(&r.html)).unwrap();

            entries.push_str(&self.tpl_list_entry.compile().unwrap());
        }

        // fill out the basic template
        self.tpl_report.set_var("report_list", &entries).unwrap();

        // store report
        let store = &format!("{}/{}", self.report_dir, "report.html");
        Template::store(&store, &self.tpl_report.compile().unwrap()).unwrap();

        println!("Report: {}", store);
    }

    fn _get_report_name(&self, report: &str) -> String {
        if let Some(p) = String::from(report).rfind("/") {
            return String::from(&report[p..]);
        }

        String::from("unknown")
    }

    fn _get_report_file(&self, report: &str) -> String {
        let i: usize = self.report_dir.len() + 1;
        let r: String = String::from(report);

        if i < r.len() {
            return String::from(&report[i..]);
        }

        String::from("unkown")
    }
}