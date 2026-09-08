use bip39::{Language, Mnemonic};
use sha2::{Digest, Sha256};
use std::process::ExitCode;

slint::include_modules!();

const ENTROPY_BYTES: usize = 32;
const MIN_MOVEMENT_SAMPLES: usize = 1024;
const MAX_MOVEMENT_SAMPLES: usize = 1024;
const MOVEMENT_DOMAIN: &[u8] = b"OpenSeed Stage 4 movement entropy v1";
const DICE_SIDES: u8 = 6;
const DICE_ROLLS_PER_BLOCK: usize = 8;
const DICE_BITS_PER_BLOCK: usize = 20;
const DICE_TARGET_BITS: usize = ENTROPY_BYTES * 8;
const DICE_TARGET_BLOCKS: usize = DICE_TARGET_BITS.div_ceil(DICE_BITS_PER_BLOCK);
const DICE_DOMAIN: &[u8] = b"OpenSeed Stage 5 dice entropy v1";
const VERIFICATION_WORD_COUNT: usize = 24;

fn local_network_status() -> Result<bool, String> {
    #[cfg(unix)]
    {
        let mut addresses = std::ptr::null_mut();
        let result = unsafe { libc::getifaddrs(&mut addresses) };
        if result != 0 {
            return Err(std::io::Error::last_os_error().to_string());
        }

        let mut online = false;
        let mut current = addresses;
        while !current.is_null() {
            let interface = unsafe { &*current };
            if !interface.ifa_addr.is_null() {
                let flags = interface.ifa_flags as libc::c_uint;
                let is_up = flags & libc::IFF_UP as libc::c_uint != 0;
                let is_loopback = flags & libc::IFF_LOOPBACK as libc::c_uint != 0;
                let family = unsafe { (*interface.ifa_addr).sa_family as libc::c_int };
                let has_ip_address = family == libc::AF_INET || family == libc::AF_INET6;

                if is_up && !is_loopback && has_ip_address {
                    online = true;
                    break;
                }
            }
            current = interface.ifa_next;
        }

        unsafe { libc::freeifaddrs(addresses) };
        Ok(online)
    }

    #[cfg(not(unix))]
    {
        Err("local network inspection is unavailable on this platform".to_owned())
    }
}

fn network_status_text() -> (String, bool) {
    match local_network_status() {
        Ok(true) => ("ONLINE — disconnect before generating".to_owned(), true),
        Ok(false) => (
            "OFFLINE — no active non-loopback interface found".to_owned(),
            false,
        ),
        Err(error) => (format!("UNKNOWN — check manually ({error})"), false),
    }
}

fn verify_mnemonic_words(words: &[String]) -> Result<(), String> {
    if words.len() != VERIFICATION_WORD_COUNT {
        return Err("Enter exactly 24 words.".to_owned());
    }

    let normalized = words
        .iter()
        .enumerate()
        .map(|(index, word)| {
            let normalized = word.trim().to_ascii_lowercase();
            if normalized.is_empty() {
                return Err(format!("Word {} is empty.", index + 1));
            }
            if normalized.split_whitespace().count() != 1 {
                return Err(format!("Word {} must contain exactly one word.", index + 1));
            }
            if !Language::English
                .word_list()
                .iter()
                .any(|candidate| *candidate == normalized)
            {
                return Err(format!(
                    "Word {} is not in the English BIP-39 list.",
                    index + 1
                ));
            }
            Ok(normalized)
        })
        .collect::<Result<Vec<_>, _>>()?;

    let sentence = normalized.join(" ");
    Mnemonic::parse_in_normalized(Language::English, &sentence)
        .map(|_| ())
        .map_err(|_| "The mnemonic checksum is invalid.".to_owned())
}

fn generate_mnemonic() -> Result<Mnemonic, getrandom::Error> {
    let mut entropy = [0_u8; ENTROPY_BYTES];
    getrandom::fill(&mut entropy)?;
    Ok(mnemonic_from_entropy(entropy))
}

