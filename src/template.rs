
use std::collections::HashMap;
use std::fs::{File};
use std::io::prelude::*;
use std::result::{Result};
use std::io::{Error, ErrorKind};
use regex::Regex;


pub struct Template {
    name: String,
    file: String,
    data: String,
    variables: HashMap<String, String>
}

impl Template {
    /*!
     * Load a template from a file.
     * 
     * name:    Name of the template.
     * file:    The location where the template is stored.
     * 
     * return:  Returns an std::result::Result with a Template instance.
     */
    pub fn load(name: &str, file: &str) -> Result<Template, Error> {
        match File::open(name) {
            Ok(mut fin) => {
                let mut tmpl = String::new();
                fin.read_to_string(&mut tmpl).expect("Can't read the template content!");

                Ok(Template {
                    name: String::from(name),
                    file: String::from(file),
                    data: tmpl,
                    variables: HashMap::new()
                })
            },
            Err(_) => { 
                Err(Error::new(ErrorKind::NotFound, 
                    format!("Cannot open template {}", name)))
            }
        }
    }

    pub fn list_vars(&self) -> Result<Vec<String>, Error> {
        let mut keys: Vec<String> = Vec::new();

        for (key, _value) in self.variables.iter() {
            keys.push(*key);
        }

        Ok(keys)
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
        // replace variables
        let mut tpl: String = self.data;

        for (var, key) in self.variables.iter() {
            tpl = tpl.replace(&format!("#[[{}]]", var), key);
        }

        Ok(tpl)
    }

    fn aquire_variables(&self) {
        let regex = Regex::new("#\\[\\[(.*)\\]\\]").unwrap();
        for i in regex.captures_iter(&self.data) {
            self.variables.insert(String::from(&i[1]), String::new());
        }
    }
}
