# mdbook-environments

A mdbook preprocessor adding LaTeX styled environments. It uses the template engine upon for rendering. An example book has been included.

### Built-in Environments

The following environments are included by default:

- center
- boxed [background-color] [border-color]
- proof
- theorem [theorem-name]
- lemma [lemma-name]
- proposition [proposition-name]
- remark [remark-name]
- definition [definition-name]

The environments theorem, lemma, proposition, remark and definition all use the counter_id of `theorem`.

You can disable the built-in environments by passing `--no-builtin`.

### Custom Environment

Custom environments can be passed through a config file (pass the flag `-c <file.json>`). Each environment has access to the following variables:

- name: Name of the environment
- has_arg: Array of booleans, indicating whether or not a specific argument (0-9) is provided.
- args: Array of arguments
- body: Environment body
- counter: String for counter. If the environment doesn't use a counter it defaults to an empty string. 

Example configuration:

```json
{
    "environments": {
        "custom": {
            "template": "**Custom Template:** {{body}}"
        },
        "custom2": {
            "template": "**Custom {% if has_arg.0 %}** ({{args.0}})**{% endif %}{{counter}}.** {{body}}",
            "counter_id": "custom"
        }
    }
}
```

### Known issues:

- Environments cannot be nested.