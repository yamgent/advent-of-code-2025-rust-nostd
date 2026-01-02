#![no_std]
#![feature(maybe_uninit_array_assume_init)]

use core::{fmt, mem::MaybeUninit, str::FromStr};

const ACTUAL_INPUT: &str = include_str!("../../../../actual_inputs/2025/01/input.txt");

#[derive(Debug)]
struct ArrayCapMaxed;

#[must_use = "this `ArrayVecResult` may be an `Err` variant, which should be handled"]
enum ArrayVecResult<T> {
    Ok(T),
    Err(ArrayCapMaxed),
}

impl<T> ArrayVecResult<T> {
    fn expect(self, msg: &str) -> T {
        match self {
            ArrayVecResult::Ok(item) => item,
            ArrayVecResult::Err(array_cap_maxed) => panic!("{}: {:?}", msg, array_cap_maxed),
        }
    }
}

struct ArrayVec<T, const N: usize> {
    buffer: [T; N],
    used: usize,
}

impl<T, const N: usize> AsRef<[T]> for ArrayVec<T, N> {
    fn as_ref(&self) -> &[T] {
        &self.buffer[..self.used]
    }
}

impl<T: fmt::Debug, const N: usize> fmt::Debug for ArrayVec<T, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ArrayVec")
            .field("buffer", &&self.buffer[..self.used])
            .field("used", &self.used)
            .finish()
    }
}

impl<T: PartialEq, const N: usize> PartialEq for ArrayVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.used == other.used && self.buffer[..self.used] == other.buffer[..other.used]
    }
}

impl<T, const N: usize> ArrayVec<T, N> {
    fn new() -> Self {
        let buffer = [const { MaybeUninit::uninit() }; N];
        // SAFETY: ArrayVec API will ensure that uninitialized elements are never accessed.
        let buffer = unsafe { MaybeUninit::array_assume_init(buffer) };

        Self { buffer, used: 0 }
    }

    fn push(&mut self, value: T) -> ArrayVecResult<()> {
        if self.used >= self.buffer.len() {
            ArrayVecResult::Err(ArrayCapMaxed)
        } else {
            self.buffer[self.used] = value;
            self.used += 1;
            ArrayVecResult::Ok(())
        }
    }
}

// TODO: Unlike Rust's RawVec, we haven't found a way to do T::IS_ZST, so we cannot
// use ?Sized yet. Work on removing this constraint later
impl<T, const N: usize> FromIterator<T> for ArrayVecResult<ArrayVec<T, N>> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut buffer = [const { MaybeUninit::uninit() }; N];
        let mut used = 0;

        for item in iter.into_iter() {
            if used >= buffer.len() {
                return Self::Err(ArrayCapMaxed);
            }

            buffer[used].write(item);
            used += 1;
        }

        Self::Ok(ArrayVec::<_, _> {
            // SAFETY: 0..used are initialized. used..buf.len() are not, but
            // the ArrayVec API will ensure that they are initialized if the
            // vec grows and they start getting used.
            buffer: unsafe { MaybeUninit::array_assume_init(buffer) },
            used,
        })
    }
}

struct ArrayVecIntoIter<T, const N: usize> {
    buffer: [Option<T>; N],
    used: usize,
    current: usize,
}

impl<T, const N: usize> Iterator for ArrayVecIntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.used {
            None
        } else {
            let next = self.buffer[self.current]
                .take()
                .expect("each element is only visited once");
            self.current += 1;
            Some(next)
        }
    }
}

impl<T, const N: usize> IntoIterator for ArrayVec<T, N> {
    type Item = T;

    type IntoIter = ArrayVecIntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        let buffer = self
            .buffer
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| *idx < self.used)
            .fold(
                [const { MaybeUninit::uninit() }; N],
                |mut buffer, (idx, item)| {
                    buffer[idx].write(Some(item));
                    buffer
                },
            );

        Self::IntoIter {
            // SAFETY: 0..used is initialized, and ArrayVecIntoIter will never access
            // elements outside of 0..used
            buffer: unsafe { MaybeUninit::array_assume_init(buffer) },
            used: self.used,
            current: 0,
        }
    }
}

struct ArrayString<const N: usize> {
    buffer: ArrayVec<u8, N>,
}

impl<const N: usize> FromIterator<char> for ArrayVecResult<ArrayString<N>> {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut buffer = [0; N];
        let mut used = 0;

        for ch in iter.into_iter() {
            let mut ch_buffer = [0; 4];
            for byte in ch.encode_utf8(&mut ch_buffer).as_bytes().iter() {
                if used >= buffer.len() {
                    return Self::Err(ArrayCapMaxed);
                }
                buffer[used] = *byte;
                used += 1;
            }
        }

