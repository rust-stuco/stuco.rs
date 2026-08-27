use std::sync::LazyLock;

static RULES: LazyLock<Vec<ShortLink>> = LazyLock::new(|| parse(REDIRECTS));

const REDIRECTS: &str = include_str!("../_redirects");

struct ShortLink {
    path: &'static str,
    destination: &'static str,
}

pub(crate) fn destination(path: &str) -> &'static str {
    RULES
        .iter()
        .find(|link| link.path == path)
        .unwrap_or_else(|| panic!("`_redirects` should declare a rule for {path}"))
        .destination
}

fn parse(redirects: &'static str) -> Vec<ShortLink> {
    redirects
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(|line| {
            let mut fields = line.split_whitespace();
            let mut next = |name| {
                fields
                    .next()
                    .unwrap_or_else(|| panic!("`_redirects` rule `{line}` should have a {name}"))
            };

            ShortLink {
                path: next("path"),
                destination: next("destination"),
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_are_well_formed() {
        assert!(!RULES.is_empty(), "`_redirects` should declare rules");

        for link in RULES.iter() {
            assert!(
                link.path.starts_with('/'),
                "{} should be a rooted path",
                link.path
            );
            assert!(
                link.destination.starts_with("https://"),
                "{} should redirect to an https destination, got {}",
                link.path,
                link.destination
            );
        }
    }

    #[test]
    fn paths_are_unique() {
        let mut paths = RULES.iter().map(|link| link.path).collect::<Vec<_>>();
        paths.sort_unstable();
        let unique = paths.len();
        paths.dedup();

        assert_eq!(paths.len(), unique, "`_redirects` should not repeat a path");
    }

    #[test]
    fn short_links_cover_the_navigation() {
        for (path, _) in crate::navbar::SHORT_LINKS {
            destination(path);
        }
    }
}