fn mnemonic_from_entropy(entropy: [u8; ENTROPY_BYTES]) -> Mnemonic {
    Mnemonic::from_entropy(&entropy).expect("32 bytes is valid BIP-39 entropy")
}

fn movement_status(sample_count: usize) -> String {
    format!("Distinct movement samples: {sample_count} (minimum {MIN_MOVEMENT_SAMPLES})")
}

fn dice_status(roll_count: usize, accepted_blocks: usize) -> String {
    format!(
        "Rolls recorded: {roll_count}   Accepted blocks: {accepted_blocks} / {DICE_TARGET_BLOCKS}"
    )
}

fn entropy_from_sources(
    use_system_rng: bool,
    movement_bytes: &[u8],
    dice_entropy: Option<&[u8; ENTROPY_BYTES]>,
) -> Result<[u8; ENTROPY_BYTES], getrandom::Error> {
    if !use_system_rng && movement_bytes.is_empty() && dice_entropy.is_none() {
        return Err(getrandom::Error::UNSUPPORTED);
    }

    if use_system_rng && movement_bytes.is_empty() && dice_entropy.is_none() {
        let mut entropy = [0_u8; ENTROPY_BYTES];
        getrandom::fill(&mut entropy)?;
        return Ok(entropy);
    }

    let mut input = Vec::with_capacity(
        MOVEMENT_DOMAIN.len() + DICE_DOMAIN.len() + ENTROPY_BYTES + movement_bytes.len(),
    );
    input.extend_from_slice(MOVEMENT_DOMAIN);

    if use_system_rng {
        let mut system_entropy = [0_u8; ENTROPY_BYTES];
        getrandom::fill(&mut system_entropy)?;
        input.push(1);
        input.extend_from_slice(&system_entropy);
    }

    if !movement_bytes.is_empty() {
        input.push(2);
        input.extend_from_slice(movement_bytes);
    }

    if let Some(dice_entropy) = dice_entropy {
        input.extend_from_slice(DICE_DOMAIN);
        input.push(3);
        input.extend_from_slice(dice_entropy);
    }

    Ok(Sha256::digest(input).into())
}

fn print_usage() {
    println!("OpenSeed 0.1.0");
    println!("Launches the graphical interface by default.");
    println!();
    println!("Usage: openseed [--cli|--help]");
    println!();
    println!("Use --cli to generate one mnemonic in the terminal.");
}

fn run_cli() -> ExitCode {
    match generate_mnemonic() {
        Ok(mnemonic) => {
            println!("{mnemonic}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Unable to obtain secure operating-system randomness: {error}");
            ExitCode::from(1)
        }
    }
}

#[derive(Default)]
struct MovementCollector {
    bytes: Vec<u8>,
    last_position: Option<(i32, i32)>,
}

#[derive(Default)]
struct DiceCollector {
    rolls: Vec<u8>,
}

#[derive(Default)]
struct VerificationState {
    words: Vec<String>,
}

impl VerificationState {
    fn new() -> Self {
        Self {
            words: vec![String::new(); VERIFICATION_WORD_COUNT],
        }
    }

    fn clear(&mut self) {
        self.words.fill(String::new());
    }
}

fn verification_model(words: &[String]) -> slint::ModelRc<slint::SharedString> {
    slint::ModelRc::new(slint::VecModel::from_iter(
        words.iter().cloned().map(slint::SharedString::from),
    ))
}

impl DiceCollector {
    fn clear(&mut self) {
        self.rolls.clear();
    }

    fn record(&mut self, roll: u8) {
        if (1..=DICE_SIDES).contains(&roll) {
            self.rolls.push(roll);
        }
    }

    fn accepted_blocks(&self) -> usize {
        dice_accepted_blocks(&self.rolls)
    }

    fn entropy(&self) -> Option<[u8; ENTROPY_BYTES]> {
        dice_entropy_from_rolls(&self.rolls)
    }
}

fn dice_block_value(block: &[u8]) -> Option<u32> {
    if block.len() != DICE_ROLLS_PER_BLOCK
        || block.iter().any(|roll| *roll == 0 || *roll > DICE_SIDES)
    {
        return None;
    }

    Some(block.iter().fold(0_u32, |value, roll| {
        value * u32::from(DICE_SIDES) + u32::from(*roll - 1)
    }))
}

