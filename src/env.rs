use super::EnvBlock;
use super::Counter;
use anyhow::{anyhow, Result};
use mdbook::errors::Result as MdbookResult;
use upon::Engine;
use std::collections::HashMap;
use serde::{Serialize};

pub struct Environment {
    pub template: String,
    pub counter_id: Option<String>,
}

impl Environment {
    pub fn new(template: &str, counter_id: Option<&str>) -> Self {
        Self {
            template: String::from(template),
            counter_id: counter_id.map(|x| String::from(x)),
        }
    }
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
    engine: Engine<'a>,
    envs: HashMap<String, Environment>,
}

impl<'a> Default for Environments<'a> {
    fn default() -> Self {
        Self {
            engine: Engine::new(),
            envs: HashMap::new(),
        }
    }
}

impl<'a> Environments<'a> {
    pub fn register(&mut self, name: &str, env: Environment) -> Result<(), Box<dyn std::error::Error>> {
        self.engine.add_template(String::from(name), env.template.clone()).unwrap();
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
            BuiltinEnvironments::Center => self.register("center", Environment::new(
r#"<center>

{{body}}

</center>"#, 
                None 
            )).unwrap(),
            BuiltinEnvironments::Boxed => self.register("boxed", Environment::new(
r#"<div style="width: 100%; padding: 5px; border: 2px solid {%if has_arg.1%}{{args.1}}{%else%}black{%endif%}; {%if has_arg.0%}background: {{args.0}}{%endif%};">

{{body}}

</div>"#, 
                None 
            )).unwrap(),
            BuiltinEnvironments::Proof => self.register("proof", Environment::new(
                "*Proof.* {{body}}<p style='width: 100%; text-align: right;'>▯</p>", 
                None 
            )).unwrap(),
            BuiltinEnvironments::Theorem => self.register("theorem", Environment::new(
                "{%if has_arg.0%}**Theorem {{counter}}** ({{args.0}})**.**{%else%}**Theorem {{counter}}.**{%endif%} *{{body}}*", 
                Some("theorem"), 
            )).unwrap(),
            BuiltinEnvironments::Lemma => self.register("lemma", Environment::new(
                "{%if has_arg.0%}**Lemma {{counter}}** ({{args.0}})**.**{%else%}**Lemma {{counter}}.**{%endif%} *{{body}}*", 
                Some("theorem"), 
            )).unwrap(),
            BuiltinEnvironments::Proposition => self.register("proposition", Environment::new(
                "{%if has_arg.0%}**Proposition {{counter}}** ({{args.0}})**.**{%else%}**Proposition {{counter}}.**{%endif%} *{{body}}*", 
                Some("theorem"), 
            )).unwrap(),            
            BuiltinEnvironments::Remark => self.register("remark", Environment::new(
                "{%if has_arg.0%}**Remark {{counter}}** ({{args.0}})**.**{%else%}**Remark {{counter}}.**{%endif%} *{{body}}*", 
                Some("theorem"), 
            )).unwrap(),
            BuiltinEnvironments::Definition => self.register("definition", Environment::new( 
                "{%if has_arg.0%}**Definition {{counter}}** ({{args.0}})**.**{%else%}**Definition {{counter}}.**{%endif%} *{{body}}*", 
                Some("theorem"), 
            )).unwrap(),
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
        let template = self.engine.get_template(&block.info.name).ok_or(anyhow!("Render error occured: failed to find environment named `{}`", block.info.name))?;
        template.render(&context).map_err(|e| anyhow!("Render error occurred: {}", e))
    }
}