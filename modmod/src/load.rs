use std::{
    any::type_name,
    fmt, fs,
    path::{Path, PathBuf},
};

use error_stack::{IntoReport, Result, ResultExt};
use serde::{de::DeserializeOwned, Deserialize};

use crate::io::PathExt;

use super::{Exercise, Module, Topic, Track, Unit};

#[derive(Debug, Deserialize)]
pub struct TrackDef {
    pub name: String,
    pub modules: Vec<PathBuf>,
}

impl PathTo<TrackDef> {
    pub fn resolve(self) -> Result<Track, HydrateTrackError> {
        let PathTo {
            data,
            path: track_path,
        } = self;
        let TrackDef {
            name,
            modules: module_paths,
        } = data;

        let mut modules = Vec::with_capacity(module_paths.len());
        let base_path = track_path.parent().unwrap();
        for (module_path, module_index) in module_paths.into_iter().zip(1..) {
            modules.push(
                ModuleDef::load(&module_path, Some(base_path))
                    .change_context(HydrateTrackError)?
                    .resolve(module_index)?,
            );
        }

        Ok(Track { name, modules })
    }
}

#[derive(Debug, Deserialize)]
pub struct ModuleDef {
    pub name: String,
    pub description: String,
    pub units: Vec<UnitDef>,
}

impl PathTo<ModuleDef> {
    fn resolve(self, module_index: usize) -> Result<Indexed<Module>, HydrateTrackError> {
        let PathTo {
            data: def,
            path: module_path,
        } = self;
        let ModuleDef {
            name,
            description,
            units: unit_defs,
        } = def;

        let mut units = Vec::with_capacity(unit_defs.len());
        let base_path = module_path.parent().unwrap();
        for (unit_def, unit_index) in unit_defs.into_iter().zip(1..) {
            units.push(unit_def.resolve(unit_index, base_path)?);
        }

        Ok(Module {
            name,
            description,
            units,
        }
        .with_index(module_index))
    }
}

#[derive(Debug, Deserialize)]
pub struct UnitDef {
    pub name: String,
    pub template: Option<PathBuf>,
    pub topics: Vec<PathBuf>,
}

impl UnitDef {
    fn resolve(
        self,
        unit_index: usize,
        base_path: &Path,
    ) -> Result<Indexed<Unit>, HydrateTrackError> {
        let UnitDef {
            name,
            template,
            topics: topic_paths,
        } = self;

        let mut topics = Vec::with_capacity(topic_paths.len());
        for (topic_path, topic_index) in topic_paths.into_iter().zip(1..) {
            topics.push(
                TopicDef::load(&topic_path, Some(base_path))
                    .change_context(HydrateTrackError)?
                    .resolve(topic_index)?,
            );
        }

        let template = match template {
            Some(t) => Some(
                base_path
                    .join(t)
                    .canonicalize()
                    .into_report()
                    .change_context(HydrateTrackError)?,
            ),
            None => None,
        };

        Ok(Unit {
            name,
            template,
            topics,
        }
        .with_index(unit_index))
    }
}

#[derive(Debug, Deserialize)]
pub struct TopicDef {
    pub name: String,
    #[serde(default)]
    pub exercises: Vec<ExerciseDef>,
    #[serde(default)]
    pub summary: Vec<String>,
    #[serde(default)]
    pub objectives: Vec<String>,
    #[serde(default = "crate::load::serde_defaults::topic_slides_md")]
    pub content: PathBuf,
    #[serde(default)]
    pub further_reading: Vec<String>,
}

impl PathTo<TopicDef> {
    fn resolve(self, topic_index: usize) -> Result<Indexed<Topic>, HydrateTrackError> {
        let PathTo {
            data: def,
            path: topic_path,
        } = self;

        let TopicDef {
            name,
            exercises: exercise_defs,
            summary,
            objectives,
            content,
            further_reading,
        } = def;

        let mut exercises = Vec::new();
        let base_path = topic_path.parent().unwrap();
        for (exercise_def, exercise_index) in exercise_defs.into_iter().zip(1..) {
            exercises.push(exercise_def.resolve(exercise_index, base_path)?)
        }

        let content = base_path
            .join(content)
            .canonicalize()
            .into_report()
            .change_context(HydrateTrackError)?;

        let images = dir_content(&base_path.join("images"))?;

        Ok(Topic {
            name,
            exercises,
            summary,
            objectives,
            content,
            further_reading,
            images,
        }
        .with_index(topic_index))
    }
}

