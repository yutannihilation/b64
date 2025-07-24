use base64::{
    alphabet,
    engine::{general_purpose, DecodePaddingMode},
    prelude::*,
    read::DecoderReader,
    write::EncoderStringWriter,
};
use itertools::Itertools;
use savvy::{
    savvy, savvy_err, NotAvailableValue as _, NullSexp, NumericScalar, OwnedListSexp, OwnedRawSexp,
    OwnedStringSexp, Sexp, StringSexp, TypedSexp,
};
use std::io::Read;

#[savvy]
struct GeneralPurpose(base64::engine::GeneralPurpose);

#[savvy]
fn encode_(what: Sexp, engine: &GeneralPurpose) -> savvy::Result<Sexp> {
    let eng = &engine.0;
    match what.into_typed() {
        TypedSexp::String(s) => eng.encode(s.iter().next().unwrap_or_default()).try_into(),
        TypedSexp::Raw(r) => eng.encode(r.as_slice()).try_into(),
        _ => Err(savvy_err!("Unsupported type")),
    }
}

#[savvy]
fn encode_vectorized_(what: Sexp, engine: &GeneralPurpose) -> savvy::Result<Sexp> {
    let eng = &engine.0;
    match what.into_typed() {
        TypedSexp::String(s) => {
            let iter = s.iter().map(|s| {
                if s.is_na() {
                    s.to_string()
                } else {
                    let to_encode = s.as_bytes();
                    eng.encode(to_encode)
                }
            });

            Ok(OwnedStringSexp::try_from_iter(iter)?.into())
        }
        TypedSexp::List(r) => {
            let iter = r
                .iter()
                .map(|(_, b)| match b.into_typed() {
                    TypedSexp::Null(_) => Ok(<&str>::na().to_string()),
                    TypedSexp::Raw(r) => Ok(eng.encode(r.as_slice())),
                    _ => Err(savvy_err!("Unsupported type")),
                })
                .collect::<savvy::Result<Vec<String>>>()?;

            Ok(OwnedStringSexp::try_from_iter(iter)?.into())
        }
        _ => Err(savvy_err!("Unsupported type")),
    }
}

#[savvy]
fn encode_file_(path: &str, engine: &GeneralPurpose) -> savvy::Result<Sexp> {
    let eng = &engine.0;
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut encoder = EncoderStringWriter::new(eng);
    std::io::copy(&mut reader, &mut encoder).unwrap();
    encoder.into_inner().try_into()
}

/// Utility Functions
///
/// Functions to perform common tasks when working with base64 encoded strings.
///
/// @details
///
/// `b64_chunk()` splits a character vector of base64 encoded strings into chunks of a
/// specified width.
///
/// `b64_wrap()` wraps a character vector of base64 encoded strings with a newline character.
///
/// @returns
///
/// - `b64_chunk()` returns a list of character vectors.
/// - `b64_wrap()` returns a scalar character vector.
///
/// @examples
/// encoded <- encode("Hello, world!")
/// chunked <- b64_chunk(encoded, 4)
/// chunked
///
/// b64_wrap(chunked, "\n")
/// @param width a numeric scalar defining the width of the chunks. Must be divisible by 4.
/// @param encoded a character vector of base64 encoded strings.
/// @export
/// @rdname utils
#[savvy]
fn b64_chunk(encoded: StringSexp, width: NumericScalar) -> savvy::Result<Sexp> {
    let width = width.as_i32()?;

    if width % 4 != 0 {
        return Err(savvy_err!("Chunk size must be a multiple of 4."));
    }

    let mut result = OwnedListSexp::new(encoded.len(), false)?;

    for (i, s) in encoded.iter().enumerate() {
        if s.is_na() {
            result.set_value(i, OwnedStringSexp::new(0)?)?;
        } else {
            let v = OwnedStringSexp::try_from_iter(
                s.chars()
                    .chunks(width as usize)
                    .into_iter()
                    .map(|chunk| chunk.collect::<String>()),
            )?;

            result.set_value(i, v)?;
        }
    }

    result.into()
}

