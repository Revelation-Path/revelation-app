use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

/// Song category enum matching database type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(
    feature = "db",
    sqlx(type_name = "song_category", rename_all = "snake_case")
)]
pub enum SongCategory {
    Praise,
    Worship,
    Christmas,
    Easter,
    Wedding,
    Funeral,
    Youth,
    Children,
    Communion,
    Baptism,
    Prayer,
    Thanksgiving,
    Evangelism,
    Repentance,
    Faith,
    Hope,
    Love,
    SecondComing,
    Heaven,
    Trinity,
    HolySpirit,
    Salvation
}

impl SongCategory {
    /// Get Russian display name
    pub fn name_ru(&self) -> &'static str {
        match self {
            Self::Praise => "Прославление",
            Self::Worship => "Поклонение",
            Self::Christmas => "Рождественские",
            Self::Easter => "Пасхальные",
            Self::Wedding => "Свадебные",
            Self::Funeral => "Похоронные",
            Self::Youth => "Молодёжные",
            Self::Children => "Детские",
            Self::Communion => "Вечеря Господня",
            Self::Baptism => "Крещение",
            Self::Prayer => "Молитвенные",
            Self::Thanksgiving => "Благодарственные",
            Self::Evangelism => "Евангелизационные",
            Self::Repentance => "Покаяние",
            Self::Faith => "Вера",
            Self::Hope => "Надежда",
            Self::Love => "Любовь",
            Self::SecondComing => "Второе пришествие",
            Self::Heaven => "Небеса",
            Self::Trinity => "Троица",
            Self::HolySpirit => "Святой Дух",
            Self::Salvation => "Спасение"
        }
    }

    /// Get all categories
    pub fn all() -> &'static [SongCategory] {
        &[
            Self::Praise,
            Self::Worship,
            Self::Christmas,
            Self::Easter,
            Self::Wedding,
            Self::Funeral,
            Self::Youth,
            Self::Children,
            Self::Communion,
            Self::Baptism,
            Self::Prayer,
            Self::Thanksgiving,
            Self::Evangelism,
            Self::Repentance,
            Self::Faith,
            Self::Hope,
            Self::Love,
            Self::SecondComing,
            Self::Heaven,
            Self::Trinity,
            Self::HolySpirit,
            Self::Salvation
        ]
    }
}

/// Songbook (collection of songs)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Songbook {
    pub id:          Uuid,
    pub code:        String,
    pub name:        String,
    pub name_ru:     String,
    pub description: Option<String>,
    pub cover_url:   Option<String>,
    pub songs_count: i32,
    pub is_public:   bool,

    // Extended metadata
    pub year_first_published: Option<i16>,
    pub year_latest_edition:  Option<i16>,
    pub edition_name:         Option<String>,
    pub total_songs_in_print: Option<i32>,
    pub publisher:            Option<String>,
    pub editor:               Option<String>,
    pub isbn:                 Option<String>,
    pub language:             Option<String>,
    pub country:              Option<String>,
    pub denomination:         Option<String>,
    pub website_url:          Option<String>,
    pub purchase_url:         Option<String>,
    pub history:              Option<String>,
    pub notes:                Option<String>
}

/// Songbook edition (for historical tracking)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongbookEdition {
    pub id:             Uuid,
    pub songbook_id:    Uuid,
    pub edition_name:   String,
    pub year_published: i16,
    pub songs_count:    i32,
    pub publisher:      Option<String>,
    pub isbn:           Option<String>,
    pub notes:          Option<String>
}

