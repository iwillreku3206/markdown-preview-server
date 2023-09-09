use std::{borrow::Cow, collections::HashMap, fs, io::Error};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use unicode_segmentation::UnicodeSegmentation;

use crate::config::Config;

#[derive(JsonSchema, Clone, Copy, Deserialize, Serialize, Debug)]
pub struct PrintingMetadata {
    pub paper_width_mm: f64,
    pub paper_height_mm: f64,
    pub page_margin_left_mm: f64,
    pub page_margin_right_mm: f64,
    pub page_margin_top_mm: f64,
    pub page_margin_bottom_mm: f64,
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
    pub unique_id: String,
    pub display_name: String,
    pub print_options: PrintingMetadata,
    pub required_fonts: Vec<String>,
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
    pub metadata: TemplateMetadata,
    pub document_template: String,
    pub preview_template: String,
    pub variables: Vec<String>,
}

impl Default for PreparedTemplate {
    fn default() -> Self {
        PreparedTemplate {
            metadata: TemplateMetadata::default(),
            document_template: "{body}".to_string(),
            preview_template: "{body}".to_string(),
            variables: vec!["body".to_string()],
        }
    }
}

fn find_variables(template: &str) -> (Vec<String>, String) {
    let chars: Vec<&str> = template.graphemes(true).collect();
    let len = chars.len();

    let mut open = false;
    let mut i = 0;
    let mut var_name = String::new();
    let mut vars: Vec<String> = Vec::new();
    let mut document = String::new();

    'cloop: for _ in 0..(len) {
        if let Some(char) = chars.get(i) {
            let next_char = chars.get(i + 1).unwrap_or(&"");
            if ((char) == &"\\" && !(next_char == &"\\")) || char != &"\\" {
                document.push_str(char);
                if char == &"\\" {
                    document.push_str(next_char);
                }
            }
            match char {
                &"\\" => {
                    i += 1;
                }
                &"{" => {
                    i += 1;
                    if !open {
                        open = true;
                        continue 'cloop;
                    }
                }
                &"}" => {
                    open = false;
                    if var_name != String::new() {
                        vars.push(var_name.clone());
                        var_name = String::new();
                    }
                    i += 1;
                    continue 'cloop;
                }
                _ => (),
            }

            if open {
                if let None = vars.iter().find(|v| var_name.eq(*v)) {
                    var_name.push_str(char);
                }
            }

            i += 1
        }
    }
    (vars, document)
}

impl PreparedTemplate {
    pub fn load(template: &str, config: &Config) -> Result<Self, Error> {
        let cfg = config.clone();
        let template_dir_path =
            shellexpand::env(&cfg.template_dir).unwrap_or(Cow::from(config.clone().template_dir));

        log::info!("Loading templates from {}", template_dir_path);
        log::info!("Current template: {}", template);
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

        let (body_variables, body_document) = find_variables(&body_template_str);
        body_template_str = body_document;

        for v in &body_variables {
            if v.trim() == "css" {
                body_template_str = body_template_str.replace(&format!("{{{v}}}"), &css);
            }
            all_variables.push(v.to_string());
        }

        let (document_variables, document_document) = find_variables(&document_template_str);
        document_template_str = document_document;

        for v in &document_variables {
            if v.trim() == "css" {
                document_template_str = document_template_str.replace(&format!("{{{v}}}"), &css);
            }

            if v.trim() == "body" {
                document_template_str =
                    document_template_str.replace(&format!("{{{v}}}"), &body_template_str);
            }
            all_variables.push(v.to_string());
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
                preview = preview.replace(&format!("{{{v}}}"), &content);
            }

            if v.trim().starts_with("fm.") {
                let fm_key = v.trim().replace("fm.", "");
                preview = preview.replace(
                    &format!("{{{v}}}"),
                    &frontmatter.get(&fm_key).unwrap_or(&"undefined".to_string()),
                );
            }
        }

        preview
    }

    pub fn get_document(&self, content: &str, frontmatter: &HashMap<String, String>) -> String {
        let mut document = self.document_template.clone();

        for v in &self.variables {
            if v.trim() == "body" {
                document = document.replace(&format!("{{{v}}}"), &content);
            }

            if v.trim().starts_with("fm.") {
                let fm_key = v.trim().replace("fm.", "");
                document = document.replace(
                    &format!("{{{v}}}"),
                    &frontmatter.get(&fm_key).unwrap_or(&"undefined".to_string()),
                );
            }
        }

        document
    }

    pub fn landscape(&self) -> bool {
        self.metadata.print_options.paper_height_mm < self.metadata.print_options.paper_width_mm
    }
}
