use std::collections::HashSet;
use regex::Regex;

lazy_static::lazy_static! {
    static ref AD_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"ad\d+").unwrap(),
        Regex::new(r"banner|advertisement").unwrap(),
        Regex::new(r"advert|promotional").unwrap(),
    ];
    
    static ref TRACKER_DOMAINS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("google-analytics.com");
        set.insert("facebook.com");
        set.insert("doubleclick.net");
        set.insert("scorecardresearch.com");
        set.insert("hotjar.com");
        set.insert("mixpanel.com");
        set
    };
}

pub struct AdblockEngine {
    pub enabled: bool,
    pub blocked_count: usize,
}

impl AdblockEngine {
    pub fn new() -> Self {
        AdblockEngine {
            enabled: true,
            blocked_count: 0,
        }
    }

    pub fn should_block_url(&mut self, url: &str) -> bool {
        if !self.enabled {
            return false;
        }

        // Check tracker domains
        for tracker in TRACKER_DOMAINS.iter() {
            if url.contains(tracker) {
                self.blocked_count += 1;
                return true;
            }
        }

        // Check ad patterns
        for pattern in AD_PATTERNS.iter() {
            if pattern.is_match(url) {
                self.blocked_count += 1;
                return true;
            }
        }

        false
    }

    pub fn should_block_element(&mut self, class: &str) -> bool {
        if !self.enabled {
            return false;
        }

        for pattern in AD_PATTERNS.iter() {
            if pattern.is_match(class) {
                self.blocked_count += 1;
                return true;
            }
        }

        false
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn get_stats(&self) -> String {
        format!("Blocked: {} ads/trackers", self.blocked_count)
    }
}