/// Full song with all details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id:            Uuid,
    pub songbook_id:   Option<Uuid>,
    pub songbook_code: Option<String>,
    pub number:        Option<i32>,

    // Metadata
    pub title:         String,
    pub title_alt:     Option<String>,
    pub author_lyrics: Option<String>,
    pub author_music:  Option<String>,
    pub translator:    Option<String>,
    pub year_written:  Option<i16>,
    pub copyright:     Option<String>,

    // Musical info
    pub original_key:   Option<String>,
    pub tempo:          Option<i32>,
    pub time_signature: Option<String>,

    // Content
    pub content:    String, // ChordPro format
    pub first_line: String,

    // Relations
    pub categories: Vec<SongCategory>,
    pub tags:       Vec<SongTag>,

    // User-specific (populated based on current user)
    pub is_favorite:     bool,
    pub user_transpose:  i16, // last used transposition
    pub views_count:     i32,
    pub favorites_count: i32
}

/// Song summary for lists
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongSummary {
    pub id:              Uuid,
    pub songbook_id:     Option<Uuid>,
    pub songbook_code:   Option<String>,
    pub number:          Option<i32>,
    pub title:           String,
    pub author_lyrics:   Option<String>,
    pub first_line:      String,
    pub original_key:    Option<String>,
    pub has_chords:      bool,
    pub categories:      Vec<SongCategory>,
    pub is_favorite:     bool,
    pub views_count:     i32,
    pub favorites_count: i32
}

/// Song tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongTag {
    pub id:          Uuid,
    pub name:        String,
    pub name_ru:     String,
    pub usage_count: i32
}

/// Search result with highlight info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongSearchResult {
    pub song:          SongSummary,
    pub songbook_name: Option<String>,
    pub highlight:     Option<String>, // snippet with matched text
    pub rank:          f32             // search relevance
}

/// User playlist (setlist)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongPlaylist {
    pub id:          Uuid,
    pub user_id:     Uuid,
    pub church_id:   Option<Uuid>,
    pub name:        String,
    pub description: Option<String>,
    pub is_public:   bool,
    pub event_date:  Option<NaiveDate>,
    pub songs_count: i32,
    pub created_at:  DateTime<Utc>,
    pub updated_at:  DateTime<Utc>
}

/// Playlist item with song and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItem {
    pub id:                  Uuid,
    pub song:                SongSummary,
    pub position:            i16,
    pub transpose_semitones: i16,
    pub notes:               Option<String>
}

/// Song history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongHistoryEntry {
    pub song:                SongSummary,
    pub transpose_semitones: i16,
    pub viewed_at:           DateTime<Utc>
}

// ============================================================================
// ChordPro Parsing Types
// ============================================================================

/// Musical note (for transposition)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B
}

impl Note {
    /// Parse note from string (C, C#, Db, etc.)
    pub fn parse(s: &str) -> Option<(Self, bool)> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        let chars: Vec<char> = s.chars().collect();
        let base = chars[0].to_ascii_uppercase();
        let modifier = chars.get(1).copied();

        let (note, is_flat) = match (base, modifier) {
            ('C', Some('#')) => (Note::CSharp, false),
            ('C', Some('b')) => (Note::B, true), // Cb = B
            ('C', _) => (Note::C, false),
            ('D', Some('#')) => (Note::DSharp, false),
            ('D', Some('b')) => (Note::CSharp, true),
            ('D', _) => (Note::D, false),
            ('E', Some('#')) => (Note::F, false), // E# = F
            ('E', Some('b')) => (Note::DSharp, true),
            ('E', _) => (Note::E, false),
            ('F', Some('#')) => (Note::FSharp, false),
            ('F', Some('b')) => (Note::E, true), // Fb = E
            ('F', _) => (Note::F, false),
            ('G', Some('#')) => (Note::GSharp, false),
            ('G', Some('b')) => (Note::FSharp, true),
            ('G', _) => (Note::G, false),
            ('A', Some('#')) => (Note::ASharp, false),
            ('A', Some('b')) => (Note::GSharp, true),
            ('A', _) => (Note::A, false),
            ('B', Some('#')) => (Note::C, false), // B# = C
            ('B', Some('b')) => (Note::ASharp, true),
            ('B', _) => (Note::B, false),
            ('H', _) => (Note::B, false), // German notation
            _ => return None
        };