/// @param chunks a character vector of base64 encoded strings.
/// @param newline a character scalar defining the newline character.
/// @export
/// @rdname utils
#[savvy]
fn b64_wrap(chunks: Sexp, newline: &str) -> savvy::Result<Sexp> {
    match chunks.into_typed() {
        TypedSexp::List(l) => l
            .iter()
            .map(|(_, s)| match s.into_typed() {
                TypedSexp::String(s) => Ok(b64_wrap_(s, newline)),
                _ => Err(savvy_err!("Unsupported type")),
            })
            .collect::<savvy::Result<Vec<String>>>()?
            .try_into(),
        TypedSexp::String(s) => b64_wrap_(s, newline).try_into(),
        _ => Err(savvy_err!("Unsupported type")),
    }
}

fn b64_wrap_(chunks: StringSexp, newline: &str) -> String {
    chunks.iter().join(newline)
}

#[savvy]
fn decode_(input: Sexp, engine: &GeneralPurpose) -> savvy::Result<Sexp> {
    let eng = &engine.0;
    let res = match input.into_typed() {
        TypedSexp::String(s) if s.len() == 1 => eng.decode(s.to_vec()[0])?,
        TypedSexp::Raw(r) => eng.decode(r.as_slice())?,
        _ => return Err(savvy_err!("Unsupported type")),
    };

    let mut result = OwnedListSexp::new(1, false)?;
    result.set_value(0, OwnedRawSexp::try_from_slice(res)?)?;
    result.set_class(&["blob", "vctrs_list_of", "vctrs_vctr", "list"])?;
    result.into()
}

#[savvy] // Either<Strings, List>
fn decode_vectorized_(what: Sexp, engine: &GeneralPurpose) -> savvy::Result<Sexp> {
    let eng = &engine.0;
    match what.into_typed() {
        TypedSexp::String(s) => {
            let mut list = OwnedListSexp::new(s.len(), false)?;
            for (i, s) in s.iter().enumerate() {
                if s.is_na() {
                    list.set_value(i, NullSexp)?;
                } else {
                    let decoded = eng.decode(s);
                    match decoded {
                        Ok(d) => {
                            let v = OwnedRawSexp::try_from_slice(d)?;
                            list.set_value(i, v)?
                        }
                        Err(_) => list.set_value(i, NullSexp)?,
                    }
                }
            }
            list.set_class(&["blob", "vctrs_list_of", "vctrs_vctr", "list"])?;
            list.into()
        }
        TypedSexp::List(l) => {
            let mut list = OwnedListSexp::new(l.len(), false)?;
            for (i, (_nm, val)) in l.iter().enumerate() {
                match val.into_typed() {
                    TypedSexp::Raw(r) => {
                        let decoded = eng.decode(r.as_slice());
                        match decoded {
                            Ok(d) => {
                                let v = OwnedRawSexp::try_from_slice(d)?;
                                list.set_value(i, v)?
                            }
                            Err(_) => list.set_value(i, NullSexp)?,
                        }
                    }
                    _ => list.set_value(i, NullSexp)?,
                }
            }

            list.set_class(&["blob", "vctrs_list_of", "vctrs_vctr", "list"])?;
            list.into()
        }
        _ => return Err(savvy_err!("Unsupported type")),
    }
}

#[savvy]
fn decode_as_string_(
    what: &str,
    engine: &GeneralPurpose,
    split: Option<&str>,
) -> savvy::Result<Sexp> {
    let eng = &engine.0;
    match split {
        Some(sp) => {
            let res = what
                .split(&sp)
                .map(|split| -> savvy::Result<String> {
                    let bytes = eng.decode(split)?;
                    let string = String::from_utf8(bytes)?;
                    Ok(string)
                })
                .collect::<savvy::Result<Vec<String>>>()?;
            res.try_into()
        }
        None => {
            let res = eng.decode(what)?;
            let res = String::from_utf8(res)?;
            res.try_into()
        }
    }
}

