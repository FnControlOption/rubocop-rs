use std::collections::HashMap;

use lib_ruby_parser_nodes::helpers::camelcase_to_snakecase;

type Error = Box<dyn std::error::Error>;

fn main() -> Result<(), Error> {
    // println!("cargo:warning={}", s);

    let mut cops = parse_cops("config/default.yml")?;
    cops.sort_by(|a, b| a["qualified_name"].cmp(&b["qualified_name"]));

    let departments = cops.iter().map(|c| c["department_mod"].clone());
    let mut departments = departments.collect::<Vec<_>>();
    departments.dedup();

    render_with_ast("codegen/ast/node_ref.liquid", "src/ast/node_ref.rs")?;
    render_with_ast("codegen/ast/processor.liquid", "src/ast/processor.rs")?;

    render_with_ast(
        "codegen/commissioner/visitor.liquid",
        "src/commissioner/visitor.rs",
    )?;

    render_with_ast("codegen/cop/base.liquid", "src/cop/base.rs")?;
    render_with_cops("codegen/cop/name.liquid", "src/cop/name.rs", &cops)?;

    render_with_cops("codegen/default/cops.liquid", "src/default/cops.rs", &cops)?;

    for dm in departments {
        let cops = cops.iter().filter(|c| dm == c["department_mod"]);
        let cops = cops.map(|c| c.clone()).collect::<Vec<_>>();

        render_with_cops(
            "codegen/cop/mod.liquid",
            &format!("src/cop/{dm}/mod.rs"),
            &cops,
        )?;

        render_with_cops(
            "codegen/cop/mod.liquid",
            &format!("tests/cop/{dm}/mod.rs"),
            &cops,
        )?;
    }

    for cop in cops.iter() {
        let dm = &cop["department_mod"];
        let sn = &cop["snakecase_name"];

        render_single_cop(
            "codegen/new_cop.liquid",
            &format!("src/cop/{dm}/{sn}.rs"),
            &cop,
        )?;

        render_single_cop(
            "codegen/new_cop_tests.liquid",
            &format!("tests/cop/{dm}/{sn}.rs"),
            &cop,
        )?;
    }

    Ok(())
}

fn render_with_ast(template_path: &str, output_path: &str) -> Result<(), Error> {
    let template = lib_ruby_parser_nodes::LiquidTemplate::new(template_path);
    let rendered = template.render();
    std::fs::write(output_path, rendered)?;
    Ok(())
}

fn render_with_cops(
    template_path: &str,
    output_path: &str,
    cops: &[HashMap<String, String>],
) -> Result<(), Error> {
    println!("cargo:rerun-if-changed={}", template_path);

    let template = {
        let src = std::fs::read_to_string(template_path)?;
        liquid::ParserBuilder::with_stdlib().build()?.parse(&src)?
    };

    let rendered = {
        let globals = liquid::object!({ "template": template_path, "cops": cops });
        template.render(&globals)?
    };

    std::fs::write(output_path, rendered)?;

    Ok(())
}

fn render_single_cop(
    template_path: &str,
    output_path: &str,
    cop: &HashMap<String, String>,
) -> Result<(), Error> {
    if std::path::Path::new(output_path).exists() {
        return Ok(());
    }

    println!("cargo:rerun-if-changed={}", template_path);

    let template = {
        let src = std::fs::read_to_string(template_path)?;
        liquid::ParserBuilder::with_stdlib().build()?.parse(&src)?
    };

    let rendered = {
        let globals = liquid::object!({ "template": template_path, "cop": cop });
        template.render(&globals)?
    };

    std::fs::write(output_path, rendered)?;

    Ok(())
}

fn parse_cops(config_path: &str) -> Result<Vec<HashMap<String, String>>, Error> {
    println!("cargo:rerun-if-changed={}", config_path);

    let s = std::fs::read_to_string(config_path)?;
    let config: HashMap<String, serde_yaml::Value> = serde_yaml::from_str(&s)?;

    let mut cops = Vec::new();
    for k in config.keys() {
        let qualified_name = k.clone();

        let parts = qualified_name.split('/').collect::<Vec<_>>();
        let [department_mod, camelcase_name] = parts[..] else { continue };

        let department_mod = camelcase_to_snakecase(department_mod);
        let snakecase_name = camelcase_to_snakecase(camelcase_name);

        let camelcase_name = camelcase_name.to_string();

        cops.push({
            let mut info = HashMap::new();
            info.insert("qualified_name".to_string(), qualified_name);
            info.insert("department_mod".to_string(), department_mod);
            info.insert("camelcase_name".to_string(), camelcase_name);
            info.insert("snakecase_name".to_string(), snakecase_name);
            info
        });
    }
    Ok(cops)
}
