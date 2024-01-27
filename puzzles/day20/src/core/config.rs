use anyhow::Context;

use std::collections::{HashMap, HashSet, VecDeque};

use super::{ModuleKind, PulseKind};

type Kinds<'a> = HashMap<&'a str, ModuleKind>;
type Connections<'a> = HashMap<&'a str, Vec<&'a str>>;

pub type PulseHistory<'a> = Vec<(PulseKind, &'a str)>;

#[derive(Debug, Clone)]
pub struct Configuration<'a> {
    pub kinds: Kinds<'a>,
    pub inputs: Connections<'a>,
    pub outputs: Connections<'a>,
}

impl<'a> Configuration<'a> {
    pub fn slice(&'a self, start: &'a str, end: &'a str) -> Self {
        let mut queue = vec![start];

        let mut kinds: Kinds = Default::default();
        let mut inputs: Connections = Default::default();
        let mut outputs: Connections = Default::default();

        let mut seen: HashSet<&str> = Default::default();

        while let Some(name) = queue.pop() {
            if !seen.insert(name) {
                continue;
            }

            if let Some(kind) = self.kinds.get(name) {
                kinds.insert(name, *kind);
            }

            if name == end {
                continue;
            }

            for output in self.outputs[name].iter() {
                inputs.entry(output).or_default().push(name);
                outputs.entry(name).or_default().push(output);
                queue.push(output);
            }
        }

        Self {
            kinds,
            inputs,
            outputs,
        }
    }

    pub fn iter_button_pushes(
        &'a self,
        start: &'a str,
    ) -> impl Iterator<Item = PulseHistory<'a>> + 'a {
        let mut flipflop_state: HashMap<_, _> = self
            .kinds
            .iter()
            .filter_map(|(&name, &kind)| (kind == ModuleKind::FlipFlop).then_some((name, false)))
            .collect();

        let mut conjunction_state: HashMap<_, _> = self
            .kinds
            .iter()
            .filter(|(_, &kind)| kind == ModuleKind::Conjunction)
            .map(|(&name, _)| {
                (
                    name,
                    self.inputs[name]
                        .iter()
                        .map(|&input| (input, PulseKind::Low))
                        .collect::<HashMap<_, _>>(),
                )
            })
            .collect();

        std::iter::from_fn(move || {
            let mut queue: VecDeque<_> = [(PulseKind::Low, "button", start)].into_iter().collect();

            let mut pulse_history: PulseHistory = Default::default();

            while let Some((pulse, from, to)) = queue.pop_front() {
                pulse_history.push((pulse, to));

                if let Some(kind) = self.kinds.get(to) {
                    let pulse = match kind {
                        ModuleKind::Broadcast => pulse,
                        ModuleKind::FlipFlop => {
                            let state = flipflop_state.get_mut(to).unwrap();

                            if pulse == PulseKind::High {
                                continue;
                            }

                            *state = !*state;

                            if *state {
                                PulseKind::High
                            } else {
                                PulseKind::Low
                            }
                        }
                        ModuleKind::Conjunction => {
                            let recent = conjunction_state.get_mut(to).unwrap();

                            recent.insert(from, pulse);

                            if recent.values().any(|&pulse| pulse == PulseKind::Low) {
                                PulseKind::High
                            } else {
                                PulseKind::Low
                            }
                        }
                    };

                    if let Some(outputs) = self.outputs.get(to) {
                        for output in outputs {
                            queue.push_back((pulse, to, output));
                        }
                    }
                }
            }

            Some(pulse_history)
        })
    }
}

impl<'a> std::convert::TryFrom<&'a str> for Configuration<'a> {
    type Error = anyhow::Error;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        fn parse_mapping(input: &str) -> anyhow::Result<(&str, (ModuleKind, Vec<&str>))> {
            let (left, right) = input
                .split_once("->")
                .context("expected module/outputs to be delimited by an arrow")?;

            let left = left.trim();

            let mut chars = left.char_indices();
            let first = chars.next().context("missing module")?.1;

            let kind = match first {
                '%' => ModuleKind::FlipFlop,
                '&' => ModuleKind::Conjunction,
                _ => ModuleKind::Broadcast,
            };

            let name = if kind == ModuleKind::Broadcast {
                left
            } else {
                let i = chars.next().context("missing module name")?.0;
                left.split_at(i).1
            };

            let outputs = right.split(',').map(|s| s.trim()).collect();

            Ok((name, (kind, outputs)))
        }

        let mut kinds: Kinds = Default::default();
        let mut inputs: Connections = Default::default();
        let mut outputs: Connections = Default::default();

        for (i, line) in input.lines().enumerate() {
            let (name, (kind, connections)) =
                parse_mapping(line).with_context(|| format!("mapping number {}", i + 1))?;

            kinds.insert(name, kind);

            for output in &connections {
                inputs.entry(output).or_default().push(name);
            }

            outputs.insert(name, connections);
        }

        Ok(Self {
            kinds,
            inputs,
            outputs,
        })
    }
}
