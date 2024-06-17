use indoc::indoc;
use std::{
    collections::HashMap,
    fmt,
    path::{Path, PathBuf},
};

use error_stack::Result;

use crate::{
    io::{copy_files, PathExt, WriteExt},
    to_tag,
};

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RenderBookError;

impl fmt::Display for RenderBookError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unable to render book")
    }
}

impl error_stack::Context for RenderBookError {}

#[derive(Debug)]
pub struct Book<'track> {
    pub title: &'track str,
    pub chapters: Vec<Chapter<'track>>,
}

const IMAGE_PLACEHOLDER: &str = "#[modmod:images]";

impl<'track> Book<'track> {
    pub fn builder(title: &'track str) -> BookBuilder {
        BookBuilder {
            book: Book {
                title,
                chapters: vec![],
            },
        }
    }

    pub fn render(
        &self,
        exercise_paths: &HashMap<PathBuf, PathBuf>,
        out_dir: impl AsRef<Path>,
    ) -> Result<(), RenderBookError> {
        let book_out_dir = out_dir.as_ref().join("book");
        let book_src_dir = book_out_dir.join("src");
        book_src_dir.create_dir_all()?;

        let book_toml_path = book_out_dir.join("book.toml");
        let mut book_toml = book_toml_path.create_file()?;
        book_toml.write_all(indoc! {r#"
                [book]
                language = "en"
                multilingual = false
                
                [build]
                build-dir = "./target"
            "#})?;

        let summary_md_path = book_src_dir.join("SUMMARY.md");

        let mut summary_md = summary_md_path.create_file()?;
        summary_md.write_all("# Summary\n\n")?;

        for (chapter, chapter_i) in self.chapters.iter().zip(1..) {
            // Sadly, at the time of writing, mdbook does not allow for custom section numbering.
            // Therefore, we insert a draft chapter to keep the section numbering in sync
            summary_md.write_fmt(format_args!("- [{}]()\n", chapter.title))?;

            for (section, section_i) in chapter.sections.iter().zip(1..) {
                let section_file_name = Path::new(&to_tag(section.title)).with_extension("md");
                summary_md.write_fmt(format_args!(
                    "\t- [{}]({})\n",
                    section.title,
                    section_file_name.to_str().unwrap()
                ))?;

                let section_file_path = book_src_dir.join(&section_file_name);
                let mut section_file = section_file_path.create_file()?;
                section_file.write_fmt(format_args!(
                    "# Unit {chapter_i}.{section_i} - {}\n\n",
                    section.title
                ))?;

                if !section.subsections.is_empty() {
                    for (subsection, subsection_i) in section.subsections.iter().zip(1..) {
                        section_file.write_fmt(format_args!(
                            "## Exercise {chapter_i}.{section_i}.{subsection_i}: {}\n\n",
                            subsection.title
                        ))?;
                        let exercise_out_dir = &exercise_paths[subsection.exercise_path];
                        let book_images_subdir =
                            format!("images/{chapter_i}/{section_i}/{subsection_i}");
                        if !subsection.images.is_empty() {
                            let book_images_dir = book_src_dir.join(&book_images_subdir);
                            book_images_dir.create_dir_all()?;
                            copy_files(&subsection.images, &book_images_dir)?;
                        }

                        let content = subsection.content.read_to_string()?;
                        check_images(
                            &content,
                            &subsection.images,
                            &subsection.exercise_path.join("images"),
                        );
                        let content = content
                            // Insert exercise directory paths
                            .replace(
                                "#[modmod:exercise_dir]",
                                &exercise_out_dir.to_string_lossy(),
                            )
                            // Insert exercise references
                            .replace(
                                "#[modmod:exercise_ref]",
                                &format!("{chapter_i}.{section_i}.{subsection_i}"),
                            )
                            // Insert exercise image directory paths
                            .replace(IMAGE_PLACEHOLDER, &book_images_subdir)
                            // Convert exercise sections into subsubsections
                            .replace("\n# ", "\n### ");
                        section_file.write_fmt(format_args!("{}\n", content.trim()))?;
                    }
                } else {
                    section_file.write_all("*There are no exercises for this unit*")?;
                }
            }
            summary_md.write_all("\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Chapter<'track> {
    pub title: &'track str,
    pub sections: Vec<Section<'track>>,
    pub module_index: usize,
}

#[derive(Debug)]
pub struct Section<'track> {
    pub title: &'track str,
    pub subsections: Vec<SubSection<'track>>,
    pub module_index: usize,
    pub unit_index: usize,
}

#[derive(Debug)]
pub struct SubSection<'track> {
    pub title: &'track str,
    pub content: &'track Path,
    pub images: &'track [PathBuf],
    pub exercise_path: &'track Path,
}

pub struct BookBuilder<'track> {
    book: Book<'track>,
}

impl<'track> BookBuilder<'track> {
    pub fn chapter<'b>(
        &'b mut self,
        title: &'track str,
        module_index: usize,
    ) -> ChapterBuilder<'track, 'b> {
        ChapterBuilder {
            book_builder: self,
            chapter: Chapter {
                title,
                module_index,
                sections: vec![],
            },
        }
    }

    pub fn build(self) -> Book<'track> {
        self.book
    }
}