fn dice_accepted_blocks(rolls: &[u8]) -> usize {
    rolls
        .chunks_exact(DICE_ROLLS_PER_BLOCK)
        .filter_map(dice_block_value)
        .filter(|value| *value < (1 << DICE_BITS_PER_BLOCK))
        .count()
}

fn dice_entropy_from_rolls(rolls: &[u8]) -> Option<[u8; ENTROPY_BYTES]> {
    let mut entropy = [0_u8; ENTROPY_BYTES];
    let mut output_bits = 0;

    for value in rolls
        .chunks_exact(DICE_ROLLS_PER_BLOCK)
        .filter_map(dice_block_value)
        .filter(|value| *value < (1 << DICE_BITS_PER_BLOCK))
    {
        for bit_index in (0..DICE_BITS_PER_BLOCK).rev() {
            if output_bits == DICE_TARGET_BITS {
                return Some(entropy);
            }
            let bit = ((value >> bit_index) & 1) as u8;
            entropy[output_bits / 8] |= bit << (7 - (output_bits % 8));
            output_bits += 1;
        }
    }

    (output_bits == DICE_TARGET_BITS).then_some(entropy)
}

impl MovementCollector {
    fn clear(&mut self) {
        self.bytes.clear();
        self.last_position = None;
    }

    fn record(&mut self, x: f32, y: f32) {
        if self.bytes.len() / (2 * std::mem::size_of::<i32>()) >= MAX_MOVEMENT_SAMPLES {
            return;
        }
        if !x.is_finite() || !y.is_finite() {
            return;
        }

        let position = (x.round() as i32, y.round() as i32);
        if self.last_position == Some(position) {
            return;
        }
        self.last_position = Some(position);
        self.bytes.extend_from_slice(&position.0.to_le_bytes());
        self.bytes.extend_from_slice(&position.1.to_le_bytes());
    }

    fn sample_count(&self) -> usize {
        self.bytes.len() / (2 * std::mem::size_of::<i32>())
    }
}