pub fn dir_content(path: &Path) -> Result<Vec<PathBuf>, HydrateTrackError> {
    Ok(path
        .is_dir()
        .then_some(
            path.get_dir_content()
                .map(|c| c.files.into_iter().map(PathBuf::from)),
        )
        .transpose()?
        .into_iter()
        .flatten()
        .collect())
}

#[derive(Debug, Deserialize)]
pub struct ExerciseDef {
    pub name: String,
    pub path: PathBuf,
    #[serde(default = "crate::load::serde_defaults::exercise_description_md")]
    pub description: PathBuf,
    #[serde(default = "crate::load::serde_defaults::exercise_includes")]
    pub includes: Vec<String>,
}

impl ExerciseDef {
    fn resolve(
        self,
        exercise_index: usize,
        base_path: &Path,
    ) -> Result<Indexed<Exercise>, HydrateTrackError> {
        let ExerciseDef {
            name,
            path: exercise_path,
            description,
            includes,
        } = self;
        let path = base_path
            .join(exercise_path)
            .canonicalize()
            .into_report()
            .change_context(HydrateTrackError)?;
        let description = path
            .join(description)
            .canonicalize()
            .into_report()
            .change_context(HydrateTrackError)?;
        let description_images = dir_content(&path.join("images"))?;
        Ok(Exercise {
            name,
            path,
            description,
            description_images,
            includes,
        }
        .with_index(exercise_index))
    }
}

#[derive(Debug)]
pub struct LoadError(&'static str, PathBuf);

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let LoadError(ty, path) = self;
        write!(
            f,
            "unable to load item of type {ty} from path {path}",
            path = path.to_string_lossy()
        )
    }
}

impl error_stack::Context for LoadError {}

pub trait Load: DeserializeOwned + Sized + 'static {
    fn load(path: &Path, base_path: Option<&Path>) -> Result<PathTo<Self>, LoadError> {
        let path = base_path.map(|b| b.join(path)).unwrap_or(path.to_owned());
        let path = path
            .canonicalize()
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to canonicalize path {path}. Make sure the path leads to an existing file.",
                    path = path.to_string_lossy()
                )
            })
            .change_context_lazy(|| LoadError(type_name::<Self>(), path.clone()))?;
        let content = fs::read_to_string(&path)
            .into_report()
            .attach_printable_lazy(|| {
                format!(
                    "Unable to read contents of file at path {path}",
                    path = path.to_string_lossy()
                )
            })
            .change_context_lazy(|| LoadError(type_name::<Self>(), path.clone()))?;
        let data: Self = toml::from_str(&content)
            .into_report()
            .attach_printable_lazy(|| {
                format!("Unable to parse TOML file with contents '{content}'")
            })
            .change_context_lazy(|| LoadError(type_name::<Self>(), path.clone()))?;
        Ok(data.with_path(path))
    }
}

impl Load for TrackDef {}
impl Load for ModuleDef {}
impl Load for TopicDef {}

#[derive(Debug)]
pub struct PathTo<T> {
    pub data: T,
    pub path: PathBuf,
}

trait WithPath: Sized {
    fn with_path(self, path: PathBuf) -> PathTo<Self> {
        PathTo { data: self, path }
    }
}

impl<T> WithPath for T {}

#[derive(Debug)]
pub struct Indexed<T> {
    pub data: T,
    pub index: usize,
}

trait WithIndex: Sized {
    fn with_index(self, index: usize) -> Indexed<Self> {
        Indexed { data: self, index }
    }
}

impl<T> WithIndex for T {}

#[derive(Debug, Default)]
pub struct HydrateTrackError;

impl fmt::Display for HydrateTrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Error resolving track path references")
    }
}

impl error_stack::Context for HydrateTrackError {}

#[doc(hidden)]
pub mod serde_defaults {
    use std::path::PathBuf;

    pub fn exercise_description_md() -> PathBuf {
        PathBuf::from("description.md")
    }

    pub fn exercise_includes() -> Vec<String> {
        ["Cargo.toml", "Cargo.lock", "src/**/*"]
            .map(String::from)
            .to_vec()
    }

    pub fn topic_slides_md() -> PathBuf {
        PathBuf::from("slides.md")
    }
}
