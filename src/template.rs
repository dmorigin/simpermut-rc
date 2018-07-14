
use std::collections::HashMap;
use std::fs::{File};
use std::io::prelude::*;
use std::result::{Result};
use std::io::{Error, ErrorKind};
use regex::Regex;


#[derive(Default)]
pub struct Template {
    alias: String,
    file: String,
    data: String,
    variables: HashMap<String, String>,
    imports: Vec<Template>
}

impl Template {

    /// Load a template from a file.
    /// 
    /// name:    Name of the template.
    /// file:    The location where the template is stored.
    /// 
    /// return:  Returns an std::result::Result with a Template instance.
    /// 
    pub fn load(alias: &str, file: &str) -> Result<Template, Error> {
        match File::open(file) {
            Ok(mut fin) => {
                let mut tmpl = String::new();
                fin.read_to_string(&mut tmpl).expect("Can't read the template content!");

                let mut tpl = Template {
                    alias: String::from(alias),
                    file: String::from(file),
                    data: tmpl,
                    variables: HashMap::new(),
                    imports: Vec::new()
                };

                tpl.aquire_imports();
                tpl.aquire_variables();

                Ok(tpl)
            },
            Err(_) => {
                Err(Error::new(ErrorKind::NotFound, 
                    format!("Cannot open template {}", file)))
            }
        }
    }

    pub fn store(file: &str, tpl: &str) -> Result<bool, Error> {
        match File::create(file) {
            Ok(mut f) => {
                // write data to disc
                f.write(String::from(tpl).as_bytes());
                Ok(true)
            },
            Err(err) => Err(err)
        }
    }

    pub fn list_vars(&self) -> Result<Vec<String>, Error> {
        let mut keys: Vec<String> = Vec::new();

        for (key, _value) in self.variables.iter() {
            keys.push(key.clone());
        }

        Ok(keys)
    }

    pub fn var_exist(&self, var: &str) -> bool {
        for (key, _value) in self.variables.iter() {
            if key == var {
                return true;
            }
        }

        return false;
    }

    pub fn set_var(&mut self, var: &str, value: &str) -> Result<bool, Error> {
        match self.variables.get_mut(var) {
            Some(mut v) => {
                *v = String::from(value);
                Ok(true)
            },
            None => {
                Err(Error::new(ErrorKind::NotFound, format!("Variable {} not exist", var)))
            }
        }
    }

    pub fn compile(&self) -> Result<String, Error> {
        let mut tpl: String = self.data.clone();

        // insert imports
        for import in self.imports.iter() {
            tpl = tpl.replace(&format!("#[[import={}", import.file), &import.compile().unwrap());
        }

        // replace variables
        for (var, value) in self.variables.iter() {
            tpl = tpl.replace(&format!("#[[var={}]]", var), value);
        }

        Ok(tpl)
    }

    fn aquire_variables(&mut self) {
        let regex = Regex::new("#\\[\\[var=(.*)\\]\\]").unwrap();
        for i in regex.captures_iter(&self.data) {
            self.variables.insert(String::from(&i[1]), String::new());
        }

        // aquire variables from imported templates
        for tpl in self.imports.iter() {
            self.variables.extend(tpl.variables.clone());
        }
    }

    fn aquire_imports(&mut self) {
        let regex = Regex::new("#\\[\\[import=(.*)\\]\\]").unwrap();
        for i in regex.captures_iter(&self.data) {
            let tpl = Template::load("", &i[1]).unwrap();
            self.imports.push(tpl);
        }
    }
}
