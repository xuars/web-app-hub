use common::{assets, config, utils::OnceLockExt};
use gtk::{
    License,
    glib::{
        GString,
        object::{Cast, ObjectExt},
    },
    prelude::{BuildableExt, WidgetExt},
};
use libadwaita::{
    AboutDialog, ActionRow,
    prelude::{AdwDialogExt, PreferencesRowExt},
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;

static CREDITS_DOCUMENTATION: &str = include_str!("../../../../credits/documentation.yml");
static CREDITS_TRANSLATIONS: &str = include_str!("../../../../credits/translations.yml");

#[derive(Serialize, Deserialize)]
pub struct CreditsYaml {
    name: String,
    link: Option<String>,
}
/// Downcast and translate nested ``AboutDialog`` widgets
enum AboutDialogWidget {
    Changelog(ActionRow),
    Details(ActionRow),
    Website(ActionRow),
    Support(ActionRow),
    Issue(ActionRow),
    Troubleshooting(ActionRow),
    Credits(ActionRow),
    Legal(ActionRow),
    Acknowledgements(ActionRow),
}
impl AboutDialogWidget {
    fn from_widget(widget: &gtk::Widget) -> Option<Self> {
        let id: &str = &widget.buildable_id().unwrap_or_default();
        let title: &str = if widget.has_property("title") {
            &widget.property::<GString>("title")
        } else {
            ""
        };

        match (id, title) {
            ("whats_new_row", _) | (_, "What’s _New") => {
                widget.downcast_ref().cloned().map(Self::Changelog)
            }
            ("details_row", _) | (_, "_Details") => {
                widget.downcast_ref().cloned().map(Self::Details)
            }
            ("website_row", _) | (_, "_Website") => {
                widget.downcast_ref().cloned().map(Self::Website)
            }
            ("support_row", _) | (_, "_Support Questions") => {
                widget.downcast_ref().cloned().map(Self::Support)
            }
            ("issue_row", _) | (_, "_Report an Issue") => {
                widget.downcast_ref().cloned().map(Self::Issue)
            }
            ("troubleshooting_row", _) | (_, "_Troubleshooting") => {
                widget.downcast_ref().cloned().map(Self::Troubleshooting)
            }
            // These suddenly don't have a buildable_id in inspector
            (_, "_Credits") => widget.downcast_ref().cloned().map(Self::Credits),
            (_, "_Legal") => widget.downcast_ref().cloned().map(Self::Legal),
            (_, "Acknowledgements") => widget.downcast_ref().cloned().map(Self::Acknowledgements),

            _ => None,
        }
    }

    fn translate_entry(&self) {
        match self {
            Self::Changelog(action_row) => {
                action_row.set_title(&t!("about.changelog"));
            }
            Self::Details(action_row) => {
                action_row.set_title(&t!("about.details"));
            }
            Self::Website(action_row) => {
                action_row.set_title(&t!("about.website"));
            }
            Self::Support(action_row) => {
                action_row.set_title(&t!("about.support"));
            }
            Self::Issue(action_row) => {
                action_row.set_title(&t!("about.issue"));
            }
            Self::Troubleshooting(action_row) => {
                action_row.set_title(&t!("about.troubleshooting"));
            }
            Self::Credits(action_row) => {
                action_row.set_title(&t!("about.credits"));
            }
            Self::Legal(action_row) => {
                action_row.set_title(&t!("about.legal"));
            }
            Self::Acknowledgements(action_row) => {
                action_row.set_title(&t!("about.acknowledgements"));
            }
        }
    }
}

pub fn get_dialog() -> AboutDialog {
    let license = match config::LICENSE.get_value().as_str() {
        "GPL-3.0" => License::Gpl30,
        "GPL-3.0-only" => License::Gpl30Only,
        _ => panic!("Could not convert license"),
    };

    let about_dialog = AboutDialog::builder()
        .application_icon(config::APP_ID.get_value())
        .application_name(config::APP_NAME.get_value())
        .version(config::VERSION.get_value())
        .developer_name(config::DEVELOPER.get_value())
        .license_type(license)
        .issue_url(config::ISSUES_URL.get_value())
        .release_notes(parse_release_notes_xml())
        .copyright(format!("© 2025 {}", config::DEVELOPER.get_value()))
        .documenters(parse_documenters())
        .translator_credits(parse_translators())
        .build();

    translate_about_dialog_widgets(&about_dialog);

    about_dialog
}

fn parse_release_notes_xml() -> String {
    let metainfo = assets::get_meta_info();
    let mut release_xml = String::new();

    let mut release_version = String::new();
    let mut release_count = 1;

    for line in metainfo.lines() {
        let line = line.trim();
        if line.starts_with("<release") {
            if release_count >= 5 {
                break;
            }

            let start_pattern = r#"version=""#;
            let end_pattern = r#"" date="#;
            let Some(version_start) = line.find(start_pattern) else {
                continue;
            };
            let Some(version_end) = line.find(end_pattern) else {
                continue;
            };
            let version_str = &line[version_start + start_pattern.len()..version_end];
            let (Ok(version), Ok(app_version)) = (
                Version::parse(version_str),
                Version::parse(config::VERSION.get_value()),
            ) else {
                continue;
            };
            if version != app_version {
                let _ = write!(release_xml, "<p><em>Previous version {version}</em></p>");
                release_count += 1;
            }

            let _ = write!(release_version, "{version}");
            continue;
        } else if line.starts_with("</release>") {
            release_version.clear();
            continue;
        }
        if release_version.is_empty() {
            continue;
        }

        if line.starts_with("<p>")
            || line.starts_with("<ul>")
            || line.starts_with("<ol>")
            || line.starts_with("<li>")
            || line.starts_with("</p>")
            || line.starts_with("</ul>")
            || line.starts_with("</ol>")
            || line.starts_with("</li>")
        {
            let _ = writeln!(release_xml, "{line}");
        }
    }

    release_xml
}

fn translate_about_dialog_widgets(about_dialog: &AboutDialog) {
    /// Recursive fn to translate child nested child widgets
    fn recursive_translate<F>(widget: &gtk::Widget, recursive_fn: &F)
    where
        F: Fn(&gtk::Widget),
    {
        recursive_fn(widget);

        let mut child = widget.first_child();
        while let Some(w) = child {
            recursive_translate(&w, recursive_fn);
            child = w.next_sibling();
        }
    }

    let translate_widget = &|widget: &gtk::Widget| {
        let Some(found_widget) = AboutDialogWidget::from_widget(widget) else {
            return;
        };
        found_widget.translate_entry();
    };

    recursive_translate(&about_dialog.child().unwrap(), translate_widget);
}

fn parse_documenters() -> Vec<String> {
    let yaml: Vec<CreditsYaml> = serde_yaml::from_str(CREDITS_DOCUMENTATION).unwrap_or_default();

    yaml.into_iter()
        .map(|credit| format!("{} {}", credit.name, credit.link.unwrap_or_default()))
        .collect()
}

fn parse_translators() -> String {
    let yaml: Vec<CreditsYaml> = serde_yaml::from_str(CREDITS_TRANSLATIONS).unwrap_or_default();
    let mut credits = String::new();

    for credit in yaml {
        let _ = writeln!(
            credits,
            "{} {}",
            credit.name,
            credit.link.unwrap_or_default()
        );
    }

    credits.trim().to_string()
}
