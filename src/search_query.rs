pub struct GithubSearchQuery {
    pub term: String,
    pub language: Option<String>,
    pub min_stars: Option<String>,
    pub topic: Option<String>,
}

impl GithubSearchQuery {
    // Initialize a new search query with a search term
    pub fn new(term: &str) -> Self {
        Self {
            term: term.to_owned(),
            language: None,
            min_stars: None,
            topic: None,
        }
    }

    // Add a language filter to the search query
    pub fn language(mut self, lang: &str) -> Self {
        self.language = Some(lang.to_owned());
        self
    }

    // Add a min_stars filter to the search query
    pub fn min_stars(mut self, stars: &str) -> Self {
        self.min_stars = Some(stars.to_owned());
        self
    }

    // Add a topic filter to the search query
    pub fn topic(mut self, topic: &str) -> Self {
        self.topic = Some(topic.to_owned());
        self
    }

    // Convert the query to a GitHub-compatible query string
    pub fn to_query_string(&self) -> String {
        let mut query = self.term.clone();
        if let Some(language) = &self.language {
            query.push_str(&format!(" language:{}", language));
        }
        if let Some(stars) = &self.min_stars {
            query.push_str(&format!(" stars:>={}", stars));
        }
        if let Some(topic) = &self.topic {
            query.push_str(&format!(" (topic:{})", topic));
        }
        query
    }
}