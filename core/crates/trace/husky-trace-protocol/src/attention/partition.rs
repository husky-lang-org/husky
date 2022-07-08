use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct PartitionDefnData {
    pub name: String,
    pub ncol: u32,
    pub variant: PartitionDefnDataVariant,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum PartitionDefnDataVariant {
    Label(Label),
    LabelSet(Vec<Label>),
    Other,
}

impl PartitionDefnData {
    pub fn contains(&self, query: Label) -> bool {
        match self.variant {
            PartitionDefnDataVariant::Label(label) => label == query,
            PartitionDefnDataVariant::LabelSet(ref labels) => labels.contains(&query),
            PartitionDefnDataVariant::Other => true,
        }
    }
}

pub struct PartitionedSamplesCollector<T> {
    // suppose there are three partitions (including Other)
    // then partition_filled is 0...0111
    // the first partition is filled iff the last digit of partition_filled is 0
    // the second partition is filled iff the last second digit of partition_filled is 0
    flags: u32,
    partitioned_samples: Vec<(PartitionDefnData, Vec<(SampleId, T)>)>,
    col_len: u32,
}

impl<T> PartitionedSamplesCollector<T> {
    pub fn new(partition_defns: Vec<PartitionDefnData>) -> Self {
        let flags: u32 = (!0u32 << partition_defns.len()) ^ (!0u32);
        let partitioned_samples: Vec<(PartitionDefnData, Vec<(SampleId, T)>)> = partition_defns
            .iter()
            .map(|partition| (partition.clone(), vec![]))
            .collect();
        Self {
            flags,
            partitioned_samples,
            col_len: 5,
        }
    }

    // returns a flag indicating whether the partitions are all full
    pub fn process<E>(
        &mut self,
        label: Label,
        f: impl FnOnce() -> Result<(SampleId, T), E>,
    ) -> Result<bool, E> {
        for (i, (partition, samples)) in self.partitioned_samples.iter_mut().enumerate() {
            let max_samples_len = (partition.ncol * self.col_len) as usize;
            if samples.len() < max_samples_len {
                if partition.contains(label) {
                    samples.push(f()?);
                    if samples.len() == max_samples_len {
                        self.flags &= !(1 << i);
                    }
                }
                break;
            } else {
                assert!((self.flags & (1 << i)) == 0)
            }
        }
        Ok(self.flags == 0)
    }

    pub fn finish(self) -> Vec<(PartitionDefnData, Vec<(SampleId, T)>)> {
        self.partitioned_samples
    }
}