pub struct ChapterBuilder<'track, 'b> {
    book_builder: &'b mut BookBuilder<'track>,
    chapter: Chapter<'track>,
}

impl<'track, 'b> ChapterBuilder<'track, 'b> {
    pub fn section<'c>(
        &'c mut self,
        module_index: usize,
        unit_index: usize,
        title: &'track str,
    ) -> SectionBuilder<'track, 'b, 'c> {
        SectionBuilder {
            chapter_builder: self,
            section: Section {
                title,
                module_index,
                unit_index,
                subsections: vec![],
            },
        }
    }

    pub fn add(self) -> &'b mut BookBuilder<'track> {
        self.book_builder.book.chapters.push(self.chapter);
        self.book_builder
    }
}

pub struct SectionBuilder<'track, 'b, 'c> {
    chapter_builder: &'c mut ChapterBuilder<'track, 'b>,
    section: Section<'track>,
}

impl<'track, 'b, 'c> SectionBuilder<'track, 'b, 'c> {
    pub fn subsection(
        &mut self,
        title: &'track str,
        content: &'track Path,
        images: &'track [PathBuf],
        exercise_path: &'track Path,
    ) {
        self.section.subsections.push(SubSection {
            title,
            content,
            images,
            exercise_path,
        })
    }

    pub fn add(self) -> &'c mut ChapterBuilder<'track, 'b> {
        self.chapter_builder.chapter.sections.push(self.section);
        self.chapter_builder
    }
}

/// Scan content for #[modmod:images] references.
fn find_image_placeholders(mut content: &str) -> std::collections::HashSet<&str> {
    let mut found_images = std::collections::HashSet::new();
    loop {
        let Some(pos) = content.find(IMAGE_PLACEHOLDER) else {
            break;
        };
        content = &content[pos + IMAGE_PLACEHOLDER.len()..];
        let Some(end) = content.find(')') else {
            break;
        };
        found_images.insert((&content[..end]).trim_start_matches('/'));
        content = &content[end + 1..];
    }
    found_images
}

/// Print warning messages for non unreferenced images in the exercise image folder as well as for image references pointing to non existing images.
fn check_images(content: &str, existing_images: &[std::path::PathBuf], base_path: &Path) {
    let mut referenced_images = find_image_placeholders(content);
    let report_unused = |img: &Path| eprintln!("⚠️ Unused image: {}", img.to_string_lossy());
    for image in existing_images {
        let Ok(relative_path) = image.strip_prefix(base_path) else {
            report_unused(image);
            continue;
        };
        if !referenced_images.remove(std::borrow::Borrow::<str>::borrow(
            &relative_path.to_string_lossy(),
        )) {
            report_unused(image);
        }
    }
    if !referenced_images.is_empty() {
        eprintln!(
            "⚠️ Non existing images: {}",
            referenced_images.into_iter().collect::<Vec<_>>().join(", ")
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_image_placeholders_works_for_0_placeholders() {
        let content = r#"
...
"#;
        let placeholders = find_image_placeholders(content);
        assert_eq!(placeholders, std::collections::HashSet::new());
    }
    #[test]
    fn find_image_placeholders_works_for_2_placeholders() {
        let content = r#"
...

![](#[modmod:images]/image_1.svg)

...

![](#[modmod:images]/image_2.svg)

...
"#;
        let placeholders = find_image_placeholders(content);
        assert_eq!(
            placeholders,
            std::collections::HashSet::from(["image_1.svg", "image_2.svg"])
        );
    }
}
