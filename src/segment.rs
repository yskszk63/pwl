use crate::Color;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Group {
    GitStatus,
}

#[derive(Debug)]
pub struct Builder {
    color: Color,
    text: String,
    group: Option<Group>,
}

impl Builder {
    pub fn group(&mut self, group: Group) -> &mut Self {
        self.group = Some(group);
        self
    }

    pub fn build(&self) -> Segment {
        Segment {
            color: self.color.clone(),
            text: self.text.clone(),
            group: self.group.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Segment {
    color: Color,
    text: String,
    group: Option<Group>,
}

impl Segment {
    pub fn builder(color: Color, text: impl ToString) -> Builder {
        Builder {
            color,
            text: text.to_string(),
            group: None,
        }
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn group(&self) -> Option<&Group> {
        self.group.as_ref()
    }
}
