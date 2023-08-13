use std::{borrow::Cow, collections::HashMap, fs, io::Error};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::config::Config;

#[derive(JsonSchema, Clone, Copy, Deserialize, Serialize, Debug)]
pub struct PrintingMetadata {
    paper_width_mm: f64,
    paper_height_mm: f64,
    page_margin_left_mm: f64,
    page_margin_right_mm: f64,
    page_margin_top_mm: f64,
    page_margin_bottom_mm: f64,
}

impl Default for PrintingMetadata {
    fn default() -> Self {
        PrintingMetadata {
            paper_width_mm: 210.0,
            paper_height_mm: 297.0,
            page_margin_left_mm: 25.4,
            page_margin_right_mm: 25.4,
            page_margin_top_mm: 25.4,
            page_margin_bottom_mm: 25.4,
        }
    }
}

#[derive(JsonSchema, Serialize, Deserialize, Debug, Clone)]
pub struct TemplateMetadata {
    unique_id: String,
    display_name: String,
    print_options: PrintingMetadata,
    required_fonts: Vec<String>,
}

impl Default for TemplateMetadata {
    fn default() -> Self {
        TemplateMetadata {
            unique_id: "default".to_string(),
            display_name: "default".to_string(),
            print_options: PrintingMetadata::default(),
            required_fonts: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PreparedTemplate {
    metadata: TemplateMetadata,
    document_template: String,
    preview_template: String,
    variables: Vec<String>,
}

impl Default for PreparedTemplate {
    fn default() -> Self {
        PreparedTemplate {
            metadata: TemplateMetadata::default(),
            document_template: "{{body}}".to_string(),
            preview_template: "{{body}}".to_string(),
            variables: vec!["body".to_string()],
        }
    }
}

impl PreparedTemplate {
    pub fn load(template: &str, config: Config) -> Result<Self, Error> {
        let cfg = config.clone();
        let template_dir_path =
            shellexpand::env(&cfg.template_dir).unwrap_or(Cow::from(config.clone().template_dir));

        let template_path = format!("{}/{}", template_dir_path, template);
        let template_metadata_str = fs::read_to_string(format!("{}/template.json", template_path))?;
        let template_metadata: TemplateMetadata =
            serde_json::from_str(&template_metadata_str).unwrap();
        let mut document_template_str =
            fs::read_to_string(format!("{}/template.document.html", template_path))?;
        let mut body_template_str =
            fs::read_to_string(format!("{}/template.body.html", template_path))?;
        let css = fs::read_to_string(format!("{}/template.css", template_path))?;
        let mut all_variables: Vec<String> = Vec::new();

        {
            let mut open_brace = false;
            let mut half_brace = false;
            let mut current_variable = String::new();
            let mut variables: Vec<String> = Vec::new();

            let mut i = 0;

            while i < body_template_str.len() {
                let char = body_template_str.chars().nth(i).unwrap();
                if char == '{' && half_brace == false {
                    half_brace = true;
                }

                if char == '{' && half_brace == true {
                    open_brace = true;
                    half_brace = false;
                }

                if char != '{' && half_brace == true {
                    half_brace = false;
                }

                if char == '}'
                    && body_template_str.chars().nth(i + 1).unwrap() == '}'
                    && open_brace == true
                {
                    open_brace = false;
                    half_brace = false;
                    variables.push(current_variable.clone());
                    current_variable.clear();
                    i += 1;
                }

                if char != '{' && open_brace == true {
                    current_variable.push(char);
                }

                i += 1;
            }

            for v in &variables {
                if v.trim() == "css" {
                    body_template_str = body_template_str.replace(&format!("{{{{{v}}}}}"), &css);
                }
                all_variables.push(v.to_string());
            }
        }

        {
            let mut open_brace = false;
            let mut half_brace = false;
            let mut current_variable = String::new();
            let mut variables: Vec<String> = Vec::new();

            let mut i = 0;

            while i < document_template_str.len() {
                let char = document_template_str.chars().nth(i).unwrap();
                if char == '{' && half_brace == false {
                    half_brace = true;
                }

                if char == '{' && half_brace == true {
                    open_brace = true;
                    half_brace = false;
                }

                if char != '{' && half_brace == true {
                    half_brace = false;
                }

                if char == '}'
                    && document_template_str.chars().nth(i + 1).unwrap() == '}'
                    && open_brace == true
                {
                    open_brace = false;
                    half_brace = false;
                    variables.push(current_variable.clone());
                    current_variable.clear();
                    i += 1;
                }

                if char != '{' && open_brace == true {
                    current_variable.push(char);
                }

                i += 1;
            }

            for v in &variables {
                if v.trim() == "css" {
                    document_template_str =
                        document_template_str.replace(&format!("{{{{{v}}}}}"), &css);
                }

                if v.trim() == "body" {
                    document_template_str =
                        document_template_str.replace(&format!("{{{{{v}}}}}"), &body_template_str);
                }
                all_variables.push(v.to_string());
            }
        }

        Ok(PreparedTemplate {
            metadata: template_metadata,
            document_template: document_template_str,
            preview_template: body_template_str,
            variables: all_variables,
        })
    }

    pub fn get_preview(&self, content: &str, frontmatter: &HashMap<String, String>) -> String {
        let mut preview = self.preview_template.clone();

        for v in &self.variables {
            if v.trim() == "body" {
                preview = preview.replace(&format!("{{{{{v}}}}}"), &content);
            }

            if v.trim().starts_with("fm.") {
                let fm_key = v.trim().replace("fm.", "");
                preview = preview.replace(
                    &format!("{{{{{v}}}}}"),
                    &frontmatter.get(&fm_key).unwrap_or(&"undefined".to_string()),
                );
            }
        }

        preview
    }
}
