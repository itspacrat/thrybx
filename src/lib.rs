/// # Octave
/// one of thrybx's most important structures, serves as a baseline for scaling
/// notes and frequencies on a theoretical "piano"
/// 
/// ### Example instantiation
/// ```
/// let oct: Octave = Octave::new(440_f32, 12_f32, 2_f32) // standard tuning + even temperment
/// 
/// let freq: f32 = oct.get_note_frequency(-9); // ~261.63 hz, C4
/// -
/// let tone = Tone::new(400_u64, freq) // a C4 note lasting 0.4 seconds
/// ```
/// 
/// *Note: in even western temperment and standard tuning, a good*
/// *default Octave to start with is a root_frequency of <b><u>440</u></b>*
/// *hz (standard A4), an octave length of <b><u>12.0</u></b> notes,*
/// *and an octave scale of <b><u>2.0</u></b>, since octaves are at*
/// *2x the frequency of the base note*
pub struct Octave {
    /// # Octave Root Frequency
    /// the frequency this Octave is based on -
    /// the *0th* note in the "center" of all semitones
    /// 
    /// *Note: in even western temperment, this would likely be set*
    /// *to the traditional A4 @ <b><u>440.0</u></b> hz*
    root_freq: f32,
    /// # Octave Length (in semitones)
    /// how many notes (inclusive) it takes to get from your
    /// root frequency to the octave frequency
    /// 
    /// *Note: in even western temperment, this would be <b><u>12</u></b> semitones*
    /// *to your octave.*
    length: f32,
    /// # Octave Scale
    /// the factor an "octave" scales a frequency by
    /// 
    /// *Note: in even western temperment, this would be <b><u>2.0</u></b> (since*
    /// *traditional western octaves double frequency)*
    scale: f32,
}

/// returns a frequency for a semitone based on root_frequency, given the number of semitones
/// away from the root (semitone index = 0), using the number of notes in an octave.
pub fn frequency_from_octave_index(
    root_frequency: f32,
    semitone_index: f32,
    octave_length: f32,
    octave_scale: f32,
) -> f32 {
    root_frequency * (octave_scale.powf(semitone_index / octave_length))
}

impl Octave {
    /// form a new Octave struct to generate tone frequencies with
    pub fn new(root_frequency: f32, octave_length: f32, octave_scale: f32) -> Self {
        Octave {
            root_freq: root_frequency,
            length: octave_length,
            scale: octave_scale
        }
    }

    /// # Set Notes
    /// set the number of notes in this octave to a truncated integer
    ///
    /// *Note: in even western temperment, this would be 12.*
    pub fn set_notes(&mut self, num: f32) {
        self.length = num.floor()
    }
    /// # Get Notes
    /// get the length (in notes) of an octave for this
    /// [Octave]
    ///
    /// *Note: in even western temperment, this would be 12.*
    pub fn get_notes(&self) -> f32 {
        self.length
    }
    /// shortcut which returns the frequency of a certain semitone
    /// on this [Octave].
    ///
    /// index 0 plays the root frequency unchanged, -1 scales
    /// down one semitone, using [Octave.length] scales up one octave etc. ...
    pub fn get_note_frequency(&self, index: f32) -> f32 {
        frequency_from_octave_index(self.root_freq, index, self.length, self.scale)
    }
}

struct Tone {
    /// duration of the sine wave in milliseconds
    len_msecs: u64,
    /// frequency of the sine wave in Hz
    frequency: f32,
}
impl Tone {
    fn new(len_msecs: u64, freq: f32) -> Self {
        Self {
            len_msecs: len_msecs,
            frequency: freq,
        }
    }
}
struct Measure {}
struct NoteSequence {}