use crate::network::TrainingData;
use csv::{Error, Reader};
use std::iter::zip;

pub enum IncludeType {
    Blacklist,
    Whitelist,
}

pub struct CsvDataLoader {
    /// specifies how to include columns (blacklist/whitelist)
    pub include_type: IncludeType,

    /// specifies what columns to include cannot
    /// when loading test data it ignores the label column even if included
    pub include: Vec<usize>,
}

/// for regression specifies the column index
/// for classification specifies the column index and the label strings
pub enum Label<'a> {
    Regression(usize),
    Classification(usize, &'a Vec<&'a str>),
}

impl<'a> CsvDataLoader {
    pub fn new() -> Self {
        Self {
            // empty blacklist = everything allowed
            include_type: IncludeType::Blacklist,
            include: vec![],
        }
    }

    /// data for training and testing
    pub fn load_labeled_data(&self, file_path: &str, label: Label) -> Result<TrainingData, Error> {
        let mut rdr = Reader::from_path(file_path)?;

        let mut inputs: Vec<Vec<f32>> = vec![];
        let mut expected: Vec<Vec<f32>> = vec![];

        for result in rdr.records() {
            let record = result?;

            let (expected_for_sample, label_col_i): (Vec<f32>, usize) = match label {
                Label::Regression(col_i) => (vec![record[col_i].parse().unwrap()], col_i),
                Label::Classification(col_i, labels) => {
                    let mut vals = vec![0.0; labels.len()];
                    let expected_label = &record[col_i];

                    let label_i = labels
                        .iter()
                        .position(|&l| l == expected_label)
                        .expect("label from file not found in vector");

                    vals[label_i] = 1.0;

                    (vals, label_i)
                }
            };
            expected.push(expected_for_sample);

            let col_range: Vec<usize> = (0..record.len())
                .filter(|&i| {
                    i != label_col_i
                        && match self.include_type {
                            IncludeType::Whitelist => self.include.contains(&i),
                            IncludeType::Blacklist => !self.include.contains(&i),
                        }
                })
                .collect();

            let inputs_for_sample: Vec<f32> = col_range
                .iter()
                .map(|&i| {
                    (&record)[i]
                        .parse::<f32>()
                        .expect("value in file is not a number")
                })
                .collect();
            inputs.push(inputs_for_sample);
        }

        Ok(zip(inputs, expected).collect())
    }

    /// data for predicting
    pub fn load_unlabeled_data(&self, file_path: &str) -> Result<Vec<Vec<f32>>, Error> {
        let mut rdr = Reader::from_path(file_path)?;

        let mut inputs: Vec<Vec<f32>> = vec![];

        for result in rdr.records() {
            let record = result?;

            let col_range: Vec<usize> = (0..record.len())
                .filter(|&i| match self.include_type {
                    IncludeType::Whitelist => self.include.contains(&i),
                    IncludeType::Blacklist => !self.include.contains(&i),
                })
                .collect();

            let inputs_for_sample: Vec<f32> = col_range
                .iter()
                .map(|&i| {
                    (&record)[i]
                        .parse::<f32>()
                        .expect("value in file is not a number")
                })
                .collect();
            inputs.push(inputs_for_sample);
        }

        Ok(inputs)
    }
}