#[savvy]
fn decode_file_(path: &str, engine: &GeneralPurpose) -> savvy::Result<Sexp> {
    let eng = &engine.0;
    let file = std::fs::File::open(path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let mut decoder = DecoderReader::new(&mut reader, eng);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result).unwrap();
    result.try_into()
}

#[savvy]
struct Alphabet(alphabet::Alphabet);

// use a built-in alphabet
#[savvy]
fn alphabet_(which: &str) -> savvy::Result<Alphabet> {
    match which {
        "bcrypt" => Ok(Alphabet(alphabet::BCRYPT)),
        "bin_hex" => Ok(Alphabet(alphabet::BIN_HEX)),
        "crypt" => Ok(Alphabet(alphabet::CRYPT)),
        "imap_mutf7" => Ok(Alphabet(alphabet::IMAP_MUTF7)),
        "standard" => Ok(Alphabet(alphabet::STANDARD)),
        "url_safe" => Ok(Alphabet(alphabet::URL_SAFE)),
        _ => Err(savvy_err!("Unknown alphabet: {}", which)),
    }
}

// Create new alphabet
#[savvy]
fn new_alphabet_(chars: &str) -> savvy::Result<Alphabet> {
    let res = alphabet::Alphabet::new(chars)?;
    Ok(Alphabet(res))
}

// get alphabet as a string for printing
#[savvy]
fn get_alphabet_(alphabet: &Alphabet) -> savvy::Result<Sexp> {
    let alph = &alphabet.0;
    alph.as_str().try_into()
}

#[savvy]
struct GeneralPurposeConfig(base64::engine::GeneralPurposeConfig);

// default configs
// padding = true,
// decode_allow_trailing_bits = false,
// and decode_padding_mode = DecodePaddingMode::RequireCanonicalPadding
#[savvy]
fn new_config_(
    encode_padding: bool,
    decode_padding_trailing_bits: bool,
    decode_padding_mode: &str,
) -> savvy::Result<GeneralPurposeConfig> {
    let pad_mode = match decode_padding_mode {
        "indifferent" => DecodePaddingMode::Indifferent,
        "canonical" => DecodePaddingMode::RequireCanonical,
        "none" => DecodePaddingMode::RequireNone,
        _ => return Err(savvy_err!("Unknown padding mode: {}", decode_padding_mode)),
    };

    let config = base64::engine::GeneralPurposeConfig::new()
        .with_encode_padding(encode_padding)
        .with_decode_allow_trailing_bits(decode_padding_trailing_bits)
        .with_decode_padding_mode(pad_mode);

    Ok(GeneralPurposeConfig(config))
}

#[savvy]
fn print_config_(config: &GeneralPurposeConfig) -> savvy::Result<Sexp> {
    let conf = &config.0;
    format!("{:#?}", conf).try_into()
}

#[savvy]
fn engine_(which: &str) -> savvy::Result<GeneralPurpose> {
    match which {
        "standard" => Ok(GeneralPurpose(general_purpose::STANDARD)),
        "standard_no_pad" => Ok(GeneralPurpose(general_purpose::STANDARD_NO_PAD)),
        "url_safe" => Ok(GeneralPurpose(general_purpose::URL_SAFE)),
        "url_safe_no_pad" => Ok(GeneralPurpose(general_purpose::URL_SAFE_NO_PAD)),
        _ => return Err(savvy_err!("Unknown engine: {}", which)),
    }
}

// need to figure out a nice print pattern here
#[savvy]
fn print_engine_(engine: &GeneralPurpose) -> savvy::Result<Sexp> {
    let eng = &engine.0;
    format!("{:#?}", eng).try_into()
}

#[savvy]
fn new_engine_(
    alphabet: &Alphabet,
    config: &GeneralPurposeConfig,
) -> savvy::Result<GeneralPurpose> {
    let alph = &alphabet.0;
    let conf = config.0;
    let engine = general_purpose::GeneralPurpose::new(alph, conf);
    Ok(GeneralPurpose(engine))
}