fn run_gui() -> Result<(), slint::PlatformError> {
    let ui = OpenSeedWindow::new()?;
    ui.set_movement_status(movement_status(0).into());
    ui.set_dice_status(dice_status(0, 0).into());
    let (network_status, network_online) = network_status_text();
    ui.set_network_status(network_status.into());
    ui.set_network_online(network_online);
    let movement = std::rc::Rc::new(std::cell::RefCell::new(MovementCollector::default()));
    let dice = std::rc::Rc::new(std::cell::RefCell::new(DiceCollector::default()));
    let verification = std::rc::Rc::new(std::cell::RefCell::new(VerificationState::new()));
    ui.set_verification_words(verification_model(&verification.borrow().words));

    let movement_for_events = movement.clone();
    let count_ui = ui.as_weak();
    ui.on_pointer_moved(move |x, y| {
        let mut movement = movement_for_events.borrow_mut();
        movement.record(x, y);
        if let Some(ui) = count_ui.upgrade() {
            ui.set_movement_sample_count(movement.sample_count() as i32);
            ui.set_movement_status(movement_status(movement.sample_count()).into());
        }
    });

    let movement_for_reset = movement.clone();
    let reset_ui = ui.as_weak();
    ui.on_reset_movement_requested(move || {
        movement_for_reset.borrow_mut().clear();
        if let Some(ui) = reset_ui.upgrade() {
            ui.set_movement_sample_count(0);
            ui.set_movement_status(movement_status(0).into());
        }
    });

    let dice_for_events = dice.clone();
    let dice_ui = ui.as_weak();
    ui.on_dice_roll(move |roll| {
        let mut dice = dice_for_events.borrow_mut();
        dice.record(roll as u8);
        if let Some(ui) = dice_ui.upgrade() {
            ui.set_dice_roll_count(dice.rolls.len() as i32);
            ui.set_dice_accepted_blocks(dice.accepted_blocks() as i32);
            ui.set_dice_status(dice_status(dice.rolls.len(), dice.accepted_blocks()).into());
        }
    });

    let dice_for_reset = dice.clone();
    let reset_dice_ui = ui.as_weak();
    ui.on_reset_dice_requested(move || {
        dice_for_reset.borrow_mut().clear();
        if let Some(ui) = reset_dice_ui.upgrade() {
            ui.set_dice_roll_count(0);
            ui.set_dice_accepted_blocks(0);
            ui.set_dice_status(dice_status(0, 0).into());
        }
    });

    let movement_for_start_over = movement.clone();
    let dice_for_start_over = dice.clone();
    let verification_for_start_over = verification.clone();
    let start_over_ui = ui.as_weak();
    ui.on_start_over_requested(move || {
        movement_for_start_over.borrow_mut().clear();
        dice_for_start_over.borrow_mut().clear();
        verification_for_start_over.borrow_mut().clear();
        if let Some(ui) = start_over_ui.upgrade() {
            ui.set_has_mnemonic(false);
            ui.set_verification_mode(false);
            ui.set_verification_valid(false);
            ui.set_verification_status("".into());
            ui.set_verification_words(verification_model(&vec![
                String::new();
                VERIFICATION_WORD_COUNT
            ]));
            ui.set_mnemonic_words(slint::ModelRc::new(slint::VecModel::from_iter(
                std::iter::empty::<slint::SharedString>(),
            )));
            ui.set_error_text("".into());
            ui.set_movement_sample_count(0);
            ui.set_movement_status(movement_status(0).into());
            ui.set_dice_roll_count(0);
            ui.set_dice_accepted_blocks(0);
            ui.set_dice_status(dice_status(0, 0).into());
            ui.set_show_donation(false);
            ui.set_show_about(false);
            ui.set_show_passphrase_info(false);
        }
    });

    let verification_for_open = verification.clone();
    let open_verification_ui = ui.as_weak();
    ui.on_open_verification_requested(move || {
        verification_for_open.borrow_mut().clear();
        if let Some(ui) = open_verification_ui.upgrade() {
            ui.set_has_mnemonic(false);
            ui.set_verification_mode(true);
            ui.set_verification_valid(false);
            ui.set_verification_status("".into());
            ui.set_verification_words(verification_model(&vec![
                String::new();
                VERIFICATION_WORD_COUNT
            ]));
            ui.set_error_text("".into());
            ui.set_show_donation(false);
            ui.set_show_about(false);
            ui.set_show_passphrase_info(false);
        }
    });

    let verification_for_edit = verification.clone();
    let edit_verification_ui = ui.as_weak();
    ui.on_verification_word_edited(move |index, word| {
        let index = index as usize;
        if index < VERIFICATION_WORD_COUNT {
            verification_for_edit.borrow_mut().words[index] = word.to_string();
        }
        if let Some(ui) = edit_verification_ui.upgrade() {
            ui.set_verification_valid(false);
            ui.set_verification_status("".into());
        }
    });

    let verification_for_check = verification.clone();
    let check_verification_ui = ui.as_weak();
    ui.on_verify_mnemonic_requested(move || {
        let result = verify_mnemonic_words(&verification_for_check.borrow().words);
        if let Some(ui) = check_verification_ui.upgrade() {
            match result {
                Ok(()) => {
                    ui.set_verification_valid(true);
                    ui.set_verification_status(
                        "VALID — 24 English BIP-39 words with a correct checksum.".into(),
                    );
                }
                Err(error) => {
                    ui.set_verification_valid(false);
                    ui.set_verification_status(error.into());
                }
            }
        }
    });

    let verification_for_back = verification.clone();
    let back_verification_ui = ui.as_weak();
    ui.on_back_to_start_requested(move || {
        verification_for_back.borrow_mut().clear();
        if let Some(ui) = back_verification_ui.upgrade() {
            ui.set_verification_mode(false);
            ui.set_verification_valid(false);
            ui.set_verification_status("".into());
            ui.set_verification_words(verification_model(&vec![
                String::new();
                VERIFICATION_WORD_COUNT
            ]));
            ui.set_error_text("".into());
        }
    });

    let refresh_network_ui = ui.as_weak();
    ui.on_refresh_network_status(move || {
        let (network_status, network_online) = network_status_text();
        if let Some(ui) = refresh_network_ui.upgrade() {
            ui.set_network_status(network_status.into());
            ui.set_network_online(network_online);
        }
    });

    let generate_ui = ui.as_weak();
    let movement_for_generate = movement.clone();
    let dice_for_generate = dice.clone();
    ui.on_generate_requested(move || {
        if let Some(ui) = generate_ui.upgrade() {
            let use_system_rng = ui.get_use_system_rng();
            let use_movement = ui.get_use_movement();
            let use_dice = ui.get_use_dice();
            let movement = movement_for_generate.borrow();
            let dice = dice_for_generate.borrow();

            if !use_system_rng && !use_movement && !use_dice {
                ui.set_error_text("Select at least one entropy source.".into());
                return;
            }
            if use_movement && movement.sample_count() < MIN_MOVEMENT_SAMPLES {
                ui.set_error_text(
                    format!(
                        "Move the pointer through the collection area until at least {MIN_MOVEMENT_SAMPLES} distinct samples are collected, or reset the collection."
                    )
                    .into(),
                );
                return;
            }
            if use_dice && dice.accepted_blocks() < DICE_TARGET_BLOCKS {
                ui.set_error_text(
                    format!(
                        "Record at least {DICE_TARGET_BLOCKS} accepted dice blocks; rejected blocks require additional rolls."
                    )
                    .into(),
                );
                return;
            }

            let movement_bytes = if use_movement { &movement.bytes[..] } else { &[] };
            let dice_entropy = if use_dice { dice.entropy() } else { None };
            match entropy_from_sources(use_system_rng, movement_bytes, dice_entropy.as_ref()) {
                Ok(entropy) => {
                    let mnemonic = mnemonic_from_entropy(entropy);
                    let words = mnemonic
                        .words()
                        .enumerate()
                        .map(|(index, word)| {
                            slint::SharedString::from(format!("{:02}. {word}", index + 1))
                        })
                        .collect::<Vec<_>>();
                    ui.set_mnemonic_words(slint::ModelRc::new(slint::VecModel::from_iter(words)));
                    ui.set_has_mnemonic(true);
                }
                Err(error) => {
                    ui.set_error_text(format!("Unable to obtain secure randomness: {error}").into());
                }
            }
        }
    });

    let close_ui = ui.as_weak();
    ui.on_close_requested(move || {
        if let Some(ui) = close_ui.upgrade() {
            ui.hide().ok();
        }
    });

    ui.run()
}

