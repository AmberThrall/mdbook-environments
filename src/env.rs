use super::EnvBlock;
use super::Counter;
use anyhow::{anyhow, Result};
use mdbook::errors::Result as MdbookResult;
use tinytemplate::TinyTemplate;
use std::collections::HashMap;
use serde::ser::{Serialize, SerializeStruct, Serializer, SerializeMap};

use std::error::Error;

pub struct Environment<'a> {
    pub template: &'a str,
    pub counter_id: Option<String>,
}

pub enum BuiltinEnvironments {
    All,
    Center,
    Boxed,
    Proof,
    Theorem,
    Lemma,
    Proposition,
    Remark,
    Definition,
}

#[derive(Serialize)]
struct Context {
    name: String,
    has_arg: Vec<bool>,
    args: Vec<String>,
    body: String,
    counter: String,
}

pub struct Environments<'a> {
    tt: TinyTemplate<'a>,
    envs: HashMap<String, Environment<'a>>,
}

impl<'a> Default for Environments<'a> {
    fn default() -> Self {
        Self {
            tt: TinyTemplate::new(),
            envs: HashMap::new(),
        }
    }
}

impl<'a> Environments<'a> {
    pub fn register(&mut self, name: &'a str, env: Environment<'a>) -> Result<(), tinytemplate::error::Error> {
        self.tt.add_template(name, &env.template)?;
        self.envs.insert(String::from(name), env);
        Ok(())
    }

    pub fn get_counters(&self) -> HashMap<String, Counter> {
        let mut counters = HashMap::new();
        for (_, env) in &self.envs {
            if let Some(ref counter_id) = env.counter_id {
                counters.insert(counter_id.clone(), Counter::default());
            }
        }

        counters
    }

    pub fn register_builtin(&mut self, env: BuiltinEnvironments) {
        match env {
            BuiltinEnvironments::All => {
                self.register_builtin(BuiltinEnvironments::Center);
                self.register_builtin(BuiltinEnvironments::Boxed);
                self.register_builtin(BuiltinEnvironments::Proof);
                self.register_builtin(BuiltinEnvironments::Theorem);
                self.register_builtin(BuiltinEnvironments::Lemma);
                self.register_builtin(BuiltinEnvironments::Proposition);
                self.register_builtin(BuiltinEnvironments::Remark);
                self.register_builtin(BuiltinEnvironments::Definition);
            },
            BuiltinEnvironments::Center => self.register("center", Environment { 
                template: r#"<center>

{body}

</center>"#, 
                counter_id: None 
            }).unwrap(),
            BuiltinEnvironments::Boxed => self.register("boxed", Environment { 
                template: r#"<div style="width: 100%; padding: 5px; border: 2px solid {{if has_arg.1}}{args.1}{{else}}black{{endif}}; {{if has_arg.0}}background: {args.0}{{endif}};">

{body}

</div>"#, 
                counter_id: None 
            }).unwrap(),
            BuiltinEnvironments::Proof => self.register("proof", Environment { 
                template: "*Proof.* {body}<p style='width: 100%; text-align: right;'>▯</p>", 
                counter_id: None 
            }).unwrap(),
            BuiltinEnvironments::Theorem => self.register("theorem", Environment { 
                template: "{{if has_arg.0}}**Theorem {counter}** ({args.0})**.**{{else}}**Theorem {counter}.**{{endif}} *{body}*", 
                counter_id: Some("theorem".to_string()), 
            }).unwrap(),
            BuiltinEnvironments::Lemma => self.register("lemma", Environment { 
                template: "{{if has_arg.0}}**Lemma {counter}** ({args.0})**.**{{else}}**Lemma {counter}.**{{endif}} *{body}*", 
                counter_id: Some("theorem".to_string()), 
            }).unwrap(),
            BuiltinEnvironments::Proposition => self.register("proposition", Environment { 
                template: "{{if has_arg.0}}**Proposition {counter}** ({args.0})**.**{{else}}**Proposition {counter}.**{{endif}} *{body}*", 
                counter_id: Some("theorem".to_string()), 
            }).unwrap(),            
            BuiltinEnvironments::Remark => self.register("remark", Environment { 
                template: "{{if has_arg.0}}**Remark {counter}** ({args.0})**.**{{else}}**Remark {counter}.**{{endif}} {body}", 
                counter_id: Some("theorem".to_string()), 
            }).unwrap(),
            BuiltinEnvironments::Definition => self.register("definition", Environment { 
                template: "{{if has_arg.0}}**Definition {counter}** ({args.0})**.**{{else}}**Definition {counter}.**{{endif}} {body}", 
                counter_id: Some("theorem".to_string()), 
            }).unwrap(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&Environment> {
        self.envs.get(&String::from(name))
    }

    pub fn render(&self, block: &EnvBlock, counter: &Counter) -> MdbookResult<String> {
        let mut has_arg = Vec::new();
        for i in 0..10 {
            has_arg.push(i < block.info.args.len());
        }

        let context = Context {
            name: block.info.name.clone(),
            has_arg,
            args: block.info.args.clone(),
            body: block.body.clone(),
            counter: format!("{}", counter),
        };
        self.tt.render(&block.info.name, &context).map_err(|e| anyhow!("Render error occurred: {}", e))
    }
}