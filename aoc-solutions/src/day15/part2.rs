use crate::day15::part1;
use std::collections::HashMap;

pub fn solve(input: &str) -> Result<String, String> {
    let sequence = input.split(',').map(|step| step.trim());

    let mut boxes: HashMap<u32, Vec<(String, u32)>> =
        (0..256).map(|key| (key, Vec::new())).collect();

    for step in sequence {
        dbg!(&step);
        let (label, focal_length): (String, Option<u32>) = match step.split_once('=') {
            Some((label, focal_length)) => (label.to_string(), Some(focal_length.parse().unwrap())),
            None => (
                String::from_iter(step.chars().take_while(|c| *c != '-')),
                None,
            ),
        };
        let box_number = part1::calculate_hash(&label);

        boxes
            .entry(box_number)
            .and_modify(|lenses| match focal_length {
                None => {
                    if let Some((lens_index, _)) = lenses
                        .iter()
                        .enumerate()
                        .find(|(_, (lens_label, _))| *lens_label == label)
                    {
                        lenses.remove(lens_index);
                    }
                }
                Some(focal_length) => {
                    match lenses
                        .iter_mut()
                        .find(|(lens_label, _)| *lens_label == label)
                    {
                        Some((_, old_focal_length)) => *old_focal_length = focal_length,
                        None => lenses.push((label, focal_length)),
                    };
                }
            });
    }

    let focusing_power: u32 = boxes
        .iter()
        .flat_map(|(box_number, lenses)| {
            lenses.iter().enumerate().map(|(slot, (_, focal_length))| {
                (1 + *box_number) * (slot as u32 + 1) * *focal_length
            })
        })
        .sum();

    Ok(focusing_power.to_string())
}
