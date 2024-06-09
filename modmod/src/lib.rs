mod book;
mod exercises;
mod io;
mod load;
mod slides;

use self::{
    book::{Book, BookBuilder, ChapterBuilder, SectionBuilder},
    load::{Load, TrackDef},
};
use error_stack::{IntoReport, Report, Result, ResultExt};
use exercises::{
    ExerciseCollection, ExerciseCollectionBuilder, ModuleExercisesBuilder, UnitExercisesBuilder,
};
use io::PathExt;
use load::Indexed;
use slides::{SlideDeckBuilder, SlidesPackage, SlidesPackageBuilder};
use std::{
    fmt::{self, Display},
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Track {
    pub name: String,
    pub modules: Vec<Indexed<Module>>,
}

impl Track {
    pub fn load_toml_def(path: impl AsRef<Path>) -> Result<Self, LoadTrackError> {
        let def = TrackDef::load(path.as_ref(), None).change_context(LoadTrackError)?;
        def.resolve().change_context(LoadTrackError)
    }

    pub fn render(
        &self,
        output_dir: impl AsRef<Path>,
        slide_url_base: &str,
        clear_output: bool,
    ) -> Result<(), LoadTrackError> {
        let output_dir = output_dir.as_ref();
        output_dir.create_dir_all()?;
        let output_dir = &output_dir
            .canonicalize()
            .into_report()
            .change_context(LoadTrackError)?;

        if output_dir.exists() {
            if clear_output {
                // remove output dir and contents
                fs::remove_dir_all(output_dir)
                    .into_report()
                    .change_context(LoadTrackError)?;
            } else {
                // Return error if output dir is not empty
                let None = fs::read_dir(output_dir)
                    .into_report()
                    .change_context(LoadTrackError)?
                    .next()
                else {
                    return Err(Report::new(LoadTrackError)
                        .attach_printable("Output directory is not empty"));
                };
            }
        }
        // Ensure output dir exists
        output_dir.create_dir_all()?;

        // Render the modules in the track
        let mut book_builder = Book::builder(&self.name);
        let mut slides_builder = SlidesPackage::builder(&self.name);
        let mut exercises_builder = ExerciseCollection::builder();

        self.modules.iter().try_for_each(|module| {
            module.render(
                &mut book_builder,
                &mut slides_builder,
                &mut exercises_builder,
            )
        })?;

        // Build and render exercise packages
        let exercises = exercises_builder.build();
        let exercise_paths = exercises
            .render(output_dir)
            .change_context(LoadTrackError)?;
        // Build and render the exercise book
        let book = book_builder.build();
        book.render(&exercise_paths, output_dir)
            .change_context(LoadTrackError)?;

        // Build and render the slides package
        let slides_package = slides_builder.build();
        slides_package
            .render(output_dir, slide_url_base)
            .change_context(LoadTrackError)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub description: String,
    pub units: Vec<Indexed<Unit>>,
}

impl Indexed<Module> {
    fn render<'me>(
        &'me self,
        book_builder: &mut BookBuilder<'me>,
        slides: &mut SlidesPackageBuilder<'me>,
        exercises: &mut ExerciseCollectionBuilder<'me>,
    ) -> Result<(), LoadTrackError> {
        let Indexed {
            data,
            index: module_index,
        } = self;

        let mut chapter = book_builder.chapter(&data.name, *module_index);
        let mut module_exercises = exercises.module(&data.name, *module_index);

        // Render all units in this module
        data.units.iter().try_for_each(|unit| {
            unit.render(
                &data.name,
                *module_index,
                &mut chapter,
                slides,
                &mut module_exercises,
            )
        })?;

        chapter.add();
        module_exercises.add();
        Ok(())
    }
}

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub template: Option<PathBuf>,
    pub topics: Vec<Indexed<Topic>>,
}

impl Indexed<Unit> {
    fn render<'me>(
        &'me self,
        module_name: &'me str,
        module_index: usize,
        chapter: &mut ChapterBuilder<'me, '_>,
        slides: &mut SlidesPackageBuilder<'me>,
        module_exercises: &mut ModuleExercisesBuilder<'me, '_>,
    ) -> Result<(), LoadTrackError> {
        let Indexed {
            data,
            index: unit_index,
        } = self;

        let mut section = chapter.section(module_index, *unit_index, &data.name);
        let mut deck = slides.deck(
            &data.name,
            module_name,
            module_index,
            *unit_index,
            data.template.as_deref(),
        );
        let mut unit_exercises = module_exercises.unit(&data.name, *unit_index);

        data.topics
            .iter()
            .try_for_each(|topic| topic.render(&mut section, &mut deck, &mut unit_exercises))?;

        section.add();
        deck.add();
        unit_exercises.add();

        Ok(())
    }
}

#[derive(Debug)]
pub struct Topic {
    pub name: String,
    pub exercises: Vec<Indexed<Exercise>>,
    pub summary: Vec<String>,
    pub objectives: Vec<String>,
    pub content: PathBuf,
    pub further_reading: Vec<String>,
    pub images: Vec<PathBuf>,
}

impl Indexed<Topic> {
    fn render<'me>(
        &'me self,
        section: &mut SectionBuilder<'me, '_, '_>,
        deck: &mut SlideDeckBuilder<'me, '_>,
        unit_exercises: &mut UnitExercisesBuilder<'me, '_, '_>,
    ) -> Result<(), LoadTrackError> {
        let Indexed { data, .. } = self;

        let mut slides_section = deck.section(&data.content);

        data.summary
            .iter()
            .for_each(|item| slides_section.summary(item));

        data.objectives
            .iter()
            .for_each(|obj| slides_section.objective(obj));

        data.further_reading
            .iter()
            .for_each(|item| slides_section.further_reading(item));

        data.images
            .iter()
            .for_each(|image| slides_section.image(image));

        data.exercises
            .iter()
            .try_for_each(|exercise| exercise.render(section, unit_exercises))?;

        slides_section.add();

        Ok(())
    }
}

#[derive(Debug)]
pub struct Exercise {
    pub name: String,
    pub path: PathBuf,
    pub description: PathBuf,
    pub description_images: Vec<PathBuf>,
    pub includes: Vec<String>,
}

impl Indexed<Exercise> {
    fn render<'me>(
        &'me self,
        section: &mut SectionBuilder<'me, '_, '_>,
        unit_exercises: &mut UnitExercisesBuilder<'me, '_, '_>,
    ) -> Result<(), LoadTrackError> {
        let Indexed { data, .. } = self;

        section.subsection(
            &data.name,
            &data.description,
            &data.description_images,
            &data.path,
        );

        unit_exercises.package(&data.name, &data.path, &data.includes);

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct LoadTrackError;

impl fmt::Display for LoadTrackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unable to load track")
    }
}

impl error_stack::Context for LoadTrackError {}

fn to_prefixed_tag<S, P>(s: S, p: P) -> String
where
    S: Display,
    P: Display,
{
    to_tag(format!("{p}-{s}"))
}

fn to_tag<S>(s: S) -> String
where
    S: ToString,
{
    let mut s = s.to_string();
    s.make_ascii_lowercase();
    let mut tag = String::new();
    let mut words = s.split_whitespace();

    match words.next() {
        Some(w) => tag.push_str(w),
        None => return s,
    }

    for word in words {
        tag.push('-');
        tag.push_str(word);
    }
    tag
}
