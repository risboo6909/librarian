use regex::Regex;
use std::collections::HashMap;

//parses go.mod file fetching github/gitlab unique link
pub(crate) fn parse(input: &str) -> Vec<String> {
    let mut link_map: HashMap<&str, usize> = HashMap::new();
    let re = Regex::new(r"^(github|gitlab)\.(com|ru)\S*").unwrap();
    input
        .split(char::is_whitespace)
        .filter(|x| re.is_match(x))
        .map(|x| link_map.insert(x, 0))
        .for_each(drop);
    let links: Vec<&str> = link_map.keys().map(|x| x.to_owned()).collect();
    links.into_iter().map(|x| x.to_owned()).collect()
}

#[cfg(test)]
mod tests {

    use crate::gomod_parser::parse;
    use std::fs;

    #[test]
    fn test_parse() {
        let contents = include_str!("../gomod_parser/go.mod");
        let mut parsed_file = parse(contents);

        let mut compare = vec![
            "github.com/360EntSecGroup-Skylar/excelize".to_owned(),
            "github.com/go-chi/chi".to_owned(), //v4.0.2+incompatible
            "github.com/go-chi/cors".to_owned(), //v1.0.0
            "github.com/go-openapi/spec".to_owned(), //v0.19.3
            "github.com/golang/protobuf".to_owned(), //v1.4.2
            "github.com/grpc-ecosystem/grpc-gateway".to_owned(), //v1.14.2
            "github.com/lib/pq".to_owned(),     //v1.3.0  indirect
            "github.com/pkg/errors".to_owned(), //v0.8.1
            "github.com/spf13/pflag".to_owned(), //v1.0.5
            "github.com/spf13/viper".to_owned(), //v1.6.1
            "github.com/stretchr/testify".to_owned(), //v1.4.0
            "github.com/utrack/clay/v2".to_owned(), //v2.4.7
            "gitlab.ru/internal-projects/staff-api".to_owned(), //v0.0.0-20200619100014-b44a8677723f
            "gitlab.ru/platform/database-go".to_owned(), //v0.15.2
            "gitlab.ru/platform/errors".to_owned(), //v1.3.6
            "gitlab.ru/platform/realtime-config-go".to_owned(), //v1.8.7
            "gitlab.ru/platform/scratch".to_owned(), //v1.6.8
            "gitlab.ru/platform/tracer-go".to_owned(), //v1.16.0
        ];
        assert_eq!(parsed_file.sort(), compare.sort());
    }
}