fn main() -> ExitCode {
    match std::env::args().nth(1).as_deref() {
        Some("--help") | Some("-h") => {
            print_usage();
            ExitCode::SUCCESS
        }
        Some("--cli") => run_cli(),
        Some(argument) => {
            eprintln!("Unknown argument: {argument}");
            eprintln!("Try 'openseed --help'.");
            ExitCode::from(2)
        }
        None => match run_gui() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Unable to start graphical interface: {error}");
                ExitCode::from(1)
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bip39::Language;
    use sha2::{Digest, Sha256};

    #[test]
    fn generated_mnemonic_passes_bip39_validation() {
        let mnemonic = generate_mnemonic().expect("OS CSPRNG should be available");
        let words: Vec<_> = mnemonic.words().collect();

        // BIP-39: ENT=256 produces 24 words and CS=ENT/32=8 checksum bits.
        assert_eq!(words.len(), 24, "generated mnemonic must contain 24 words");

        let english = Language::English.word_list();
        for word in &words {
            assert!(
                english.contains(word),
                "generated word is not in the official English BIP-39 list: {word}"
            );
        }

        let entropy = mnemonic.to_entropy();
        assert_eq!(entropy.len(), ENTROPY_BYTES);
        let digest = Sha256::digest(&entropy);
        let expected_checksum = digest[0];
        let last_word_index = english
            .iter()
            .position(|word| *word == words[23])
            .expect("last word must be in the English BIP-39 list");
        let actual_checksum = (last_word_index & 0xff) as u8;
        assert_eq!(
            actual_checksum, expected_checksum,
            "BIP-39 checksum must equal the first 8 bits of SHA-256(entropy)"
        );

        assert!(Mnemonic::parse_in_normalized(Language::English, &mnemonic.to_string()).is_ok());
    }

    #[test]
    fn official_bip39_vector_is_supported() {
        let entropy = [0_u8; ENTROPY_BYTES];
        let mnemonic = Mnemonic::from_entropy(&entropy).unwrap();
        assert_eq!(
            mnemonic.to_string(),
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art"
        );
        assert!(Mnemonic::parse(mnemonic.to_string()).is_ok());
    }

    #[test]
    fn verification_accepts_official_24_word_vector() {
        let words = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art"
            .split_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>();

        assert!(verify_mnemonic_words(&words).is_ok());
    }

    #[test]
    fn verification_rejects_invalid_word_and_checksum() {
        let mut invalid_word = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art"
            .split_whitespace()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        invalid_word[0] = "not-a-bip39-word".to_owned();
        assert!(verify_mnemonic_words(&invalid_word)
            .expect_err("unknown word must be rejected")
            .contains("not in the English BIP-39 list"));

        let mut invalid_checksum = invalid_word;
        invalid_checksum[0] = "abandon".to_owned();
        invalid_checksum[23] = "about".to_owned();
        assert_eq!(
            verify_mnemonic_words(&invalid_checksum),
            Err("The mnemonic checksum is invalid.".to_owned())
        );
    }

    #[test]
    fn verification_rejects_incomplete_input() {
        let words = vec!["abandon".to_owned(); VERIFICATION_WORD_COUNT - 1];
        assert_eq!(
            verify_mnemonic_words(&words),
            Err("Enter exactly 24 words.".to_owned())
        );
    }

    #[test]
    fn verification_state_clear_removes_all_entered_words() {
        let mut state = VerificationState::new();
        state.words[0] = "abandon".to_owned();
        state.words[23] = "art".to_owned();

        state.clear();

        assert_eq!(state.words, vec![String::new(); VERIFICATION_WORD_COUNT]);
    }

    #[test]
    fn movement_collector_deduplicates_and_caps_samples() {
        let mut collector = MovementCollector::default();
        collector.record(10.2, 20.7);
        collector.record(10.4, 20.6);
        assert_eq!(collector.sample_count(), 1);

        for index in 0..(MAX_MOVEMENT_SAMPLES + 10) {
            collector.record(index as f32, 0.0);
        }
        assert_eq!(collector.sample_count(), MAX_MOVEMENT_SAMPLES);
    }

    #[test]
    fn movement_entropy_is_valid_bip39_entropy_and_changes_with_input() {
        let first = [1_u8, 2, 3, 4, 5, 6, 7, 8];
        let second = [1_u8, 2, 3, 4, 5, 6, 7, 9];
        let first_entropy = entropy_from_sources(false, &first, None).unwrap();
        let second_entropy = entropy_from_sources(false, &second, None).unwrap();

        assert_ne!(first_entropy, second_entropy);
        assert!(Mnemonic::from_entropy(&first_entropy).is_ok());
        assert!(Mnemonic::from_entropy(&second_entropy).is_ok());
    }

    #[test]
    fn dice_rejection_sampling_produces_256_bits_without_modulo_bias() {
        let accepted_rolls = vec![1_u8; DICE_ROLLS_PER_BLOCK * DICE_TARGET_BLOCKS];
        let rejected_rolls = vec![DICE_SIDES; DICE_ROLLS_PER_BLOCK];

        assert_eq!(dice_accepted_blocks(&accepted_rolls), DICE_TARGET_BLOCKS);
        assert_eq!(dice_accepted_blocks(&rejected_rolls), 0);

        let entropy = dice_entropy_from_rolls(&accepted_rolls).unwrap();
        assert_eq!(entropy, [0_u8; ENTROPY_BYTES]);
        assert!(Mnemonic::from_entropy(&entropy).is_ok());
    }
}