        Some((note, is_flat))
    }

    /// Convert to semitone index (0-11)
    pub fn to_semitone(self) -> u8 {
        match self {
            Note::C => 0,
            Note::CSharp => 1,
            Note::D => 2,
            Note::DSharp => 3,
            Note::E => 4,
            Note::F => 5,
            Note::FSharp => 6,
            Note::G => 7,
            Note::GSharp => 8,
            Note::A => 9,
            Note::ASharp => 10,
            Note::B => 11
        }
    }

    /// Create from semitone index
    pub fn from_semitone(semitone: u8) -> Self {
        match semitone % 12 {
            0 => Note::C,
            1 => Note::CSharp,
            2 => Note::D,
            3 => Note::DSharp,
            4 => Note::E,
            5 => Note::F,
            6 => Note::FSharp,
            7 => Note::G,
            8 => Note::GSharp,
            9 => Note::A,
            10 => Note::ASharp,
            11 => Note::B,
            _ => unreachable!()
        }
    }

    /// Transpose by semitones
    pub fn transpose(self, semitones: i32) -> Self {
        let current = self.to_semitone() as i32;
        let new = (current + semitones).rem_euclid(12) as u8;
        Self::from_semitone(new)
    }

    /// Convert to string with sharp notation
    pub fn to_sharp_string(self) -> &'static str {
        match self {
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B"
        }
    }

    /// Convert to string with flat notation
    pub fn to_flat_string(self) -> &'static str {
        match self {
            Note::C => "C",
            Note::CSharp => "Db",
            Note::D => "D",
            Note::DSharp => "Eb",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "Gb",
            Note::G => "G",
            Note::GSharp => "Ab",
            Note::A => "A",
            Note::ASharp => "Bb",
            Note::B => "B"
        }
    }
}

/// Parsed chord
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chord {
    pub root:    String,         // C, D#, Bb
    pub quality: String,         // m, 7, maj7, dim, aug, sus4, etc.
    pub bass:    Option<String>  // for slash chords like C/G
}

impl Chord {
    /// Parse chord from string like "Am7", "C#dim", "G/B"
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }

        // Find bass note (slash chord)
        let (main, bass) = if let Some(idx) = s.rfind('/') {
            let bass_part = &s[idx + 1..];
            // Verify bass is a valid note
            if Note::parse(bass_part).is_some() {
                (&s[..idx], Some(bass_part.to_string()))
            } else {
                (s, None)
            }
        } else {
            (s, None)
        };

        // Parse root note
        let chars: Vec<char> = main.chars().collect();
        if chars.is_empty() {
            return None;
        }

        let (root_end, root) = if chars.len() >= 2 && (chars[1] == '#' || chars[1] == 'b') {
            (2, format!("{}{}", chars[0], chars[1]))
        } else {
            (1, chars[0].to_string())
        };

        // Verify root is valid
        Note::parse(&root)?;

        let quality = main[root_end..].to_string();

        Some(Self {
            root,
            quality,
            bass
        })
    }

    /// Transpose chord by semitones
    pub fn transpose(&self, semitones: i32, use_flats: bool) -> Self {
        let transpose_note = |note_str: &str| -> String {
            if let Some((note, _)) = Note::parse(note_str) {
                let transposed = note.transpose(semitones);
                if use_flats {
                    transposed.to_flat_string().to_string()
                } else {
                    transposed.to_sharp_string().to_string()
                }
            } else {
                note_str.to_string()
            }
        };

        Self {
            root:    transpose_note(&self.root),
            quality: self.quality.clone(),
            bass:    self.bass.as_ref().map(|b| transpose_note(b))
        }
    }
}

impl std::fmt::Display for Chord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.bass {
            Some(bass) => write!(f, "{}{}/{}", self.root, self.quality, bass),
            None => write!(f, "{}{}", self.root, self.quality)
        }
    }
}

/// Parsed song line with chords positioned above text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongLine {
    pub text:   String,
    pub chords: Vec<PositionedChord>
}

/// Chord with position in text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionedChord {
    pub position: usize, // character position in text
    pub chord:    Chord
}

