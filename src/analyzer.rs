use hound::{WavReader, WavSamples};
use std::{fs::File, io::BufReader, path::Path};

struct WavSignalIterator<'a> {
    reader: WavReader<BufReader<File>>,
    chunk_size: usize,
    samples_iter: Option<WavSamples<'a, BufReader<File>, f32>>,
}

impl<'a> WavSignalIterator<'a> {
    pub fn new(file_path: &Path, chunk_size: usize) -> anyhow::Result<Self> {
        let mut reader = hound::WavReader::open(file_path)?;
        Ok(Self {
            reader,
            chunk_size,
            samples_iter: None,
        })
    }
    pub fn sample_rate(&self) -> u32 {
        self.reader.spec().sample_rate
    }
}

impl<'a> Iterator for WavSignalIterator<'a> {
    type Item = Box<dyn Iterator<Item = f64> + 'a>;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
