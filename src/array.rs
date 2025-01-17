use super::{Error, Miniconf, MiniconfMetadata};

use core::fmt::Write;

impl<T: Miniconf, const N: usize> Miniconf for [T; N] {
    fn string_set(
        &mut self,
        mut topic_parts: core::iter::Peekable<core::str::Split<char>>,
        value: &[u8],
    ) -> Result<(), Error> {
        let next = topic_parts.next();
        if next.is_none() {
            return Err(Error::PathTooShort);
        }

        // Parse what should be the index value
        let i: usize = serde_json_core::from_str(next.unwrap())
            .or(Err(Error::BadIndex))?
            .0;

        if i >= self.len() {
            return Err(Error::BadIndex);
        }

        self[i].string_set(topic_parts, value)?;

        Ok(())
    }

    fn string_get(
        &self,
        mut topic_parts: core::iter::Peekable<core::str::Split<char>>,
        value: &mut [u8],
    ) -> Result<usize, Error> {
        let next = topic_parts.next();
        if next.is_none() {
            return Err(Error::PathTooShort);
        }

        // Parse what should be the index value
        let i: usize = serde_json_core::from_str(next.unwrap())
            .or(Err(Error::BadIndex))?
            .0;

        if i >= self.len() {
            return Err(Error::BadIndex);
        }

        self[i].string_get(topic_parts, value)
    }

    fn get_metadata(&self) -> MiniconfMetadata {
        // First, figure out how many digits the maximum index requires when printing.
        let mut index = N - 1;
        let mut num_digits = 0;

        while index > 0 {
            index /= 10;
            num_digits += 1;
        }

        let metadata = self[0].get_metadata();

        // If the sub-members have topic size, we also need to include an additional character for
        // the path separator. This is ommitted if the sub-members have no topic (e.g. fundamental
        // types, enums).
        if metadata.max_topic_size > 0 {
            MiniconfMetadata {
                max_topic_size: metadata.max_topic_size + num_digits + 1,
                max_depth: metadata.max_depth + 1,
            }
        } else {
            MiniconfMetadata {
                max_topic_size: num_digits,
                max_depth: metadata.max_depth + 1,
            }
        }
    }

    fn recurse_paths<const TS: usize>(
        &self,
        index: &mut [usize],
        topic: &mut heapless::String<TS>,
    ) -> Option<()> {
        let original_length = topic.len();

        if index.len() == 0 {
            // Note: During expected execution paths using `into_iter()`, the size of the
            // index stack is checked in advance to make sure this condition doesn't occur.
            // However, it's possible to happen if the user manually calls `recurse_paths`.
            unreachable!("Index stack too small");
        }

        while index[0] < N {
            // Add the array index to the topic name.
            if topic.len() > 0 {
                if topic.push('/').is_err() {
                    // Note: During expected execution paths using `into_iter()`, the size of the
                    // topic buffer is checked in advance to make sure this condition doesn't occur.
                    // However, it's possible to happen if the user manually calls `recurse_paths`.
                    unreachable!("Topic buffer too short");
                }
            }

            if write!(topic, "{}", index[0]).is_err() {
                // Note: During expected execution paths using `into_iter()`, the size of the
                // topic buffer is checked in advance to make sure this condition doesn't occur.
                // However, it's possible to happen if the user manually calls `recurse_paths`.
                unreachable!("Topic buffer too short");
            }

            if self[index[0]]
                .recurse_paths(&mut index[1..], topic)
                .is_some()
            {
                return Some(());
            }

            // Strip off the previously prepended index, since we completed that element and need
            // to instead check the next one.
            topic.truncate(original_length);

            index[0] += 1;
            index[1..].iter_mut().for_each(|x| *x = 0);
        }

        None
    }
}