        Self::Ok(ArrayString::<_> {
            buffer: ArrayVec { buffer, used },
        })
    }
}

impl<const N: usize> ArrayString<N> {
    fn parse<F: FromStr>(&self) -> Result<F, F::Err> {
        // SAFETY: ArrayString is maintained to be utf8 compatible
        let str_ref = unsafe { str::from_utf8_unchecked(AsRef::<[u8]>::as_ref(&self.buffer)) };
        str_ref.parse()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Instruction {
    Left(u32),
    Right(u32),
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let direction = line.chars().next().expect("a character");
        let amount = line
            .chars()
            .skip(1)
            .collect::<ArrayVecResult<ArrayString<20>>>()
            .expect("enough capacity")
            .parse()
            .expect("a number");

        match direction {
            'L' => Self::Left(amount),
            'R' => Self::Right(amount),
            _ => {
                panic!("expect L or R");
            }
        }
    }
}

fn parse_input<const N: usize>(input: &str) -> ArrayVec<Instruction, N> {
    input
        .trim()
        .lines()
        .map(Instruction::parse)
        .collect::<ArrayVecResult<_>>()
        .expect("enough capacity")
}

const START_NUMBER: i32 = 50;
const TOTAL_NUMBERS: i32 = 100;

fn p1(input: &str) -> u32 {
    parse_input::<5000>(input)
        .into_iter()
        .fold((START_NUMBER, 0u32), |mut acc, line| {
            let amount = match line {
                Instruction::Left(amount) => -(amount as i32),
                Instruction::Right(amount) => amount as i32,
            };

            acc.0 = (acc.0 + amount).rem_euclid(TOTAL_NUMBERS);

            if acc.0 == 0 {
                acc.1 += 1;
            }

            acc
        })
        .1
}

fn p2(input: &str) -> u32 {
    parse_input::<5000>(input)
        .into_iter()
        .fold((START_NUMBER, 0u32), |mut acc, line| {
            match line {
                Instruction::Left(amount) => {
                    let amount = amount as i32;
                    if acc.0 == amount {
                        acc.1 += 1;
                    } else if acc.0 < amount {
                        let remaining = amount - acc.0;
                        acc.1 +=
                            if acc.0 == 0 { 0 } else { 1 } + (remaining / TOTAL_NUMBERS) as u32;
                    }
                    acc.0 = (acc.0 - amount).rem_euclid(TOTAL_NUMBERS);
                }
                Instruction::Right(amount) => {
                    let amount = amount as i32;
                    acc.1 += ((acc.0 + amount) / TOTAL_NUMBERS) as u32;
                    acc.0 = (acc.0 + amount).rem_euclid(TOTAL_NUMBERS);
                }
            }

            acc
        })
        .1
}

fn main() {
    // TODO: Restore this once we have ability to output to console
    // println!("{}", p1(ACTUAL_INPUT));
    // println!("{}", p2(ACTUAL_INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instructions() {
        let mut expected = ArrayVec::<Instruction, 2>::new();
        expected
            .push(Instruction::Left(20))
            .expect("enough capacity");
        expected
            .push(Instruction::Right(30))
            .expect("enough capacity");

        assert_eq!(
            parse_input::<2>(
                r"
L20
R30
"
            ),
            // TODO: Kinda sad that we cannot use vec![], we should implement this one day
            // vec![Instruction::Left(20), Instruction::Right(30)]
            expected
        );
    }

    const SAMPLE_INPUT: &str = r"
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

    #[test]
    fn test_p1_sample() {
        assert_eq!(p1(SAMPLE_INPUT), 3);
    }

    #[test]
    fn test_p1_actual() {
        assert_eq!(p1(ACTUAL_INPUT), 1066);
    }

    #[test]
    fn test_p2_sample() {
        assert_eq!(p2(SAMPLE_INPUT), 6);
    }

    #[test]
    fn test_p2_additional() {
        [
            ("L50", 1),
            ("L50\nL99", 1),
            ("L50\nL100", 2),
            ("L50\nL101", 2),
            ("L50\nL199", 2),
            ("L50\nL200", 3),
            ("L50\nL201", 3),
            ("L50\nR99", 1),
            ("L50\nR100", 2),
            ("L50\nR101", 2),
            ("L50\nR199", 2),
            ("L50\nR200", 3),
            ("L50\nR201", 3),
        ]
        .into_iter()
        .for_each(|(input, expected)| {
            assert_eq!(
                p2(input),
                expected,
                "{}, {}",
                input.replace("\n", ";"),
                expected
            );
        });
    }

    #[test]
    fn test_p2_actual() {
        assert_eq!(p2(ACTUAL_INPUT), 6223);
    }
}
