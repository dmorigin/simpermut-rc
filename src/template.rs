
use std::collections::HashMap;
use std::fs::{File};
use std::io::prelude::*;
use std::result::{Result};
use std::io::{Error, ErrorKind};
use regex::Regex;
use std::cell::RefCell;

#[derive(Default)]
pub struct Template {
    alias: String,
    file: String,
    data: String,
    variables: RefCell<HashMap<String, String>>,
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
                    variables: RefCell::new(HashMap::new()),
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
                f.write(String::from(tpl).as_bytes()).unwrap();
                Ok(true)
            },
            Err(err) => Err(err)
        }
    }

    pub fn list_vars(&self) -> Result<Vec<String>, Error> {
        let mut keys: Vec<String> = Vec::new();

        for (key, _value) in self.variables.borrow_mut().iter() {
            keys.push(key.clone());
        }

        Ok(keys)
    }

    pub fn var_exist(&self, var: &str) -> bool {
        for (key, _value) in self.variables.borrow_mut().iter() {
            if key == var {
                return true;
            }
        }

        return false;
    }

    pub fn set_var(&self, var: &str, value: &str) -> Result<bool, Error> {
        match self.variables.borrow_mut().get_mut(var) {
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
            tpl = tpl.replace(&format!("#[[import={}]]", import.file), &import._compile().unwrap());
        }

        // replace variables
        for (var, value) in self.variables.borrow_mut().iter() {
            tpl = tpl.replace(&format!("#[[var={}]]", var), value);
        }

        Ok(tpl)
    }

    /// Internal compiler function
    /// 
    /// It is used to capture all imports with assigning variables.
    fn _compile(&self) -> Result<String, Error> {
        let mut tpl: String = self.data.clone();

        // insert imports
        for import in self.imports.iter() {
            tpl = tpl.replace(&format!("#[[import={}]]", import.file), &import._compile().unwrap());
        }

        Ok(tpl)
    }

    fn aquire_variables(&self) {
        let regex = Regex::new("#\\[\\[var=(.*)\\]\\]").unwrap();
        for i in regex.captures_iter(&self.data) {
            self.variables.borrow_mut().insert(String::from(&i[1]), String::new());
        }

        // aquire variables from imported templates
        for tpl in self.imports.iter() {
                self.variables.borrow_mut().extend(tpl.variables.borrow_mut().clone());
        }
    }

    fn aquire_imports(&mut self) {
        let regex = Regex::new("#\\[\\[import=(.*)\\]\\]").unwrap();
        for i in regex.captures_iter(&self.data) {
            let tpl = Template::load("", &i[1]).unwrap();
            self.imports.push(tpl);
        }
    }

    pub fn get_alias(&self) -> String {
        self.alias.clone()
    }
}
