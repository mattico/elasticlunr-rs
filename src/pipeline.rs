use serde::ser::{Serialize, Serializer, SerializeSeq};

pub fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| c.is_whitespace() || c == '-')
        .filter(|s| s.len() > 0)
        .map(|s| String::from(s.to_ascii_lowercase()))
        .collect()
}

pub type PipelineFn = fn(String) -> Option<String>;

#[derive(Debug)]
pub struct Pipeline {
    pub(crate) queue: Vec<(String, PipelineFn)>,
}

impl Serialize for Pipeline {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.queue.len()))?;
        for &(ref name, _) in &self.queue {
            seq.serialize_element(&name)?;
        }
        seq.end()
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        ::lang::en::make_pipeline()
    }
}

impl Pipeline {
    pub fn run(&self, tokens: Vec<String>) -> Vec<String> {
        let mut ret = vec![];
        for token in tokens {
            let mut token = Some(token);
            for &(_, func) in &self.queue {
                if let Some(t) = token {
                    token = func(t);
                } else {
                    break;
                }
            }
            if let Some(t) = token {
                ret.push(t);
            }
        }
        ret
    }
}