/// Song section (verse, chorus, bridge, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongSection {
    pub section_type: SongSectionType,
    pub label:        Option<String>, // "1", "2" for verses, custom labels
    pub lines:        Vec<SongLine>
}

/// Section type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SongSectionType {
    Verse,
    Chorus,
    Bridge,
    PreChorus,
    Intro,
    Outro,
    Interlude,
    Tag,
    Ending,
    Other
}

impl SongSectionType {
    pub fn name_ru(&self) -> &'static str {
        match self {
            Self::Verse => "Куплет",
            Self::Chorus => "Припев",
            Self::Bridge => "Бридж",
            Self::PreChorus => "Предприпев",
            Self::Intro => "Вступление",
            Self::Outro => "Окончание",
            Self::Interlude => "Проигрыш",
            Self::Tag => "Тег",
            Self::Ending => "Кода",
            Self::Other => ""
        }
    }
}

/// Fully parsed song structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSong {
    pub title:          Option<String>,
    pub subtitle:       Option<String>,
    pub artist:         Option<String>,
    pub composer:       Option<String>,
    pub key:            Option<String>,
    pub tempo:          Option<i32>,
    pub time_signature: Option<String>,
    pub capo:           Option<i32>,
    pub sections:       Vec<SongSection>
}

// ============================================================================
// API Request/Response Types
// ============================================================================

/// Create song request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreateSong {
    pub songbook_id: Option<Uuid>,
    pub number:      Option<i32>,

    #[validate(length(min = 1, max = 300))]
    pub title: String,

    pub title_alt:     Option<String>,
    pub author_lyrics: Option<String>,
    pub author_music:  Option<String>,
    pub translator:    Option<String>,
    pub year_written:  Option<i16>,
    pub copyright:     Option<String>,

    #[validate(length(max = 10))]
    pub original_key: Option<String>,

    #[validate(range(min = 1, max = 300))]
    pub tempo: Option<i32>,

    pub time_signature: Option<String>,

    #[validate(length(min = 1))]
    pub content: String,

    pub categories: Vec<SongCategory>,
    pub tag_ids:    Vec<Uuid>,
    pub source_url: Option<String>
}

/// Update song request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct UpdateSong {
    pub songbook_id: Option<Uuid>,
    pub number:      Option<i32>,

    #[validate(length(min = 1, max = 300))]
    pub title: Option<String>,

    pub title_alt:      Option<String>,
    pub author_lyrics:  Option<String>,
    pub author_music:   Option<String>,
    pub translator:     Option<String>,
    pub year_written:   Option<i16>,
    pub copyright:      Option<String>,
    pub original_key:   Option<String>,
    pub tempo:          Option<i32>,
    pub time_signature: Option<String>,
    pub content:        Option<String>,
    pub categories:     Option<Vec<SongCategory>>,
    pub tag_ids:        Option<Vec<Uuid>>
}

/// Create playlist request
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CreatePlaylist {
    #[validate(length(min = 1, max = 200))]
    pub name: String,

    pub description: Option<String>,
    pub church_id:   Option<Uuid>,
    pub is_public:   bool,
    pub event_date:  Option<NaiveDate>
}

/// Add song to playlist request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToPlaylist {
    pub song_id:             Uuid,
    pub transpose_semitones: Option<i16>,
    pub notes:               Option<String>
}

/// Song list filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SongFilters {
    pub songbook_id: Option<Uuid>,
    pub category:    Option<SongCategory>,
    pub tag_id:      Option<Uuid>,
    pub key:         Option<String>,
    pub search:      Option<String>,
    pub limit:       Option<i64>,
    pub offset:      Option<i64>,
    pub sort_by:     Option<SongSortBy>
}

/// Sort options for songs
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SongSortBy {
    #[default]
    Title,
    Number,
    ViewsDesc,
    FavoritesDesc,
    RecentlyAdded,
    HasChordsFirst,
    NoChordsFirst
}
