use thiserror::Error;
use serde::{Serialize, Deserialize};
use paste::paste;

macro_rules! text_enum {
    (
    $(#[$m:meta])*
    $v:vis enum $id:ident {
        $($name:ident => $lit:literal),* $(,)?
    }) => {
        $(#[$m])*
        $v enum $id {
            $($name,)*
        }

        paste! {
        #[derive(Debug, thiserror::Error, Copy, Clone, PartialEq, Eq)]
        #[error("Invalid Value")]
        struct [<Invalid $id>];
        }

        impl core::fmt::Display for $id {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
                write!(f, "{}",
                    match self {
                        $($id::$name =>  $lit),*
                    })
            }
        }

        impl core::str::FromStr for $id {
            paste! {
            type Err = [<Invalid $id>];
            }

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($lit => Ok($id::$name),)*
                    _ => Err(paste! { [<Invalid $id>] }),
                }
            }
        }
    };
}

struct SearchRequest {
    query: SearchQuery,
    fields: Option<String>,
    sort: Option<SortOptions>,
    language: Option<Language>,
    offset: Option<SearchOffset>,
}

struct SearchQuery {
    generic: Option<String>,
    title: Vec<String>,
    author: Vec<String>,
    place: Vec<String>,
    person: Vec<String>,
    language: Vec<Language>,
    publisher: Vec<String>,
    publish_year: YearRange,
    first_publish_year: YearRange,
    title_suggest: Option<String>,
    birth_date: usize,
}

text_enum! {
    #[derive(Debug, serde_with::DeserializeFromStr, serde_with::SerializeDisplay)]
    enum SortOptions {
        Editions => "editions",
        Old => "old",
        New => "new",
        Rating => "rating",
        RatingAscending => "rating asc",
        RatingDescending => "rating desc",
        ReadingLog => "readinglog",
        WantToRead => "want_to_read",
        CurrentlyReading => "currently_reading",
        AlreadyRead => "already_read",
        Title => "title",
        Scans => "scans",
        LccSort => "lcc_sort",
        LccSortAscending => "lcc_sort asc",
        LccSortDescending => "lcc_sort desc",
        DdcSort => "ddc_sort",
        DdcSortAscending => "ddc_sort asc",
        DdcSortDescending => "ddc_sort desc",
        EbookAccess => "ebook_access",
        EbookAccessAscending => "ebook_access asc",
        EbookAccessDescending => "ebook_access desc",
        Key => "key",
        KeyAscending => "key asc",
        KeyDescending => "key desc",
        Random => "random",
        RandomAscending => "random asc",
        RandomDescending => "random desc",
        RandomHourly => "random.hourly",
        RandomDaily => "random.daily",
    }
}

text_enum! {
    #[derive(Debug, serde_with::DeserializeFromStr, serde_with::SerializeDisplay)]
    enum Language {
        English => "en",
        Spanish => "es",
        Japanese => "ja",
    }
}


enum SearchOffset {
    Offset {
        offset: usize,
        limit: usize,
    },
    Page {
        page: usize,
        limit: usize,
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchResponse {
    num_found: usize,
    start: usize,
    num_found_exact: bool,
    docs: Vec<Document>,
    q: String,
    offset: Option<usize>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Document {
    key: String,
    r#type: DocumentType,
    seed: Vec<String>,
    title: String,
    title_suggest: String,
    title_sort: String,
    edition_count: usize,
    edition_key: Vec<String>,
    publish_date: Vec<String>,
    publish_year: Vec<usize>,
    first_publish_year: usize,
    number_of_pages_median: usize,
    oclc: Vec<String>,
    lcc: Vec<String>,
    isbn: Vec<String>,
    last_modified_i: usize,
    ebook_count_i: usize,
    ebook_access: EbookAccess,
    has_fulltext: bool,
    public_scan_b: bool,
    cover_edition_key: String,
    cover_i: usize,
    first_sentence: Vec<String>,
    publisher: Vec<String>,
    language: Vec<Language>,
    author_key: Vec<String>,
    author_name: Vec<String>,
    subject: Vec<String>,
    id_goodreads: Vec<String>,
    id_librarything: Vec<String>,
    publisher_facet: Vec<String>,
    subject_facet: Vec<String>,
    lcc_sort: String,
    author_facet: Vec<String>,
    subject_key: Vec<String>
}


#[derive(Serialize, Deserialize)]
enum EbookAccess {
    #[serde(todo!())]
    NoEbook,
    PrintDisabled,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum DocumentType {
    Work
}

#[derive(Debug, Default)]
pub struct YearRange {
    start: Option<usize>,
    end: Option<usize>,
}

#[derive(Debug, Error)]
#[error("Start in year {0}, after end year {1}")]
struct StartsAfterEndError(usize, usize);

impl YearRange {
    fn new(start: Option<usize>, end: Option<usize>) -> Result<Self, StartsAfterEndError> {
        match (start, end) { 
            (Some(start), Some(end)) if end < start => Err(StartsAfterEndError(start, end)),
            _ => Ok(Self {start, end})
        }
    }
}
